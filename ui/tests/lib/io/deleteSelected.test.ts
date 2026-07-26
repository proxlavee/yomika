import { http, HttpResponse } from 'msw'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'

import { getGetSceneJsonQueryKey } from '@/lib/api/default/default'
import type { Node, Page, SceneSnapshot } from '@/lib/api/schemas'
import { cancelPendingAutoRender, deleteSelectedTextNodesOnCurrentPage } from '@/lib/io/scene'
import { queryClient } from '@/lib/queryClient'
import { useSelectionStore } from '@/lib/stores/selectionStore'

import { server } from '../../msw/server'

function textNode(id: string): Node {
  return {
    id,
    transform: { x: 0, y: 0, width: 10, height: 10, rotationDeg: 0 },
    visible: true,
    kind: { text: { raw: id } },
  } as unknown as Node
}

function seedScene(): SceneSnapshot {
  const page = {
    id: 'p-1',
    name: 'Page',
    width: 10,
    height: 10,
    nodes: {
      first: textNode('first'),
      second: textNode('second'),
      third: textNode('third'),
    },
  } as unknown as Page
  return {
    epoch: 1,
    scene: { pages: { 'p-1': page }, project: { name: 'Project' } as never } as never,
  }
}

describe('deleteSelectedTextNodesOnCurrentPage', () => {
  beforeEach(() => {
    queryClient.clear()
    useSelectionStore.getState().setPage('p-1')
    queryClient.setQueryData(getGetSceneJsonQueryKey(), seedScene())
  })

  afterEach(() => cancelPendingAutoRender())

  it('submits one ordered batch for existing selected nodes and clears selection', async () => {
    const received = vi.fn()
    server.use(
      http.post('/api/v1/history/apply', async ({ request }) => {
        received(await request.json())
        return HttpResponse.json({ epoch: 2 })
      }),
    )
    useSelectionStore.getState().selectMany(['second', 'missing', 'first'])

    await deleteSelectedTextNodesOnCurrentPage()

    expect(received).toHaveBeenCalledTimes(1)
    expect(received.mock.calls[0][0]).toMatchObject({
      batch: {
        label: 'removeNodes',
        ops: [
          { removeNode: { id: 'second', prev_index: 1 } },
          { removeNode: { id: 'first', prev_index: 0 } },
        ],
      },
    })
    expect(useSelectionStore.getState().nodeIds.size).toBe(0)
  })
})
