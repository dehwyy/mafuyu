<script lang="ts">
  import { settingsStore } from '$lib/stores/settings'
  import { onMount } from 'svelte'
  import { fade } from 'svelte/transition'

  let cssClass = ''

  export { cssClass as class }

  let isRgbEnabled = false
  export let disabledRgb = false
  onMount(() => {
    if (!disabledRgb) {
      isRgbEnabled = $settingsStore.rgbCard
      settingsStore.subscribe((v) => {
        isRgbEnabled = v.rgbCard
      })
    }
  })
</script>

<div
  in:fade={{ duration: 500 }}
  class={`${cssClass} ${
    isRgbEnabled || 'off'
  } gradient-border !card after:transition-all`}
  {...$$restProps}
>
  <slot />
</div>

<style lang="scss">
  $border-width: calc(var(--theme-border-base) * 1.6);

  .gradient-border {
    max-width: 90vw;
    width: 100%;
    position: relative;
  }

  .gradient-border::after {
    position: absolute;
    content: '';
    top: calc(-0.5 * $border-width);
    left: calc(-0.5 * $border-width);
    z-index: -1;
    width: calc(100% + $border-width * 1);
    height: calc(100% + $border-width * 1);
    background: linear-gradient(
      60deg,
      hsla(224, 85%, 66%, 1),
      hsla(269, 85%, 66%, 1),
      hsla(314, 85%, 66%, 1),
      hsla(359, 85%, 66%, 1),
      hsla(44, 85%, 66%, 1),
      hsla(89, 85%, 66%, 1),
      hsla(134, 85%, 66%, 1),
      hsla(179, 85%, 66%, 1)
    );
    background-size: 300% 300%;
    background-position: 0 50%;
    animation: moveGradient 4s alternate infinite;
    border-radius: var(--theme-rounded-container);
  }

  .off::after {
    background: transparent;
  }

  @keyframes moveGradient {
    50% {
      background-position: 100% 50%;
    }
  }
</style>
