<script lang="ts">
  import LoopIconRaw from "$lib/assets/loop.svg?raw"
  import { getModalStore } from "@skeletonlabs/skeleton"
  import { searchBarModal } from "$lib/modals"

  const modalStore = getModalStore()
  const triggerModal = () => {
    modalStore.trigger(searchBarModal)
  }
</script>

<div>
  <button on:click={() => triggerModal()} class="flex gap-x-3 btn variant-filled-surface pr-10">
    <span class="icon-sm block">{@html LoopIconRaw}</span>
    <span>Search</span>
  </button>
</div>
