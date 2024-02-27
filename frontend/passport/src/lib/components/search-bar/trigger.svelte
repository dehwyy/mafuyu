<script lang="ts">
  import LoopIconRaw from "$lib/assets/loop.svg?raw"
  import SearchBar from "./search-bar.svelte"
  import { getModalStore, type ModalSettings } from "@skeletonlabs/skeleton"

  const modalStore = getModalStore()

  const searchBarModal: ModalSettings = {
    title: "SearchBar",
    type: "component",
    position: "top-0",
    component: { ref: SearchBar },
  }

  const triggerModal = () => {
    modalStore.trigger(searchBarModal)
  }
</script>

<div>
  <button on:click={() => triggerModal()} class="flex gap-x-3 btn variant-outline-primary pr-10">
    <span class="icon-sm block">{@html LoopIconRaw}</span>
    <span>Search</span>
  </button>
</div>
