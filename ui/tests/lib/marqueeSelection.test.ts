import { describe, expect, it } from 'vitest'

import { marqueeIntersects, normalizeMarqueeRect } from '@/hooks/useMarqueeSelection'

describe('marquee selection geometry', () => {
  it('normalizes drags in every direction', () => {
    expect(normalizeMarqueeRect(80, 100, 20, 40)).toEqual({
      left: 20,
      top: 40,
      width: 60,
      height: 60,
    })
  })

  it('selects intersecting items and excludes outside items', () => {
    const selection = { left: 20, top: 20, width: 60, height: 60 }
    expect(marqueeIntersects(selection, { left: 70, top: 70, width: 20, height: 20 })).toBe(true)
    expect(marqueeIntersects(selection, { left: 90, top: 90, width: 20, height: 20 })).toBe(false)
  })
})
