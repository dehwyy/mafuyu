<script lang="ts">
  import SparklesIconRaw from "$lib/assets/sparkles.svg?raw"
  import { SlideToggle } from "@skeletonlabs/skeleton"
  import { Icon, Card } from "$lib/components/header/nav-items"
  import { settingsStore, updateSettingsStore } from "$lib/stores/settings"

  $: onChange = () => updateSettingsStore({ animatedBackground: !$settingsStore.animatedBackground })
</script>

<Card>
  <div class="flex">
    <Icon rawIcon={SparklesIconRaw} />
    <p>Animated background</p>
  </div>
  <SlideToggle size="sm" name="animatedBackground" checked={$settingsStore.animatedBackground} on:change={onChange} />
</Card>
