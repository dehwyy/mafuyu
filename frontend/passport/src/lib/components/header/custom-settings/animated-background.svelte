<script lang="ts">
  import SparklesIconRaw from "$lib/assets/sparkles.svg?raw"
  import { onMount } from "svelte"
  import { SlideToggle } from "@skeletonlabs/skeleton"
  import { settingsStore } from "$lib/stores/settings"

  $: isAnimatedBackground = $settingsStore.animatedBackground
</script>

<div class="flex justify-between px-4">
  <div class="flex">
    <p class="h-[24px] mr-2">{@html SparklesIconRaw}</p>
    <p class="">Animated BG</p>
  </div>
  <SlideToggle
    size="sm"
    name="animatedBackground"
    on:change={() => settingsStore.update(v => ({ ...v, animatedBackground: !isAnimatedBackground }))} />
</div>
