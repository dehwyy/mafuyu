<script lang="ts">
  import MafuyuIcon from "$lib/assets/mafuyu.svg?raw"
  import { AppBar } from "@skeletonlabs/skeleton"

  import Navigation from "$lib/components/header/navigation.svelte"
</script>

<AppBar border="border-b-2 border-surface-600" padding="p-2">
  <svelte:fragment slot="lead">
    <h3 class="h3 ml-3 h-[28px]">
      {@html MafuyuIcon}
    </h3>
  </svelte:fragment>
  <svelte:fragment slot="trail">
    <div class="pr-5 flex gap-x-7 mt-1">
      <Navigation />
    </div>
  </svelte:fragment>
</AppBar>
