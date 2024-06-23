<script lang="ts">
  import { getModalStore } from '@skeletonlabs/skeleton'
  import ExploreIconRaw from '$lib/assets/explore.svg?raw'
  import { searchBarModal } from '$lib/modals'
  import { fade } from 'svelte/transition'

  const modalStore = getModalStore()
  const triggerModal = () => {
    modalStore.trigger(searchBarModal)
  }
</script>

<div
  in:fade={{ duration: 100 }}
  class="flex justify-center mt-10"
>
  <button
    on:click={triggerModal}
    class="btn variant-outline-primary px-10 flex h-min gap-x-2"
  >
    <span class="block h-[28px] w-[28px] relative top-[0.5px]"
      >{@html ExploreIconRaw}</span
    >
    <span class="text-xl">Explore</span>
  </button>
</div>
