import { localStorageStore } from '@skeletonlabs/skeleton'
import { Themes } from '$lib/const'
import type { Writable } from 'svelte/store'

export interface IToggleableSettings {
  animatedBackground: boolean
  rgbCard: boolean
  minimalisticHeader: boolean
}

interface ISettings extends IToggleableSettings {
  theme: string
}

export const settingsStore: Writable<ISettings> = localStorageStore(
  'settings',
  {
    animatedBackground: true,
    rgbCard: true,
    minimalisticHeader: false,
    theme: Themes.darkest
  }
)

export const updateSettingsStore = (settings: Partial<ISettings>) => {
  settingsStore.update((v) => ({ ...v, ...settings }))
}
