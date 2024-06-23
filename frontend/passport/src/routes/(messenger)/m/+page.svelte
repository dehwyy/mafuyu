<script lang="ts">
  import { SplitPane } from '@rich_harris/svelte-split-pane'
  import Aside from '$lib/components/messenger/aside/index.svelte'
  import Content from '$lib/components/messenger/content/index.svelte'
</script>

<div>
  <SplitPane
    type="horizontal"
    id="m-main"
    priority="min"
    min="300px"
    max="400px"
    pos="350px"
  >
    <svelte:fragment slot="a">
      <Aside />
    </svelte:fragment>
    <svelte:fragment slot="b">
      <Content />
    </svelte:fragment>
  </SplitPane>
</div>

<style lang="scss">
  div {
    @apply flex h-full;
  }
</style>
