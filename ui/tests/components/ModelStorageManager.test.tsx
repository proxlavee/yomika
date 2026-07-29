import { act, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { http, HttpResponse } from 'msw'
import { beforeEach, describe, expect, it } from 'vitest'

import { ModelStorageManager } from '@/components/ModelStorageManager'
import { getGetStorageQueryKey } from '@/lib/api/default/default'
import { useDownloadsStore } from '@/lib/stores/downloadsStore'
import { useJobsStore } from '@/lib/stores/jobsStore'

import { makeQueryClient, renderWithQuery } from '../helpers'
import { server } from '../msw/server'

const catalog = { localModels: [], providers: [] }

function storage(modelsBytes: number) {
  return {
    customModelsPath: false,
    dataPath: 'C:\\Yomika',
    downloadedLocalModels: 0,
    modelsBytes,
    modelsPath: 'C:\\Yomika\\models',
    temporaryBytes: 0,
  }
}

describe('ModelStorageManager', () => {
  beforeEach(() => {
    useDownloadsStore.getState().clear()
    useJobsStore.getState().clear()
  })

  it('allows deleting vision or OCR models when no translation model is listed', async () => {
    server.use(
      http.get('/api/v1/storage', () => HttpResponse.json(storage(1024))),
      http.get('/api/v1/llm/catalog', () => HttpResponse.json(catalog)),
    )

    renderWithQuery(<ModelStorageManager />)

    const deleteAll = await screen.findByRole('button', { name: 'settings.deleteAllModels' })
    await waitFor(() => expect(deleteAll).toBeEnabled())
    expect(screen.getByText('settings.noDownloadedModels')).toBeInTheDocument()
  })

  it('keeps bulk deletion disabled when model storage is empty', async () => {
    server.use(
      http.get('/api/v1/storage', () => HttpResponse.json(storage(0))),
      http.get('/api/v1/llm/catalog', () => HttpResponse.json(catalog)),
    )

    renderWithQuery(<ModelStorageManager />)

    expect(await screen.findByRole('button', { name: 'settings.deleteAllModels' })).toBeDisabled()
  })

  it('shows a retry action when storage details fail to load', async () => {
    let attempts = 0
    server.use(
      http.get('/api/v1/storage', () => {
        attempts += 1
        return attempts <= 2
          ? new HttpResponse(null, { status: 500 })
          : HttpResponse.json(storage(0))
      }),
      http.get('/api/v1/llm/catalog', () => HttpResponse.json(catalog)),
    )

    renderWithQuery(<ModelStorageManager />, { client: makeQueryClient() })

    expect(await screen.findByRole('alert', undefined, { timeout: 3000 })).toHaveTextContent(
      'settings.modelStorageLoadFailed',
    )
    await userEvent.click(screen.getByRole('button', { name: 'bootstrap.retryNow' }))
    await waitFor(() => expect(screen.queryByRole('alert')).not.toBeInTheDocument())
    expect(attempts).toBe(3)
  })

  it('disables storage mutations while a download is active', async () => {
    server.use(
      http.get('/api/v1/storage', () => HttpResponse.json(storage(1024))),
      http.get('/api/v1/llm/catalog', () => HttpResponse.json(catalog)),
    )
    useDownloadsStore.getState().progress({
      id: 'llm:test',
      filename: 'test.gguf',
      downloaded: 512,
      total: 1024,
      status: { status: 'downloading' },
    })

    renderWithQuery(<ModelStorageManager />)

    const path = await screen.findByPlaceholderText('settings.modelsPathPlaceholder')
    await userEvent.clear(path)
    await userEvent.type(path, 'D:\\Models')

    expect(screen.getByRole('button', { name: 'settings.modelsPathUse' })).toBeDisabled()
    expect(screen.getByRole('button', { name: 'settings.modelsPathMove' })).toBeDisabled()
    expect(screen.getByRole('button', { name: 'settings.deleteAllModels' })).toBeDisabled()
  })

  it('keeps the saved model path visible while a relaunch is pending', async () => {
    server.use(
      http.get('/api/v1/storage', () => HttpResponse.json(storage(0))),
      http.get('/api/v1/llm/catalog', () => HttpResponse.json(catalog)),
      http.put('/api/v1/storage/models/location', async ({ request }) => {
        const body = (await request.json()) as { path: string }
        return HttpResponse.json({
          copiedBytes: 0,
          modelsPath: body.path,
          restartRequired: true,
          sourceRemoved: true,
        })
      }),
    )

    const { client } = renderWithQuery(<ModelStorageManager />)

    const path = await screen.findByPlaceholderText('settings.modelsPathPlaceholder')
    await waitFor(() => expect(path).toHaveValue('C:\\Yomika\\models'))
    await userEvent.clear(path)
    await userEvent.type(path, 'D:\\Models')
    await userEvent.click(screen.getByRole('button', { name: 'settings.modelsPathUse' }))
    await userEvent.click(screen.getByRole('button', { name: 'settings.restartApply' }))

    await waitFor(() => expect(path).toHaveValue('D:\\Models'))
    expect(screen.getByText('settings.restartManually')).toBeInTheDocument()
    act(() => {
      client.setQueryData(getGetStorageQueryKey(), {
        ...storage(0),
        modelsPath: 'E:\\Stale-runtime-path',
      })
    })
    await waitFor(() => expect(path).toHaveValue('D:\\Models'))
  })
})
