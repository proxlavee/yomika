'use client'

const MIN_SCALE_RATIO = 0.1
const MAX_SCALE_RATIO = 1

function clampScaleRatio(scaleRatio: number) {
  if (!Number.isFinite(scaleRatio)) return MIN_SCALE_RATIO
  return Math.max(MIN_SCALE_RATIO, Math.min(MAX_SCALE_RATIO, scaleRatio))
}

export function resolvePinchMemoScaleRatio(memo: unknown, currentScaleRatio: number) {
  if (typeof memo === 'number' && Number.isFinite(memo)) {
    return clampScaleRatio(memo)
  }
  return clampScaleRatio(currentScaleRatio)
}

export function resolvePinchNextScaleRatio(memoScaleRatio: number, movementScale: number) {
  const baseScaleRatio = clampScaleRatio(memoScaleRatio)
  if (!Number.isFinite(movementScale) || movementScale <= 0) {
    return baseScaleRatio
  }
  return clampScaleRatio(baseScaleRatio * movementScale)
}

export function resolveWheelNextScale(currentScale: number, deltaY: number): number {
  const current = clampScaleRatio(currentScale / 100) * 100
  if (!Number.isFinite(deltaY) || deltaY === 0) return Math.round(current)
  const step = Math.max(1, Math.min(10, Math.abs(deltaY) * 0.05))
  return Math.round(clampScaleRatio((current - Math.sign(deltaY) * step) / 100) * 100)
}
