import { onMount } from 'svelte'
import { writable } from 'svelte/store'

export const useIsMounted = () => {
  const isMoutedStore = writable(false)

  onMount(() => {
    isMoutedStore.set(true)
  })

  return isMoutedStore
}
