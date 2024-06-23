<script lang="ts">
  import { SlideToggle } from '@skeletonlabs/skeleton'
  import { Card, Icon } from '$lib/components/header/nav-items'
  import { settingsStore, updateSettingsStore } from '$lib/stores/settings'
  import type { IToggleableSettings } from '$lib/stores/settings'

  export let rawIcon: string
  export let title: string
  export let value: keyof IToggleableSettings

  $: onChange = () => updateSettingsStore({ [value]: !$settingsStore[value] })
</script>

<Card>
  <div class="flex">
    <Icon {rawIcon} />
    <p>{title}</p>
  </div>
  <SlideToggle
    size="sm"
    name={title}
    checked={$settingsStore[value]}
    on:change={onChange}
  />
</Card>
