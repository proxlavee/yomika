'use client'

import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'

export type NotificationTone = 'info' | 'success' | 'warning' | 'error'

export type AppNotification = {
  id: string
  tone: NotificationTone
  titleKey: string
  messageKey?: string
  values?: Record<string, number | string>
  actionLabelKey?: string
  actionUrl?: string
}

type NotificationsState = {
  notifications: AppNotification[]
  upsert: (notification: AppNotification) => void
  remove: (id: string) => void
  clear: () => void
}

const MAX_NOTIFICATIONS = 5

export const useNotificationsStore = create<NotificationsState>()(
  immer((set) => ({
    notifications: [],
    upsert: (notification) =>
      set((state) => {
        state.notifications = [
          notification,
          ...state.notifications.filter(({ id }) => id !== notification.id),
        ].slice(0, MAX_NOTIFICATIONS)
      }),
    remove: (id) =>
      set((state) => {
        state.notifications = state.notifications.filter((notification) => notification.id !== id)
      }),
    clear: () =>
      set((state) => {
        state.notifications = []
      }),
  })),
)
