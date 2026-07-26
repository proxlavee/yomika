import { describe, expect, it, vi } from 'vitest'

import { prepareDisplayBlob } from '@/lib/io/blobConvert'

describe('prepareDisplayBlob', () => {
  it('keeps encoded images as native blobs without reading the full payload', async () => {
    const blob = new Blob([new Uint8Array(1024 * 1024)], { type: 'image/png' })
    const fullRead = vi.spyOn(blob, 'arrayBuffer')

    await expect(prepareDisplayBlob(blob)).resolves.toBe(blob)
    expect(fullRead).not.toHaveBeenCalled()
  })
})
