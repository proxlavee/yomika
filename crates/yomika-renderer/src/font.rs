use std::{
    collections::{HashMap, VecDeque},
    fmt,
    sync::{Arc, Mutex},
};

use anyhow::Context;
pub use fontdb::FaceInfo;
use fontdb::{Database, ID};
use harfrust::{Direction, Feature, Language, Script, ShapePlan, ShapePlanKey, Shaper, ShaperData};
use once_cell::sync::OnceCell;

const SHAPE_PLAN_CACHE_CAPACITY: usize = 16;

#[derive(Default)]
struct ShapingCache {
    data: OnceCell<ShaperData>,
    plans: Mutex<VecDeque<Arc<ShapePlan>>>,
}

impl fmt::Debug for ShapingCache {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ShapingCache")
            .finish_non_exhaustive()
    }
}

/// A loaded font ready for shaping and rendering.
#[derive(Clone, Debug)]
pub struct Font {
    data: Arc<[u8]>,
    face: FaceInfo,
    fontdue: Arc<OnceCell<Arc<fontdue::Font>>>,
    shaping: Arc<ShapingCache>,
    pub weight: u16,
    pub style: String,
}

impl Font {
    /// Creates a skrifa FontRef for metric queries.
    pub fn skrifa(&self) -> anyhow::Result<skrifa::FontRef<'_>> {
        skrifa::FontRef::from_index(self.data.as_ref(), self.face.index)
            .context("failed to create skrifa FontRef")
    }

    /// Creates a harfrust FontRef for text shaping.
    pub fn harfrust(&self) -> anyhow::Result<harfrust::FontRef<'_>> {
        harfrust::FontRef::from_index(self.data.as_ref(), self.face.index)
            .context("failed to create harfrust FontRef")
    }

    pub(crate) fn shaper_data(&self, font_ref: &harfrust::FontRef<'_>) -> &ShaperData {
        self.shaping.data.get_or_init(|| ShaperData::new(font_ref))
    }

    pub(crate) fn shape_plan(
        &self,
        shaper: &Shaper<'_>,
        direction: Direction,
        script: Option<Script>,
        language: Option<&Language>,
        features: &[Feature],
    ) -> Arc<ShapePlan> {
        let key = ShapePlanKey::new(script, direction)
            .language(language)
            .features(features);
        let mut plans = self
            .shaping
            .plans
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if let Some(plan) = plans.iter().find(|plan| key.matches(plan)) {
            return Arc::clone(plan);
        }

        let plan = Arc::new(ShapePlan::new(
            shaper, direction, script, language, features,
        ));
        if plans.len() == SHAPE_PLAN_CACHE_CAPACITY {
            plans.pop_front();
        }
        plans.push_back(Arc::clone(&plan));
        plan
    }

    #[cfg(test)]
    pub(crate) fn shape_plan_cache_len(&self) -> usize {
        self.shaping
            .plans
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .len()
    }

    pub fn fontdue(&self) -> anyhow::Result<Arc<fontdue::Font>> {
        let font = self.fontdue.get_or_try_init(|| {
            let settings = fontdue::FontSettings {
                collection_index: self.face.index,
                ..Default::default()
            };
            let font = fontdue::Font::from_bytes(self.data.as_ref(), settings)
                .map_err(|err| anyhow::anyhow!(err))
                .context("failed to create fontdue Font")?;
            Ok::<_, anyhow::Error>(Arc::new(font))
        })?;
        Ok(Arc::clone(font))
    }

    /// Returns true if the font contains a glyph for the given character.
    pub fn has_glyph(&self, character: char) -> bool {
        self.fontdue()
            .map(|font| font.has_glyph(character))
            .unwrap_or(false)
    }

    pub fn post_script_name(&self) -> &str {
        &self.face.post_script_name
    }

    pub fn weight(&self) -> u16 {
        self.weight
    }

    pub fn style(&self) -> &str {
        &self.style
    }

    pub fn face_info(&self) -> &FaceInfo {
        &self.face
    }
}

pub(crate) fn font_key(font: &Font) -> usize {
    font as *const Font as usize
}

/// A collection of font sources for font discovery and loading.
pub struct FontBook {
    database: Database,
    cache: HashMap<ID, Font>,
    /// Maps data hash to font ID to avoid duplicate loading.
    data_cache: HashMap<[u8; 32], ID>,
}

impl FontBook {
    /// Creates a FontBook with system fonts.
    pub fn new() -> Self {
        let mut database = Database::new();
        database.load_system_fonts();

        Self {
            database,
            cache: HashMap::new(),
            data_cache: HashMap::new(),
        }
    }

    /// Returns all available font faces.
    pub fn all_families(&self) -> Vec<FaceInfo> {
        self.database.faces().cloned().collect()
    }

    /// Loads a font by PostScript name.
    pub fn query(&mut self, post_script_name: &str) -> anyhow::Result<Font> {
        let Some(id) = self
            .database
            .faces()
            .find_map(|face| (face.post_script_name == post_script_name).then_some(face.id))
        else {
            return Err(anyhow::anyhow!(
                "no font found for PostScript name: {post_script_name}"
            ));
        };
        self.load_font(id)
    }

    /// Loads a font from raw bytes (e.g., downloaded from Google Fonts).
    pub fn load_from_bytes(&mut self, data: Vec<u8>) -> anyhow::Result<Font> {
        let hash: [u8; 32] = blake3::hash(&data).into();

        if let Some(&id) = self.data_cache.get(&hash) {
            return self.load_font(id);
        }

        let data: Arc<dyn AsRef<[u8]> + Send + Sync> = Arc::new(data);
        let source = fontdb::Source::Binary(data);
        let ids = self.database.load_font_source(source);
        let id = ids
            .into_iter()
            .next()
            .context("font data contained no valid faces")?;

        self.data_cache.insert(hash, id);
        self.load_font(id)
    }

    pub fn load_font(&mut self, id: ID) -> anyhow::Result<Font> {
        if let Some(font) = self.cache.get(&id) {
            return Ok(font.clone());
        }

        let face = self
            .database
            .face(id)
            .cloned()
            .with_context(|| format!("missing font face for id {:?}", id))?;
        let data = self
            .database
            .with_face_data(id, |data, _| Arc::<[u8]>::from(data.to_vec()))
            .with_context(|| format!("failed to load font data for {:?}", id))?;

        // Determine weight and style from face info
        let fontdb::Weight(weight) = face.weight;
        let style = match face.style {
            fontdb::Style::Normal => "normal".to_string(),
            fontdb::Style::Italic => "italic".to_string(),
            fontdb::Style::Oblique => "oblique".to_string(),
        };

        let font = Font {
            data,
            face,
            fontdue: Arc::new(OnceCell::new()),
            shaping: Arc::new(ShapingCache::default()),
            weight,
            style,
        };
        self.cache.insert(id, font.clone());
        Ok(font)
    }
}

impl Default for FontBook {
    fn default() -> Self {
        Self::new()
    }
}
