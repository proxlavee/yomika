'use client'

import { keepPreviousData, useQuery } from '@tanstack/react-query'

import { getBlob } from '@/lib/api/default/default'
import { prepareDisplayBlob } from '@/lib/io/blobConvert'

const blobQueryOptions = (hash: string) => ({
  queryKey: ['blob', hash] as const,
  queryFn: async () => (await getBlob(hash)) as Blob,
  staleTime: Infinity,
  gcTime: 30 * 1000,
  structuralSharing: false as const,
})

/** Fetch a native browser blob by hash without materializing a typed array. */
export function useBlobData(hash: string | undefined): Blob | undefined {
  const { data } = useQuery({
    ...blobQueryOptions(hash ?? ''),
    enabled: !!hash,
    placeholderData: keepPreviousData,
  })
  return hash ? data : undefined
}

const blobImageQueryOptions = (hash: string) => ({
  queryKey: ['blobImage', hash] as const,
  queryFn: async () => {
    const response = await getBlob(hash)
    return prepareDisplayBlob(response as Blob)
  },
  staleTime: Infinity,
  // Display blobs can be tens of megabytes. Keep a short back-navigation
  // window without retaining every page visited during a long editing run.
  gcTime: 30 * 1000,
  structuralSharing: false as const,
})

/**
 * Fetch a display image as a native Blob. The rendering component owns the
 * object URL so it can revoke it when the image leaves the DOM.
 */
export function useBlobImage(hash: string | undefined) {
  return useQuery({
    ...blobImageQueryOptions(hash ?? ''),
    enabled: !!hash,
    placeholderData: keepPreviousData,
  })
}
