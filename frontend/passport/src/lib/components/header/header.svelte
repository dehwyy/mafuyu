<script lang="ts">
  import { fade } from "svelte/transition"
  import MafuyuIcon from "$lib/assets/mafuyu.svg?raw"
  import { AppBar } from "@skeletonlabs/skeleton"

  import SearchBarTrigger from "$lib/components/search-bar/trigger.svelte"
  import Navigation from "$lib/components/header/nav-card.svelte"

  import { settingsStore } from "$lib/stores/settings"
  import { onMount } from "svelte"

  let isMinimalistic = true

  onMount(() => {
    isMinimalistic = $settingsStore?.minimalisticHeader
    settingsStore.subscribe(v => {
      isMinimalistic = v.minimalisticHeader
    })
  })

  $: border = isMinimalistic ? "border-transparent" : "border-b-2 border-surface-600"
  $: background = isMinimalistic ? "bg-transparent" : "bg-surface-800"
</script>

<AppBar {border} {background} transition="transition-all" padding="p-2">
  <svelte:fragment slot="lead">
    {#if !isMinimalistic}
      <a transition:fade={{ duration: 300 }} href="/">
        <h3 class="h3 ml-3 h-[28px]">
          {@html MafuyuIcon}
        </h3>
      </a>
    {/if}
  </svelte:fragment>
  <svelte:fragment slot="trail">
    <div class="pr-5 flex gap-x-7 mt-1">
      {#if !isMinimalistic}
        <SearchBarTrigger />
      {/if}
      <Navigation />
    </div>
  </svelte:fragment>
</AppBar>

<style>
  :global(#shell-header .app-bar) {
    @apply transition-all duration-300;
  }
</style>
