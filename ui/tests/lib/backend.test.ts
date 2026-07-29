import { afterEach, describe, expect, it, vi } from 'vitest'

const { setEnabled, setFocus } = vi.hoisted(() => ({
  setEnabled: vi.fn(async () => {}),
  setFocus: vi.fn(async () => {}),
}))

vi.mock('@tauri-apps/api/window', () => ({
  getCurrentWindow: () => ({ setEnabled, setFocus }),
}))

import { restoreAppWindowInteraction } from '@/lib/backend'

describe('restoreAppWindowInteraction', () => {
  afterEach(() => {
    Reflect.deleteProperty(window, '__TAURI_INTERNALS__')
    setEnabled.mockReset()
    setEnabled.mockResolvedValue(undefined)
    setFocus.mockReset()
    setFocus.mockResolvedValue(undefined)
  })

  it('re-enables and focuses the Tauri window', async () => {
    Object.assign(window, { __TAURI_INTERNALS__: {} })

    await restoreAppWindowInteraction()

    expect(setEnabled).toHaveBeenCalledWith(true)
    expect(setFocus).toHaveBeenCalledOnce()
  })

  it('still tries to focus when enabling fails', async () => {
    Object.assign(window, { __TAURI_INTERNALS__: {} })
    setEnabled.mockRejectedValueOnce(new Error('permission denied'))

    await restoreAppWindowInteraction()

    expect(setFocus).toHaveBeenCalledOnce()
  })

  it('does nothing in a normal browser', async () => {
    await restoreAppWindowInteraction()

    expect(setEnabled).not.toHaveBeenCalled()
    expect(setFocus).not.toHaveBeenCalled()
  })
})
