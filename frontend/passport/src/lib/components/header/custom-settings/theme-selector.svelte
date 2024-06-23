<script lang="ts">
  import { ListBox, ListBoxItem, popup } from '@skeletonlabs/skeleton'
  import ThemeSelectIconRaw from '$lib/assets/theme-select.svg?raw'
  import { Themes } from '$lib/const'
  import { settingsStore, updateSettingsStore } from '$lib/stores/settings'
  import { onMount } from 'svelte'
  import type { PopupSettings } from '@skeletonlabs/skeleton'

  const themes = {
    [Themes.darkest]: '🦀 Darkest',
    [Themes.modern]: '🤖 Modern',
    [Themes.wintry]: '❄️ Wintry',
    [Themes.crimson]: '🕸️ Crimson'
  }
  let currentTheme: Themes | undefined

  onMount(() => {
    currentTheme = $settingsStore.theme as Themes
  })

  const themeSelectClick: PopupSettings = {
    target: 'theme-select-popup',
    placement: 'left',
    event: 'click'
  }

  const SetTheme = (selectedTheme: string) => {
    updateSettingsStore({ theme: selectedTheme })

    const body = document.querySelector('body')!
    body.dataset.theme = selectedTheme
  }
</script>

{#if currentTheme}
  <div
    use:popup={themeSelectClick}
    class="cursor-pointer flex-auto"
  >
    <div
      class="flex items-center h-[40px] hover:bg-surface-300/10 text-surface-200 hover:text-white w-full pl-4 gap-x-2 rounded-3xl select-none"
    >
      <span class="icon-sm block">{@html ThemeSelectIconRaw}</span>
      <p>{themes[currentTheme]}</p>
    </div>
  </div>
  <div
    data-popup="theme-select-popup"
    class="pr-1"
  >
    <div
      class="card shadow-md shadow-primary-500 rounded-2xl overflow-hidden min-w-[200px] py-4 px-1"
    >
      <ListBox>
        {#each Object.entries(themes) as entry}
          <ListBoxItem
            on:click={() => SetTheme(entry[0])}
            bind:group={currentTheme}
            name="theme"
            value={entry[0]}>{entry[1]}</ListBoxItem
          >
        {/each}
      </ListBox>
    </div>
  </div>
{/if}
