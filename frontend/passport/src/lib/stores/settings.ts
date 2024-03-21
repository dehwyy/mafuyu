import { localStorageStore } from "@skeletonlabs/skeleton"
import type { Writable } from "svelte/store"

interface ISettings {
  animatedBackground: boolean
  rgbCard: boolean
  theme: string
}

export const settingsStore: Writable<ISettings> = localStorageStore("settings", {
  animatedBackground: true,
  rgbCard: true,
  theme: "darkest",
})

export const updateSettingsStore = (settings: Partial<ISettings>) => {
  settingsStore.update(v => ({ ...v, ...settings }))
}
