import SearchBar from '$lib/components/search-bar/search-bar.svelte'
import type { ModalSettings } from '@skeletonlabs/skeleton'

export const searchBarModal: ModalSettings = {
  title: 'SearchBar',
  type: 'component',
  position: 'top-0',
  component: { ref: SearchBar }
}
