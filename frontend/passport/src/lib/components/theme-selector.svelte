<script lang="ts">
  import { type PopupSettings, popup } from "@skeletonlabs/skeleton"
  import { ListBox, ListBoxItem } from "@skeletonlabs/skeleton"

  const themes = {
    darkest: "🦀 Darkest",
    modern: "🤖 Modern",
    wintry: "❄️ Wintry",
    crimson: "🕸️ Crimson",
    "gold-nouveau": "⭐Goldouveau",
  }

  let current_theme: keyof typeof themes = "darkest"

  const themeSelectClick: PopupSettings = {
    target: "theme-select-popup",
    placement: "right",
    event: "click",
  }

  $: {
    if (typeof window !== "undefined") {
      const body = document.querySelector("body")
      if (body) {
        body.dataset.theme = current_theme
      }
    }
  }
</script>

<div use:popup={themeSelectClick} class="cursor-pointer">
  {themes[current_theme]}
</div>
<div data-popup="theme-select-popup" class="ml-7 card p-4">
  <ListBox>
    {#each Object.entries(themes) as entry}
      <ListBoxItem bind:group={current_theme} name="theme" value={entry[0]}>{entry[1]}</ListBoxItem>
    {/each}
  </ListBox>
</div>
