<script lang="ts">
  let extraCssClass = ''
  export let closeAfterClick = true

  export { extraCssClass as class }
</script>

{#if closeAfterClick}
  <button
    class={extraCssClass}
    {...$$restProps}
  >
    <slot />
  </button>
{:else}
  <div
    class={extraCssClass}
    {...$$restProps}
  >
    <slot />
  </div>
{/if}

<style lang="scss">
  button,
  div {
    @apply max-h-[40px] btn hover:bg-surface-300/10 text-surface-200 hover:text-white w-full justify-start pl-4 font-medium;
  }
</style>
