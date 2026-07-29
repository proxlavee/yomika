import { describe, expect, it } from 'vitest'

import { isNewerVersion } from '@/components/UpdateChecker'

describe('GitHub release version comparison', () => {
  it('compares numeric semantic-version components', () => {
    expect(isNewerVersion('0.2.0', '0.1.9')).toBe(true)
    expect(isNewerVersion('v1.10.0', '1.9.9')).toBe(true)
    expect(isNewerVersion('2.0.0', '2.0.0')).toBe(false)
    expect(isNewerVersion('1.9.9', '2.0.0')).toBe(false)
  })

  it('does not offer an update for an invalid tag', () => {
    expect(isNewerVersion('nightly', '0.2.0')).toBe(false)
    expect(isNewerVersion('0.2.0', 'unknown')).toBe(false)
  })
})
