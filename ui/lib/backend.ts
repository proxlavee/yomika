'use client'

type ProgressTarget = {
  setProgressBar: (options: { status?: ProgressBarStatus; progress?: number }) => Promise<void>
}

export enum ProgressBarStatus {
  None = 'none',
  Normal = 'normal',
  Indeterminate = 'indeterminate',
  Paused = 'paused',
  Error = 'error',
}

export const isTauri = (): boolean =>
  typeof window !== 'undefined' && !!(window as any).__TAURI_INTERNALS__

export async function openExternalUrl(url: string): Promise<void> {
  if (isTauri()) {
    const { openUrl } = await import('@tauri-apps/plugin-opener')
    await openUrl(url)
    return
  }

  if (typeof window !== 'undefined') {
    window.open(url, '_blank', 'noopener,noreferrer')
  }
}

export async function listen<T>(
  event: string,
  handler: (event: { payload: T }) => void,
): Promise<() => void> {
  if (isTauri()) {
    const { listen } = await import('@tauri-apps/api/event')
    return listen<T>(event, handler)
  }

  if (typeof window !== 'undefined' && event === 'tauri://resize') {
    const listener = () => handler({ payload: undefined as T })
    window.addEventListener('resize', listener)
    return async () => window.removeEventListener('resize', listener)
  }

  return async () => {}
}

/**
 * Restore desktop-window interaction after system UI closes. WebView2 can
 * leave the native window disabled as well as unfocused, so both operations
 * are attempted independently. Best-effort and a no-op in browsers.
 */
export async function restoreAppWindowInteraction(): Promise<void> {
  if (!isTauri()) return

  let appWindow: {
    setEnabled: (enabled: boolean) => Promise<void>
    setFocus: () => Promise<void>
  }
  try {
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    appWindow = getCurrentWindow()
  } catch {
    return
  }

  try {
    await appWindow.setEnabled(true)
  } catch {
    // Continue: focus may still be recoverable when enabling is unsupported.
  }
  try {
    await appWindow.setFocus()
  } catch {
    // Recovery is best-effort; never break the EyeDropper caller over it.
  }
}

export function getCurrentWindow(): ProgressTarget {
  if (isTauri()) {
    return {
      async setProgressBar(options) {
        const { getCurrentWindow } = await import('@tauri-apps/api/window')
        return getCurrentWindow().setProgressBar(options)
      },
    }
  }

  return {
    async setProgressBar() {
      return
    },
  }
}
