<script lang="ts">
  import RainbowIconRaw from "$lib/assets/rainbow.svg?raw"
  import { SlideToggle } from "@skeletonlabs/skeleton"
  import { Icon, Card } from "$lib/components/header/nav-items"
  import { settingsStore, updateSettingsStore } from "$lib/stores/settings"

  $: onChange = () => updateSettingsStore({ rgbCard: !$settingsStore.rgbCard })
</script>

<Card>
  <div class="flex">
    <Icon rawIcon={RainbowIconRaw} />
    <p>RGB Card</p>
  </div>
  <SlideToggle size="sm" name="rgbCard" checked={$settingsStore.rgbCard} on:change={onChange} />
</Card>
