import type { ModalSettings } from "@skeletonlabs/skeleton"
import SearchBar from "$lib/components/search-bar/search-bar.svelte"

export const searchBarModal: ModalSettings = {
  title: "SearchBar",
  type: "component",
  position: "top-0",
  component: { ref: SearchBar },
}
