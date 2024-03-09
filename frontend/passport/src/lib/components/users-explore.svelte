<script lang="ts">
  import ExploreIconRaw from "$lib/assets/world.svg?raw"
  import { fade } from "svelte/transition"
  import { getModalStore } from "@skeletonlabs/skeleton"
  import { searchBarModal } from "$lib/modals"

  const modalStore = getModalStore()
  const triggerModal = () => {
    modalStore.trigger(searchBarModal)
  }
</script>

<div in:fade={{ duration: 100 }} class="flex justify-center mt-10">
  <button on:click={triggerModal} class="btn variant-outline-primary px-10 flex gap-x-2">
    <span class="block h-[28px] w-[28px] relative top-[0.5px]">{@html ExploreIconRaw}</span>
    <span class="text-xl">Explore</span>
  </button>
</div>
