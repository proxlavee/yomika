'use client'

import { useCallback, useEffect, useRef, useState } from 'react'
import type React from 'react'

export type MarqueeRect = {
  left: number
  top: number
  width: number
  height: number
}

type DragState = {
  pointerId: number
  startX: number
  startY: number
  additive: boolean
  baseSelection: Set<string>
  moved: boolean
}

type Point = { clientX: number; clientY: number }

const DRAG_THRESHOLD = 3
const AUTO_SCROLL_EDGE = 36
const AUTO_SCROLL_MAX_STEP = 14
const BLOCKING_TARGETS =
  '[data-textblock-item], button, input, textarea, select, [contenteditable="true"]'

export function normalizeMarqueeRect(startX: number, startY: number, endX: number, endY: number) {
  return {
    left: Math.min(startX, endX),
    top: Math.min(startY, endY),
    width: Math.abs(endX - startX),
    height: Math.abs(endY - startY),
  }
}

export function marqueeIntersects(a: MarqueeRect, b: MarqueeRect): boolean {
  return (
    a.left <= b.left + b.width &&
    a.left + a.width >= b.left &&
    a.top <= b.top + b.height &&
    a.top + a.height >= b.top
  )
}

export function useMarqueeSelection({
  viewportRef,
  selectedIds,
  onSelectMany,
  onClear,
}: {
  viewportRef: React.RefObject<HTMLDivElement | null>
  selectedIds: Set<string>
  onSelectMany: (ids: string[]) => void
  onClear: () => void
}) {
  const [marqueeRect, setMarqueeRect] = useState<MarqueeRect | null>(null)
  const dragRef = useRef<DragState | null>(null)
  const pointerRef = useRef<Point | null>(null)
  const autoScrollFrameRef = useRef<number | null>(null)

  const updateSelection = useCallback(
    ({ clientX, clientY }: Point) => {
      const viewport = viewportRef.current
      const drag = dragRef.current
      if (!viewport || !drag) return
      const viewportRect = viewport.getBoundingClientRect()
      const localX = Math.max(0, Math.min(viewportRect.width, clientX - viewportRect.left))
      const localY = Math.max(0, Math.min(viewportRect.height, clientY - viewportRect.top))
      const endX = localX + viewport.scrollLeft
      const endY = localY + viewport.scrollTop
      const rect = normalizeMarqueeRect(drag.startX, drag.startY, endX, endY)
      setMarqueeRect(rect)

      if (rect.width < DRAG_THRESHOLD && rect.height < DRAG_THRESHOLD) return
      drag.moved = true
      const next = drag.additive ? new Set(drag.baseSelection) : new Set<string>()
      const items = viewport.querySelectorAll<HTMLElement>('[data-textblock-id]')
      for (const item of items) {
        const id = item.dataset.textblockId
        if (!id) continue
        const itemRect = item.getBoundingClientRect()
        const contentRect: MarqueeRect = {
          left: itemRect.left - viewportRect.left + viewport.scrollLeft,
          top: itemRect.top - viewportRect.top + viewport.scrollTop,
          width: itemRect.width,
          height: itemRect.height,
        }
        if (marqueeIntersects(rect, contentRect)) next.add(id)
      }
      onSelectMany([...next])
    },
    [onSelectMany, viewportRef],
  )

  const stopAutoScroll = useCallback(() => {
    if (autoScrollFrameRef.current !== null) cancelAnimationFrame(autoScrollFrameRef.current)
    autoScrollFrameRef.current = null
  }, [])

  const runAutoScroll = useCallback(() => {
    if (autoScrollFrameRef.current !== null) return
    const tick = () => {
      const viewport = viewportRef.current
      const pointer = pointerRef.current
      if (!viewport || !pointer || !dragRef.current) {
        autoScrollFrameRef.current = null
        return
      }
      const rect = viewport.getBoundingClientRect()
      const distanceFromTop = pointer.clientY - rect.top
      const distanceFromBottom = rect.bottom - pointer.clientY
      let delta = 0
      if (distanceFromTop < AUTO_SCROLL_EDGE) {
        delta = -Math.ceil(
          ((AUTO_SCROLL_EDGE - Math.max(0, distanceFromTop)) / AUTO_SCROLL_EDGE) *
            AUTO_SCROLL_MAX_STEP,
        )
      } else if (distanceFromBottom < AUTO_SCROLL_EDGE) {
        delta = Math.ceil(
          ((AUTO_SCROLL_EDGE - Math.max(0, distanceFromBottom)) / AUTO_SCROLL_EDGE) *
            AUTO_SCROLL_MAX_STEP,
        )
      }

      if (delta === 0) {
        autoScrollFrameRef.current = null
        return
      }
      const before = viewport.scrollTop
      viewport.scrollTop += delta
      if (viewport.scrollTop === before) {
        autoScrollFrameRef.current = null
        return
      }
      updateSelection(pointer)
      autoScrollFrameRef.current = requestAnimationFrame(tick)
    }
    autoScrollFrameRef.current = requestAnimationFrame(tick)
  }, [updateSelection, viewportRef])

  useEffect(() => stopAutoScroll, [stopAutoScroll])

  const onPointerDown = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (event.button !== 0) return
      const viewport = viewportRef.current
      const target = event.target
      if (!viewport || !(target instanceof Element) || !viewport.contains(target)) return
      if (target.closest(BLOCKING_TARGETS)) return
      const rect = viewport.getBoundingClientRect()
      const localX = Math.max(0, Math.min(rect.width, event.clientX - rect.left))
      const localY = Math.max(0, Math.min(rect.height, event.clientY - rect.top))
      const additive = event.shiftKey || event.ctrlKey || event.metaKey
      dragRef.current = {
        pointerId: event.pointerId,
        startX: localX + viewport.scrollLeft,
        startY: localY + viewport.scrollTop,
        additive,
        baseSelection: new Set(selectedIds),
        moved: false,
      }
      pointerRef.current = { clientX: event.clientX, clientY: event.clientY }
      setMarqueeRect({
        left: localX + viewport.scrollLeft,
        top: localY + viewport.scrollTop,
        width: 0,
        height: 0,
      })
      try {
        viewport.setPointerCapture?.(event.pointerId)
      } catch {
        // Pointer capture is best-effort in embedded webviews and test DOMs.
      }
      event.preventDefault()
    },
    [selectedIds, viewportRef],
  )

  const onPointerMove = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      const drag = dragRef.current
      if (!drag || drag.pointerId !== event.pointerId) return
      const point = { clientX: event.clientX, clientY: event.clientY }
      pointerRef.current = point
      updateSelection(point)
      runAutoScroll()
      event.preventDefault()
    },
    [runAutoScroll, updateSelection],
  )

  const finish = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      const viewport = viewportRef.current
      const drag = dragRef.current
      if (!drag || drag.pointerId !== event.pointerId) return
      updateSelection({ clientX: event.clientX, clientY: event.clientY })
      if (!drag.moved && !drag.additive) onClear()
      if (viewport?.hasPointerCapture?.(event.pointerId)) {
        viewport.releasePointerCapture(event.pointerId)
      }
      dragRef.current = null
      pointerRef.current = null
      setMarqueeRect(null)
      stopAutoScroll()
    },
    [onClear, stopAutoScroll, updateSelection, viewportRef],
  )

  return {
    marqueeRect,
    marqueeHandlers: {
      onPointerDown,
      onPointerMove,
      onPointerUp: finish,
      onPointerCancel: finish,
    },
  }
}
