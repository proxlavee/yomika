import { describe, expect, it } from 'vitest'

import {
  resolvePinchMemoScaleRatio,
  resolvePinchNextScaleRatio,
  resolveWheelNextScale,
} from '@/components/canvas/zoomGestures'

describe('canvas zoom gestures', () => {
  it('zooms in and out with a plain wheel delta', () => {
    expect(resolveWheelNextScale(50, -100)).toBe(55)
    expect(resolveWheelNextScale(50, 100)).toBe(45)
  })

  it('clamps wheel zoom to the supported range', () => {
    expect(resolveWheelNextScale(10, 100)).toBe(10)
    expect(resolveWheelNextScale(100, -100)).toBe(100)
  })

  it('keeps pinch scaling bounded and ignores invalid movement', () => {
    expect(resolvePinchMemoScaleRatio(undefined, 0.5)).toBe(0.5)
    expect(resolvePinchNextScaleRatio(0.5, 1.5)).toBe(0.75)
    expect(resolvePinchNextScaleRatio(0.5, Number.NaN)).toBe(0.5)
  })
})
