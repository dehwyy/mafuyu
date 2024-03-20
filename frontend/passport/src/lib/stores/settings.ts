import { localStorageStore } from "@skeletonlabs/skeleton"
import type { Writable } from "svelte/store"

interface ISettings {
  animatedBackground: boolean
  theme: string
}

export const settingsStore: Writable<ISettings> = localStorageStore("settings", {
  animatedBackground: false,
  theme: "darkest",
})
