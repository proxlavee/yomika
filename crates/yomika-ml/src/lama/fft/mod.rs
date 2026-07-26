#[cfg(feature = "cuda")]
use candle_core::cuda_backend::CudaStorage;
#[cfg(feature = "metal")]
use candle_core::metal_backend::MetalStorage;
use candle_core::{CpuStorage, CustomOp1, Layout, Result, Shape, Tensor, bail};
#[cfg(any(feature = "cuda", feature = "metal", test))]
use std::collections::HashMap;
#[cfg(any(feature = "cuda", feature = "metal", test))]
use std::hash::Hash;
use tracing::instrument;

mod cpu;
#[cfg(feature = "cuda")]
mod cuda;
#[cfg(feature = "metal")]
mod metal;

/// Maximum number of exact FFT shapes retained by each backend cache.
///
/// Crop-based inpainting produces many distinct shapes. Each CUDA plan owns a
/// sizable work area, while each Metal plan retains an MPS graph, so an
/// unbounded shape cache eventually exhausts GPU or unified memory. Six
/// forward/inverse shape pairs preserve useful short-term reuse without
/// allowing project size to determine memory use.
#[cfg(any(feature = "cuda", feature = "metal"))]
const PLAN_CACHE_CAPACITY: usize = 12;

#[cfg(any(feature = "cuda", feature = "metal", test))]
struct CacheEntry<V> {
    value: V,
    last_used: u64,
}

/// Small LRU used by the platform FFT implementations.
///
/// Insertion and pruning are deliberately separate. CUDA launches are
/// asynchronous, so callers first synchronize the model device at a safe
/// inference boundary and only then prune unused plans.
#[cfg(any(feature = "cuda", feature = "metal", test))]
struct ShapeCache<K, V> {
    entries: HashMap<K, CacheEntry<V>>,
    clock: u64,
    capacity: usize,
}

#[cfg(any(feature = "cuda", feature = "metal", test))]
impl<K, V> ShapeCache<K, V>
where
    K: Copy + Eq + Hash,
    V: Clone,
{
    fn new(capacity: usize) -> Self {
        Self {
            entries: HashMap::new(),
            clock: 0,
            capacity,
        }
    }

    fn get(&mut self, key: &K) -> Option<V> {
        let tick = self.next_tick();
        let entry = self.entries.get_mut(key)?;
        entry.last_used = tick;
        Some(entry.value.clone())
    }

    fn insert(&mut self, key: K, value: V) {
        let tick = self.next_tick();
        self.entries.insert(
            key,
            CacheEntry {
                value,
                last_used: tick,
            },
        );
    }

    /// Remove least-recently-used entries accepted by `can_evict` until the
    /// cache reaches its capacity. Values are returned so resource-heavy
    /// destructors run after the caller releases any cache lock or borrow.
    fn prune(&mut self, can_evict: impl Fn(&V) -> bool) -> Vec<V> {
        let mut removed = Vec::new();
        while self.entries.len() > self.capacity {
            let victim = self
                .entries
                .iter()
                .filter(|(_, entry)| can_evict(&entry.value))
                .min_by_key(|(_, entry)| entry.last_used)
                .map(|(key, _)| *key);
            let Some(victim) = victim else {
                break;
            };
            if let Some(entry) = self.entries.remove(&victim) {
                removed.push(entry.value);
            }
        }
        removed
    }

    fn next_tick(&mut self) -> u64 {
        self.clock = self.clock.saturating_add(1);
        self.clock
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.entries.len()
    }

    #[cfg(test)]
    fn contains_key(&self, key: &K) -> bool {
        self.entries.contains_key(key)
    }
}

/// Prune backend FFT plans after a complete LaMa forward has synchronized.
pub(super) fn prune_plan_caches() {
    #[cfg(feature = "cuda")]
    cuda::prune_plan_cache();
    #[cfg(feature = "metal")]
    metal::prune_plan_cache();
}

#[derive(Clone, Copy)]
struct Rfft2;

#[derive(Clone, Copy)]
struct Irfft2 {
    width: usize,
}

impl CustomOp1 for Rfft2 {
    fn name(&self) -> &'static str {
        "rfft2"
    }

    #[instrument(level = "debug", skip_all)]
    fn cpu_fwd(&self, storage: &CpuStorage, layout: &Layout) -> Result<(CpuStorage, Shape)> {
        cpu::rfft2(storage, layout)
    }

    #[cfg(feature = "cuda")]
    #[instrument(level = "debug", skip_all)]
    fn cuda_fwd(&self, storage: &CudaStorage, layout: &Layout) -> Result<(CudaStorage, Shape)> {
        cuda::rfft2(storage, layout)
    }

    #[cfg(feature = "metal")]
    #[instrument(level = "debug", skip_all)]
    fn metal_fwd(&self, storage: &MetalStorage, layout: &Layout) -> Result<(MetalStorage, Shape)> {
        metal::rfft2(storage, layout)
    }
}

impl CustomOp1 for Irfft2 {
    fn name(&self) -> &'static str {
        "irfft2"
    }

    #[instrument(level = "debug", skip_all)]
    fn cpu_fwd(&self, storage: &CpuStorage, layout: &Layout) -> Result<(CpuStorage, Shape)> {
        cpu::irfft2(storage, layout, self.width)
    }

    #[cfg(feature = "cuda")]
    #[instrument(level = "debug", skip_all)]
    fn cuda_fwd(&self, storage: &CudaStorage, layout: &Layout) -> Result<(CudaStorage, Shape)> {
        cuda::irfft2(storage, layout, self.width)
    }

    #[cfg(feature = "metal")]
    #[instrument(level = "debug", skip_all)]
    fn metal_fwd(&self, storage: &MetalStorage, layout: &Layout) -> Result<(MetalStorage, Shape)> {
        metal::irfft2(storage, layout, self.width)
    }
}

pub fn rfft2(xs: &Tensor) -> candle_core::Result<Tensor> {
    let xs = xs.contiguous()?;
    let op = Rfft2;
    xs.apply_op1_no_bwd(&op)
}

pub fn irfft2(spectrum: &Tensor, width: usize) -> candle_core::Result<Tensor> {
    let spectrum = spectrum.contiguous()?;
    let dims = spectrum.dims();
    if dims.len() != 5 || *dims.last().unwrap() != 2 {
        bail!("irfft2 expects spectrum shaped [batch, channels, height, width/2+1, 2]")
    }
    let (_b, _c, h, w_half) = (dims[0], dims[1], dims[2], dims[3]);
    let inferred_width = (w_half - 1) * 2;
    if width != inferred_width && width != inferred_width + 1 {
        bail!(
            "irfft2 width mismatch: spectrum implies {} or {}, got {width}",
            inferred_width,
            inferred_width + 1
        );
    }
    let op = Irfft2 { width };
    let time = spectrum.apply_op1_no_bwd(&op)?;
    let scale = 1.0f32 / ((h * width) as f32);
    time.affine(scale as f64, 0.0)?.contiguous()
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::{Device, Tensor};

    #[test]
    fn cpu_rfft2_roundtrip_matches_input() -> Result<()> {
        let device = Device::Cpu;
        let data: Vec<f32> = (0..(2 * 4 * 6)).map(|i| (i as f32).sin() * 0.25).collect();
        let input = Tensor::from_vec(data.clone(), (1, 2, 4, 6), &device)?;
        let reconstructed = irfft2(&rfft2(&input)?, 6)?;
        let diffs: Vec<f32> = (reconstructed - &input)?
            .flatten_all()?
            .to_vec1()?
            .into_iter()
            .map(|v: f32| v.abs())
            .collect();
        let max_err = diffs
            .into_iter()
            .fold(0f32, |acc, v| if v > acc { v } else { acc });
        assert!(max_err < 1e-3, "max reconstruction error: {max_err}");
        Ok(())
    }

    #[test]
    fn shape_cache_prunes_least_recently_used_entries() {
        let mut cache = ShapeCache::new(2);
        cache.insert(1, "one");
        cache.insert(2, "two");
        assert_eq!(cache.get(&1), Some("one"));
        cache.insert(3, "three");

        assert_eq!(cache.prune(|_| true), vec!["two"]);
        assert!(cache.contains_key(&1));
        assert!(cache.contains_key(&3));
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn shape_cache_temporarily_overshoots_when_entries_are_busy() {
        let mut cache = ShapeCache::new(1);
        cache.insert(1, false);
        cache.insert(2, true);

        assert_eq!(cache.prune(|busy| !busy), vec![false]);
        assert_eq!(cache.len(), 1);

        cache.insert(3, true);
        assert!(cache.prune(|busy| !busy).is_empty());
        assert_eq!(cache.len(), 2);
    }
}
