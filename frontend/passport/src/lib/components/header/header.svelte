<script lang="ts">
  import { AppBar } from '@skeletonlabs/skeleton'
  import { page } from '$app/stores'
  import MafuyuIcon from '$lib/assets/mafuyu.svg?raw'
  import Navigation from '$lib/components/header/nav-card.svelte'
  import SearchBarTrigger from '$lib/components/search-bar/trigger.svelte'
  import { settingsStore } from '$lib/stores/settings'
  import { onMount } from 'svelte'
  import { fade } from 'svelte/transition'

  let isMinimalistic = true

  onMount(() => {
    isMinimalistic = $settingsStore?.minimalisticHeader

    const url = $page.data.url

    if (!url.startsWith('/m/')) {
      settingsStore.subscribe((v) => {
        isMinimalistic = v.minimalisticHeader
      })
    } else {
      isMinimalistic = true
    }
  })

  $: border =
    'border-b-2' + isMinimalistic ? 'border-surface-800' : 'border-surface-600'
</script>

<AppBar
  {border}
  background="bg-surface-800"
  padding="p-2"
>
  <svelte:fragment slot="lead">
    {#if !isMinimalistic}
      <a
        transition:fade={{ duration: 300 }}
        href="/"
      >
        <h3 class="h3 ml-3 h-[26px]">
          {@html MafuyuIcon}
        </h3>
      </a>
    {/if}
  </svelte:fragment>
  <svelte:fragment slot="trail">
    <div class="pr-5 flex gap-x-7 mt-1">
      {#if !isMinimalistic}
        <div transition:fade={{ duration: 300 }}>
          <SearchBarTrigger />
        </div>
      {/if}
      <Navigation />
    </div>
  </svelte:fragment>
</AppBar>

<style>
  :global(.app-bar) {
    @apply transition-all duration-300 min-h-[64px] h-[64px];
  }
</style>
