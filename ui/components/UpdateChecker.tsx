'use client'

import { createContext, useCallback, useContext, useEffect, useMemo, useState } from 'react'
import type { ReactNode } from 'react'

import { isTauri, openExternalUrl } from '@/lib/backend'
import { useNotificationsStore } from '@/lib/stores/notificationsStore'
import packageInfo from '@/package.json'

export type UpdateStatus = 'idle' | 'checking' | 'current' | 'available' | 'error'

type LatestRelease = {
  tag_name: string
  html_url: string
}

type UpdateCheckerContextValue = {
  currentVersion: string
  latestVersion?: string
  latestUrl: string
  status: UpdateStatus
  checkForUpdates: () => Promise<void>
  openLatestRelease: () => Promise<void>
}

export const LATEST_RELEASE_URL = 'https://github.com/proxlavee/yomika/releases/latest'
const LATEST_RELEASE_API = 'https://api.github.com/repos/proxlavee/yomika/releases/latest'
const UPDATE_NOTIFICATION_ID = 'update-available'
const UPDATE_CHECK_TIMEOUT_MS = 10_000

const UpdateCheckerContext = createContext<UpdateCheckerContextValue>({
  currentVersion: packageInfo.version,
  latestUrl: LATEST_RELEASE_URL,
  status: 'idle',
  checkForUpdates: async () => {},
  openLatestRelease: async () => {},
})

function parseVersion(version: string): [number, number, number] | null {
  const match = version.trim().match(/^v?(\d+)\.(\d+)\.(\d+)(?:[-+].*)?$/i)
  if (!match) return null
  return [Number(match[1]), Number(match[2]), Number(match[3])]
}

export function isNewerVersion(candidate: string, current: string): boolean {
  const candidateParts = parseVersion(candidate)
  const currentParts = parseVersion(current)
  if (!candidateParts || !currentParts) return false
  for (let index = 0; index < candidateParts.length; index += 1) {
    if (candidateParts[index] !== currentParts[index]) {
      return candidateParts[index] > currentParts[index]
    }
  }
  return false
}

async function resolveDesktopVersion(): Promise<string> {
  if (!isTauri()) return packageInfo.version
  try {
    const { getVersion } = await import('@tauri-apps/api/app')
    return await getVersion()
  } catch {
    return packageInfo.version
  }
}

export function useUpdateChecker(): UpdateCheckerContextValue {
  return useContext(UpdateCheckerContext)
}

export function UpdateCheckerProvider({ children }: { children: ReactNode }) {
  const [currentVersion, setCurrentVersion] = useState(packageInfo.version)
  const [latestVersion, setLatestVersion] = useState<string>()
  const [latestUrl, setLatestUrl] = useState(LATEST_RELEASE_URL)
  const [status, setStatus] = useState<UpdateStatus>('idle')

  useEffect(() => {
    let active = true
    void resolveDesktopVersion().then((version) => {
      if (active) setCurrentVersion(version)
    })
    return () => {
      active = false
    }
  }, [])

  const checkForUpdates = useCallback(async () => {
    setStatus('checking')
    const controller = new AbortController()
    const timeout = window.setTimeout(() => controller.abort(), UPDATE_CHECK_TIMEOUT_MS)
    try {
      const response = await fetch(LATEST_RELEASE_API, {
        headers: { Accept: 'application/vnd.github+json' },
        cache: 'no-store',
        signal: controller.signal,
      })
      if (!response.ok) throw new Error(`GitHub release check failed (${response.status})`)
      const release = (await response.json()) as LatestRelease
      const version = release.tag_name.replace(/^v/i, '')
      const url = release.html_url || LATEST_RELEASE_URL
      setLatestVersion(version)
      setLatestUrl(url)
      if (isNewerVersion(version, currentVersion)) {
        setStatus('available')
        useNotificationsStore.getState().upsert({
          id: UPDATE_NOTIFICATION_ID,
          tone: 'info',
          titleKey: 'updates.available.title',
          messageKey: 'updates.available.description',
          values: { version },
          actionLabelKey: 'updates.openRelease',
          actionUrl: url,
        })
      } else {
        setStatus('current')
        useNotificationsStore.getState().remove(UPDATE_NOTIFICATION_ID)
      }
    } catch (error) {
      console.warn('[updates] check failed', error)
      setStatus('error')
    } finally {
      window.clearTimeout(timeout)
    }
  }, [currentVersion])

  useEffect(() => {
    void checkForUpdates()
  }, [checkForUpdates])

  const openLatestRelease = useCallback(
    () => openExternalUrl(latestUrl || LATEST_RELEASE_URL),
    [latestUrl],
  )

  const value = useMemo<UpdateCheckerContextValue>(
    () => ({
      currentVersion,
      latestVersion,
      latestUrl,
      status,
      checkForUpdates,
      openLatestRelease,
    }),
    [checkForUpdates, currentVersion, latestUrl, latestVersion, openLatestRelease, status],
  )

  return <UpdateCheckerContext.Provider value={value}>{children}</UpdateCheckerContext.Provider>
}
