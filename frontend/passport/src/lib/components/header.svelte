<script lang="ts">
  import Mafuyu from "$lib/assets/mafuyu.svg?raw"
  import { AppBar, type PopupSettings, popup, LightSwitch } from "@skeletonlabs/skeleton"


  import Account from "$lib/components/header/account.svelte"

  const settingsClick: PopupSettings = {
    event: "click",
    placement: "bottom-start",
    target: "settings-popup",
  }
</script>

<AppBar border="border-b-2 border-surface-600" padding="p-2">
  <svelte:fragment slot="lead">
    <h3 class="h3 ml-3 h-[28px]">
      {@html Mafuyu}
    </h3>
  </svelte:fragment>
  <svelte:fragment slot="trail">
    <div class="pr-5 flex gap-x-7 mt-1">
      <Account />
    </div>
  </svelte:fragment>
</AppBar>

<style>
  .menu-item {
    @apply flex gap-x-5 h-[20px] items-center;
  }
</style>
