'use client'

import { useQueryClient } from '@tanstack/react-query'
import { FolderOpenIcon, LoaderCircleIcon, RefreshCwIcon, Trash2Icon } from 'lucide-react'
import { useEffect, useMemo, useState } from 'react'
import { useTranslation } from 'react-i18next'

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import {
  clearModels,
  clearTemporaryCache,
  deleteLocalModel,
  getGetCatalogQueryKey,
  getGetStorageQueryKey,
  redownloadLocalModel,
  setModelLocation,
  useGetCatalog,
  useGetStorage,
} from '@/lib/api/default/default'
import type { LlmCatalogModel, ModelLocationMode } from '@/lib/api/schemas'
import { isTauri } from '@/lib/backend'
import { formatBytes } from '@/lib/format'
import { useDownloadsStore } from '@/lib/stores/downloadsStore'
import { useEditorUiStore } from '@/lib/stores/editorUiStore'
import { useJobsStore } from '@/lib/stores/jobsStore'
import { useNotificationsStore } from '@/lib/stores/notificationsStore'

type Confirmation =
  | { kind: 'location'; mode: ModelLocationMode }
  | { kind: 'cache' }
  | { kind: 'models' }
  | { kind: 'delete'; model: LlmCatalogModel }
  | { kind: 'redownload'; model: LlmCatalogModel }

export function ModelStorageManager() {
  const { t } = useTranslation()
  const queryClient = useQueryClient()
  const {
    data: storage,
    isError: storageFailed,
    isFetching: storageFetching,
    refetch: refetchStorage,
  } = useGetStorage()
  const { data: catalog } = useGetCatalog()
  const [pathDraft, setPathDraft] = useState('')
  const [confirmation, setConfirmation] = useState<Confirmation | null>(null)
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState<string>()
  const [restartPending, setRestartPending] = useState(false)
  const hasActiveDownloads = useDownloadsStore((state) =>
    Object.values(state.downloads).some(
      (download) =>
        download.status.status === 'started' || download.status.status === 'downloading',
    ),
  )
  const hasRunningJobs = useJobsStore((state) =>
    Object.values(state.jobs).some((job) => job.status === 'running'),
  )

  useEffect(() => {
    if (!restartPending && storage?.modelsPath) setPathDraft(storage.modelsPath)
  }, [restartPending, storage?.modelsPath])

  const downloadedModels = useMemo(
    () => (catalog?.localModels ?? []).filter((model) => model.downloaded),
    [catalog?.localModels],
  )

  const refreshStorage = async () => {
    await Promise.all([
      queryClient.invalidateQueries({ queryKey: getGetStorageQueryKey() }),
      queryClient.invalidateQueries({ queryKey: getGetCatalogQueryKey() }),
    ])
  }

  const chooseDirectory = async () => {
    if (!isTauri()) return
    try {
      const { open } = await import('@tauri-apps/plugin-dialog')
      const selected = await open({
        directory: true,
        multiple: false,
        title: t('settings.modelsPathChoose'),
      })
      if (typeof selected === 'string') setPathDraft(selected)
    } catch (cause) {
      setError(errorMessage(cause))
    }
  }

  const notifyCleanup = (id: string, titleKey: string, removedBytes: number) => {
    useNotificationsStore.getState().upsert({
      id,
      tone: 'success',
      titleKey,
      messageKey: 'settings.spaceFreed',
      values: { size: formatBytes(removedBytes) },
    })
  }

  const runConfirmedAction = async () => {
    const action = confirmation
    const blockedByActiveWork = hasActiveDownloads || (action?.kind !== 'cache' && hasRunningJobs)
    if (!action || busy || restartPending || blockedByActiveWork) return
    setConfirmation(null)
    setBusy(true)
    setError(undefined)
    try {
      if (action.kind === 'location') {
        const path = pathDraft.trim()
        if (!path) throw new Error(t('settings.modelsPathRequired'))
        const result = await setModelLocation({ path, mode: action.mode })
        useNotificationsStore.getState().upsert({
          id: 'models-location-changed',
          tone: 'success',
          titleKey: 'settings.modelsPathChanged',
          messageKey: result.sourceRemoved
            ? 'settings.modelsPathChangedDescription'
            : 'settings.modelsPathChangedSourceKept',
          values: { size: formatBytes(result.copiedBytes) },
        })
        setPathDraft(result.modelsPath)
        if (result.restartRequired) {
          setRestartPending(true)
          if (!isTauri()) {
            setError(t('settings.restartManually'))
            return
          }
          try {
            const { relaunch } = await import('@tauri-apps/plugin-process')
            await relaunch()
          } catch {
            setError(t('settings.restartManually'))
          }
          return
        }
        await refreshStorage()
        return
      }

      if (action.kind === 'cache') {
        const result = await clearTemporaryCache()
        notifyCleanup('temporary-cache-cleared', 'settings.cacheCleared', result.removedBytes)
      } else if (action.kind === 'models') {
        const result = await clearModels()
        notifyCleanup('models-cleared', 'settings.modelsCleared', result.removedBytes)
      } else if (action.kind === 'delete') {
        const result = await deleteLocalModel(action.model.target.modelId)
        notifyCleanup(
          `model-deleted:${action.model.target.modelId}`,
          'settings.modelDeleted',
          result.removedBytes,
        )
      } else {
        const result = await redownloadLocalModel(action.model.target.modelId)
        useDownloadsStore.getState().progress({
          id: result.operationId,
          filename: action.model.name,
          downloaded: 0,
          status: { status: 'started' },
        })
        useNotificationsStore.getState().upsert({
          id: `model-redownload:${action.model.target.modelId}`,
          tone: 'info',
          titleKey: 'settings.modelRedownloadStarted',
          messageKey: 'settings.modelRedownloadStartedDescription',
          values: { name: action.model.name },
        })
      }
      await refreshStorage()
    } catch (cause) {
      const message = errorMessage(cause)
      setError(message)
      useEditorUiStore.getState().showError(message)
    } finally {
      setBusy(false)
    }
  }

  const confirmationCopy = getConfirmationCopy(confirmation, t)
  const locationUnchanged = pathDraft.trim() === storage?.modelsPath
  const modelStorageLocked = busy || restartPending || hasActiveDownloads || hasRunningJobs
  const cacheCleanupLocked = busy || restartPending || hasActiveDownloads
  const confirmationBlocked =
    busy ||
    restartPending ||
    hasActiveDownloads ||
    (confirmation?.kind !== 'cache' && hasRunningJobs)

  return (
    <>
      <div className='space-y-5 rounded-xl border border-border bg-card p-4'>
        <div className='flex items-start justify-between gap-4'>
          <div>
            <h4 className='text-sm font-semibold'>{t('settings.modelStorage')}</h4>
            <p className='mt-1 text-xs leading-relaxed text-muted-foreground'>
              {t('settings.modelStorageDescription')}
            </p>
          </div>
          {storageFetching && (
            <LoaderCircleIcon className='size-4 animate-spin text-muted-foreground' />
          )}
        </div>

        {storageFailed && (
          <div
            role='alert'
            className='flex items-center justify-between gap-3 rounded-lg border border-destructive/30 bg-destructive/5 px-3 py-2'
          >
            <p className='text-xs text-destructive'>{t('settings.modelStorageLoadFailed')}</p>
            <Button
              type='button'
              variant='outline'
              size='sm'
              disabled={storageFetching}
              onClick={() => void refetchStorage()}
            >
              {t('bootstrap.retryNow')}
            </Button>
          </div>
        )}

        {storage && (
          <div className='grid grid-cols-3 gap-2'>
            <StorageMetric
              label={t('settings.downloadedModels')}
              value={storage.downloadedLocalModels}
            />
            <StorageMetric
              label={t('settings.modelSpace')}
              value={formatBytes(storage.modelsBytes)}
            />
            <StorageMetric
              label={t('settings.temporarySpace')}
              value={formatBytes(storage.temporaryBytes)}
            />
          </div>
        )}

        <div className='space-y-1.5'>
          <Label className='text-xs'>{t('settings.modelsPath')}</Label>
          <div className='flex gap-2'>
            <Input
              value={pathDraft}
              onChange={(event) => setPathDraft(event.target.value)}
              placeholder={t('settings.modelsPathPlaceholder')}
            />
            <Button
              type='button'
              variant='outline'
              onClick={() => void chooseDirectory()}
              disabled={!isTauri() || busy}
              aria-label={t('settings.modelsPathBrowse')}
            >
              <FolderOpenIcon className='size-4' />
            </Button>
          </div>
          <p className='text-xs leading-relaxed text-muted-foreground'>
            {t('settings.modelsPathDescription')}
          </p>
          <div className='flex flex-wrap justify-end gap-2 pt-1'>
            <Button
              variant='outline'
              size='sm'
              disabled={modelStorageLocked || !pathDraft.trim() || locationUnchanged}
              onClick={() => setConfirmation({ kind: 'location', mode: 'use_existing' })}
            >
              {t('settings.modelsPathUse')}
            </Button>
            <Button
              size='sm'
              disabled={modelStorageLocked || !pathDraft.trim() || locationUnchanged}
              onClick={() => setConfirmation({ kind: 'location', mode: 'move_existing' })}
            >
              {t('settings.modelsPathMove')}
            </Button>
          </div>
        </div>

        <div className='space-y-2'>
          <div className='flex items-center justify-between gap-3'>
            <div>
              <p className='text-xs font-medium'>{t('settings.temporaryCache')}</p>
              <p className='text-[11px] text-muted-foreground'>
                {t('settings.temporaryCacheDescription')}
              </p>
            </div>
            <Button
              variant='outline'
              size='sm'
              disabled={cacheCleanupLocked || !storage?.temporaryBytes}
              onClick={() => setConfirmation({ kind: 'cache' })}
            >
              <Trash2Icon className='size-3.5' />
              {t('settings.clearCache')}
            </Button>
          </div>
        </div>

        <div className='space-y-2 border-t border-border pt-4'>
          <div className='flex items-center justify-between gap-3'>
            <p className='text-xs font-medium'>{t('settings.downloadedModels')}</p>
            <Button
              variant='destructive'
              size='sm'
              disabled={modelStorageLocked || !storage?.modelsBytes}
              onClick={() => setConfirmation({ kind: 'models' })}
            >
              <Trash2Icon className='size-3.5' />
              {t('settings.deleteAllModels')}
            </Button>
          </div>
          {downloadedModels.length === 0 ? (
            <p className='rounded-lg bg-muted/50 px-3 py-4 text-center text-xs text-muted-foreground'>
              {t('settings.noDownloadedModels')}
            </p>
          ) : (
            <div className='space-y-2'>
              {downloadedModels.map((model) => (
                <div
                  key={model.target.modelId}
                  className='flex items-center justify-between gap-3 rounded-lg border border-border px-3 py-2'
                >
                  <div className='min-w-0'>
                    <p className='truncate text-xs font-medium' title={model.name}>
                      {model.name}
                    </p>
                    <p className='text-[11px] text-muted-foreground'>
                      {formatBytes(model.sizeBytes)}
                    </p>
                  </div>
                  <div className='flex shrink-0 gap-1'>
                    <Button
                      variant='ghost'
                      size='icon-sm'
                      disabled={modelStorageLocked}
                      onClick={() => setConfirmation({ kind: 'redownload', model })}
                      aria-label={t('settings.redownloadModel', { name: model.name })}
                    >
                      <RefreshCwIcon className='size-3.5' />
                    </Button>
                    <Button
                      variant='ghost'
                      size='icon-sm'
                      disabled={modelStorageLocked}
                      onClick={() => setConfirmation({ kind: 'delete', model })}
                      aria-label={t('settings.deleteModel', { name: model.name })}
                      className='text-destructive hover:text-destructive'
                    >
                      <Trash2Icon className='size-3.5' />
                    </Button>
                  </div>
                </div>
              ))}
            </div>
          )}
          <p className='text-[11px] leading-relaxed text-muted-foreground'>
            {t('settings.modelSpaceDescription')}
          </p>
        </div>

        {busy && (
          <p className='flex items-center gap-2 text-xs text-muted-foreground'>
            <LoaderCircleIcon className='size-3.5 animate-spin' />
            {t('settings.storageWorking')}
          </p>
        )}
        {error && <p className='text-xs text-destructive'>{error}</p>}
      </div>

      <AlertDialog
        open={confirmation !== null}
        onOpenChange={(open) => !open && setConfirmation(null)}
      >
        <AlertDialogContent>
          <AlertDialogTitle>{confirmationCopy.title}</AlertDialogTitle>
          <AlertDialogDescription>{confirmationCopy.description}</AlertDialogDescription>
          <div className='flex justify-end gap-2'>
            <AlertDialogCancel>{t('common.cancel')}</AlertDialogCancel>
            <AlertDialogAction
              disabled={confirmationBlocked}
              onClick={() => void runConfirmedAction()}
            >
              {confirmationCopy.action}
            </AlertDialogAction>
          </div>
        </AlertDialogContent>
      </AlertDialog>
    </>
  )
}

function StorageMetric({ label, value }: { label: string; value: number | string }) {
  return (
    <div className='rounded-lg bg-muted/50 px-3 py-2'>
      <p className='text-[10px] text-muted-foreground'>{label}</p>
      <p className='mt-0.5 text-xs font-semibold tabular-nums'>{value}</p>
    </div>
  )
}

function getConfirmationCopy(
  confirmation: Confirmation | null,
  t: ReturnType<typeof useTranslation>['t'],
): { title: string; description: string; action: string } {
  if (!confirmation) return { title: '', description: '', action: '' }
  if (confirmation.kind === 'location') {
    return {
      title: t('settings.modelsPathConfirmTitle'),
      description: t(
        confirmation.mode === 'move_existing'
          ? 'settings.modelsPathMoveConfirm'
          : 'settings.modelsPathUseConfirm',
      ),
      action: t('settings.restartApply'),
    }
  }
  if (confirmation.kind === 'cache') {
    return {
      title: t('settings.clearCacheConfirmTitle'),
      description: t('settings.clearCacheConfirmDescription'),
      action: t('settings.clearCache'),
    }
  }
  if (confirmation.kind === 'models') {
    return {
      title: t('settings.deleteAllModelsConfirmTitle'),
      description: t('settings.deleteAllModelsConfirmDescription'),
      action: t('settings.deleteAllModels'),
    }
  }
  if (confirmation.kind === 'redownload') {
    return {
      title: t('settings.redownloadModelConfirmTitle'),
      description: t('settings.redownloadModelConfirmDescription', {
        name: confirmation.model.name,
      }),
      action: t('settings.redownload'),
    }
  }
  return {
    title: t('settings.deleteModelConfirmTitle'),
    description: t('settings.deleteModelConfirmDescription', { name: confirmation.model.name }),
    action: t('common.delete'),
  }
}

function errorMessage(cause: unknown): string {
  return cause instanceof Error ? cause.message : String(cause)
}
