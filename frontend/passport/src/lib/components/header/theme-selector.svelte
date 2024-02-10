<script lang="ts">
  import { type PopupSettings, popup } from "@skeletonlabs/skeleton"
  import { ListBox, ListBoxItem } from "@skeletonlabs/skeleton"
  import { onMount } from "svelte"

  const themes = {
    darkest: "🦀 Darkest",
    modern: "🤖 Modern",
    wintry: "❄️ Wintry",
    crimson: "🕸️ Crimson",
    "gold-nouveau": "⭐Goldouveau",
  }

  let current_theme: keyof typeof themes | undefined

  onMount(() => {
    current_theme = localStorage.getItem("theme") as keyof typeof themes || "darkest"
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
  {themes[current_theme]}
</div>
<div data-popup="theme-select-popup" class="card rounded-2xl overflow-hidden !left-[-150px]">
  <ListBox>
    {#each Object.entries(themes) as entry}
      <ListBoxItem on:click={() => SetTheme(entry[0])} bind:group={current_theme} name="theme" value={entry[0]}>{entry[1]}</ListBoxItem>
    {/each}
  </ListBox>
</div>
{/if}
