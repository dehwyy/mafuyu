<script lang="ts">
  import ThemeSelectIconRaw from "$lib/assets/theme-select.svg?raw"
  import { type PopupSettings, popup } from "@skeletonlabs/skeleton"
  import { ListBox, ListBoxItem } from "@skeletonlabs/skeleton"
  import { onMount } from "svelte"

  const themes = {
    darkest: "🦀 Darkest",
    modern: "🤖 Modern",
    wintry: "❄️ Wintry",
    crimson: "🕸️ Crimson",
  }

  let current_theme: keyof typeof themes | undefined

  onMount(() => {
    current_theme = (localStorage.getItem("theme") as keyof typeof themes) || "darkest"
  })

  const themeSelectClick: PopupSettings = {
    target: "theme-select-popup",
    placement: "left",
    event: "click",
  }

  const SetTheme = (selected_theme: string) => {
    const body = document.querySelector("body")!

    body.dataset.theme = selected_theme
    localStorage.setItem("theme", selected_theme)
  }
</script>

{#if current_theme}
  <div use:popup={themeSelectClick} class="cursor-pointer flex-auto">
    <div
      class="flex items-center h-[40px] hover:bg-surface-300/10 text-surface-200 hover:text-white w-full pl-4 gap-x-2 font-medium rounded-3xl select-none">
      <span class="icon-sm block">{@html ThemeSelectIconRaw}</span>
      <p>{themes[current_theme]}</p>
    </div>
  </div>
  <div data-popup="theme-select-popup" class="card shadow-md shadow-primary-500 rounded-2xl overflow-hidden !left-[-215px] min-w-[200px] py-4 px-1">
    <ListBox>
      {#each Object.entries(themes) as entry}
        <ListBoxItem on:click={() => SetTheme(entry[0])} bind:group={current_theme} name="theme" value={entry[0]}>{entry[1]}</ListBoxItem>
      {/each}
    </ListBox>
  </div>
{/if}
