import { http, HttpResponse } from 'msw'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'

import * as defaultApi from '@/lib/api/default/default'
import { cancelPendingAutoRender, queueAutoRender, redoOp, undoOp } from '@/lib/io/scene'
import { usePreferencesStore } from '@/lib/stores/preferencesStore'

import { server } from '../../msw/server'

describe('queueAutoRender', () => {
  beforeEach(() => {
    vi.useFakeTimers()
  })

  afterEach(() => {
    cancelPendingAutoRender()
    vi.useRealTimers()
    vi.restoreAllMocks()
  })

  it('coalesces rapid edits into a single pipeline POST and forwards default font', async () => {
    vi.spyOn(usePreferencesStore, 'getState').mockReturnValue({
      defaultFont: 'Comic Sans MS',
    } as ReturnType<typeof usePreferencesStore.getState>)
    const pipelineHits: Array<{
      steps: string[]
      pages: string[]
      defaultFont?: string | null
      autoRenderEpoch?: number | null
    }> = []
    server.use(
      http.get('/api/v1/config', () =>
        HttpResponse.json({ pipeline: { renderer: 'yomika-renderer' } }),
      ),
      http.post('/api/v1/pipelines', async ({ request }) => {
        const body = (await request.json()) as {
          steps: string[]
          pages: string[]
          defaultFont?: string | null
          autoRenderEpoch?: number | null
        }
        pipelineHits.push({
          steps: body.steps,
          pages: body.pages,
          defaultFont: body.defaultFont,
          autoRenderEpoch: body.autoRenderEpoch,
        })
        return HttpResponse.json({ operationId: `op-${pipelineHits.length}` })
      }),
    )

    queueAutoRender('p-1', 41)
    queueAutoRender('p-1', 42)
    queueAutoRender('p-1', 43)

    // Before the debounce window elapses, no POST.
    expect(pipelineHits).toHaveLength(0)

    // Debounce = 500ms. Advance just past it and let any pending microtasks run.
    await vi.advanceTimersByTimeAsync(550)

    expect(pipelineHits).toHaveLength(1)
    expect(pipelineHits[0].steps).toEqual(['yomika-renderer'])
    expect(pipelineHits[0].pages).toEqual(['p-1'])
    expect(pipelineHits[0].defaultFont).toBe('Comic Sans MS')
    expect(pipelineHits[0].autoRenderEpoch).toBe(43)
  })

  it('undoOp cancels a pending auto-render so no stale render op lands after the undo', async () => {
    let pipelinePosts = 0
    server.use(
      http.get('/api/v1/config', () =>
        HttpResponse.json({ pipeline: { renderer: 'yomika-renderer' } }),
      ),
      http.post('/api/v1/pipelines', () => {
        pipelinePosts += 1
        return HttpResponse.json({ operationId: 'op-1' })
      }),
    )

    queueAutoRender('p-1', 1)
    const undoPromise = undoOp()
    // Flush the undo request and the (cancelled) debounce window together.
    await vi.advanceTimersByTimeAsync(600)
    await undoPromise

    expect(pipelinePosts).toBe(0)
  })

  it('redoOp cancels a pending auto-render', async () => {
    let pipelinePosts = 0
    server.use(
      http.get('/api/v1/config', () =>
        HttpResponse.json({ pipeline: { renderer: 'yomika-renderer' } }),
      ),
      http.post('/api/v1/pipelines', () => {
        pipelinePosts += 1
        return HttpResponse.json({ operationId: 'op-1' })
      }),
    )

    queueAutoRender('p-1', 1)
    const redoPromise = redoOp()
    await vi.advanceTimersByTimeAsync(600)
    await redoPromise

    expect(pipelinePosts).toBe(0)
  })

  it('undoOp invalidates an auto-render whose config request is already in flight', async () => {
    let releaseConfig:
      | ((config: Awaited<ReturnType<typeof defaultApi.getConfig>>) => void)
      | undefined
    const deferredConfig = new Promise<Awaited<ReturnType<typeof defaultApi.getConfig>>>(
      (resolve) => {
        releaseConfig = resolve
      },
    )
    const getConfig = vi.spyOn(defaultApi, 'getConfig').mockReturnValue(deferredConfig)
    const startPipeline = vi.spyOn(defaultApi, 'startPipeline')

    queueAutoRender('p-1', 1)
    vi.advanceTimersByTime(550)
    expect(getConfig).toHaveBeenCalledOnce()

    const undoPromise = undoOp()
    releaseConfig?.({ pipeline: { renderer: 'yomika-renderer' } } as Awaited<
      ReturnType<typeof defaultApi.getConfig>
    >)
    await vi.runAllTimersAsync()
    await undoPromise

    expect(startPipeline).not.toHaveBeenCalled()
  })

  it('is a no-op when no renderer is configured', async () => {
    vi.spyOn(usePreferencesStore, 'getState').mockReturnValue({
      defaultFont: undefined,
    } as ReturnType<typeof usePreferencesStore.getState>)
    let pipelinePosts = 0
    server.use(
      http.get('/api/v1/config', () => HttpResponse.json({ pipeline: {} })),
      http.post('/api/v1/pipelines', () => {
        pipelinePosts += 1
        return HttpResponse.json({ operationId: 'op-1' })
      }),
    )

    queueAutoRender('p-1', 1)
    await vi.advanceTimersByTimeAsync(550)

    expect(pipelinePosts).toBe(0)
  })
})
