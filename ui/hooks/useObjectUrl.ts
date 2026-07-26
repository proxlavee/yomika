'use client'

import { useEffect, useState } from 'react'

import { revokeObjectUrlLater } from '@/lib/io/blobConvert'

/** Create a component-scoped object URL and revoke it after use. */
export function useObjectUrl(blob: Blob | undefined): string | undefined {
  const [url, setUrl] = useState<string>()

  useEffect(() => {
    if (!blob) {
      setUrl(undefined)
      return
    }

    const next = URL.createObjectURL(blob)
    setUrl(next)
    return () => revokeObjectUrlLater(next)
  }, [blob])

  return url
}
