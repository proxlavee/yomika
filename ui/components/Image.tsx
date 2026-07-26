'use client'

import type { CSSProperties } from 'react'
import { useCallback, useEffect, useRef, useState } from 'react'

import { cancelObjectUrlRevoke, revokeObjectUrlLater } from '@/lib/io/blobConvert'

type ImageProps = {
  blob?: Blob
  visible?: boolean
  opacity?: number
  transition?: boolean
  dataKey?: string | number
} & Omit<React.ImgHTMLAttributes<HTMLImageElement>, 'src'>

const FADE_DURATION_MS = 180

// Cross-fade between successive image buffers to avoid UI flicker when
// swapping inpaint results.
export function Image({ transition = true, ...props }: ImageProps) {
  return transition ? <CrossfadeImage {...props} /> : <PlainImage {...props} />
}

type ImageVariantProps = Omit<ImageProps, 'transition'>

function PlainImage({
  blob,
  visible = true,
  opacity = 1,
  dataKey,
  style,
  alt = '',
  ...props
}: ImageVariantProps) {
  const dataDep = dataKey ?? blob

  // Simple path without transitions (used for static base image to avoid extra paints)
  const [plainSrc, setPlainSrc] = useState<string | null>(null)
  const plainSrcRef = useRef<string | null>(null)
  useEffect(() => {
    if (!dataDep || !blob) {
      revokeObjectUrlLater(plainSrcRef.current)
      plainSrcRef.current = null
      setPlainSrc(null)
      return
    }

    const prev = plainSrcRef.current
    const url = URL.createObjectURL(blob)
    cancelObjectUrlRevoke(url)
    plainSrcRef.current = url
    setPlainSrc(url)
    revokeObjectUrlLater(prev)
  }, [blob, dataDep])

  useEffect(
    () => () => {
      revokeObjectUrlLater(plainSrcRef.current)
    },
    [],
  )

  if (!visible || !plainSrc) return null
  return (
    <img
      {...props}
      alt={alt}
      src={plainSrc}
      draggable={false}
      style={{
        position: 'absolute',
        inset: 0,
        pointerEvents: 'none',
        userSelect: 'none',
        width: '100%',
        height: '100%',
        objectFit: 'contain',
        ...style,
        opacity,
      }}
    />
  )
}

function CrossfadeImage({
  blob,
  visible = true,
  opacity = 1,
  dataKey,
  style,
  alt = '',
  ...props
}: ImageVariantProps) {
  const dataDep = dataKey ?? blob

  const [currentSrc, setCurrentSrc] = useState<string | null>(null)
  const [nextSrc, setNextSrc] = useState<string | null>(null)
  const [crossfade, setCrossfade] = useState(false)

  const currentSrcRef = useRef<string | null>(null)
  const nextSrcRef = useRef<string | null>(null)

  const cleanupUrl = useCallback((url: string | null) => {
    revokeObjectUrlLater(url)
  }, [])

  useEffect(() => {
    currentSrcRef.current = currentSrc
  }, [currentSrc])

  useEffect(() => {
    nextSrcRef.current = nextSrc
  }, [nextSrc])

  useEffect(() => {
    return () => {
      cleanupUrl(currentSrcRef.current)
      cleanupUrl(nextSrcRef.current)
    }
  }, [cleanupUrl])

  const promoteNext = useCallback(() => {
    const incoming = nextSrcRef.current
    if (!incoming) return
    const outgoing = currentSrcRef.current

    currentSrcRef.current = incoming
    setCurrentSrc(incoming)
    setNextSrc(null)
    setCrossfade(false)

    if (outgoing && outgoing !== incoming) {
      cleanupUrl(outgoing)
    }
  }, [cleanupUrl])

  useEffect(() => {
    if (!dataDep || !blob) {
      cleanupUrl(currentSrcRef.current)
      cleanupUrl(nextSrcRef.current)
      currentSrcRef.current = null
      nextSrcRef.current = null
      setCurrentSrc(null)
      setNextSrc(null)
      setCrossfade(false)
      return
    }

    let cancelled = false

    const objectUrl = URL.createObjectURL(blob)
    cancelObjectUrlRevoke(objectUrl)

    const preload = new window.Image()
    preload.onload = () => {
      if (cancelled) {
        cleanupUrl(objectUrl)
        return
      }

      // First image, render immediately
      if (!currentSrcRef.current) {
        currentSrcRef.current = objectUrl
        setCurrentSrc(objectUrl)
        return
      }

      // Subsequent images: queue and cross-fade
      setNextSrc((prev) => {
        if (prev && prev !== currentSrcRef.current) {
          cleanupUrl(prev)
        }
        return objectUrl
      })

      setCrossfade(false)
      requestAnimationFrame(() => {
        requestAnimationFrame(() => setCrossfade(true))
      })
    }
    preload.onerror = () => {
      cleanupUrl(objectUrl)
    }

    preload.src = objectUrl

    return () => {
      cancelled = true
      preload.onload = null
      preload.onerror = null
      cleanupUrl(objectUrl)
    }
  }, [blob, dataDep, cleanupUrl])

  useEffect(() => {
    if (!nextSrc || !crossfade) return
    const timeout = window.setTimeout(
      promoteNext,
      FADE_DURATION_MS + 50, // safety fallback in case transitionend doesn't fire
    )
    return () => window.clearTimeout(timeout)
  }, [nextSrc, crossfade, promoteNext])

  if (!visible || (!currentSrc && !nextSrc)) return null

  const baseStyle: CSSProperties = {
    position: 'absolute',
    inset: 0,
    pointerEvents: 'none',
    userSelect: 'none',
    width: '100%',
    height: '100%',
    objectFit: 'contain',
    ...style,
  }

  return (
    <>
      {currentSrc && (
        <img
          {...props}
          alt={alt}
          src={currentSrc}
          draggable={false}
          style={{
            ...baseStyle,
            opacity: nextSrc ? (crossfade ? 0 : opacity) : opacity,
            transition: nextSrc && crossfade ? `opacity ${FADE_DURATION_MS}ms ease` : undefined,
          }}
        />
      )}
      {nextSrc && (
        <img
          {...props}
          alt={alt}
          src={nextSrc}
          draggable={false}
          onTransitionEnd={promoteNext}
          style={{
            ...baseStyle,
            opacity: crossfade ? opacity : 0,
            transition: `opacity ${FADE_DURATION_MS}ms ease`,
          }}
        />
      )}
    </>
  )
}
