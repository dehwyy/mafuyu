<script lang="ts">
  import { fade } from "svelte/transition"
  let cssClass = ""

  export { cssClass as class }
</script>

<div in:fade={{ duration: 100 }} class={`${cssClass} gradient-border !card`} {...$$restProps}>
  <slot />
</div>

<style lang="scss">
  $border-width: 3px;

  .gradient-border {
    max-width: 90vw;
    width: 100%;
    position: relative;
  }

  .gradient-border::after {
    position: absolute;
    content: "";
    top: calc(-1 * $border-width);
    left: calc(-1 * $border-width);
    z-index: -1;
    width: calc(100% + $border-width * 2);
    height: calc(100% + $border-width * 2);
    background: linear-gradient(
      60deg,
      hsl(224, 85%, 66%),
      hsl(269, 85%, 66%),
      hsl(314, 85%, 66%),
      hsl(359, 85%, 66%),
      hsl(44, 85%, 66%),
      hsl(89, 85%, 66%),
      hsl(134, 85%, 66%),
      hsl(179, 85%, 66%)
    );
    background-size: 300% 300%;
    background-position: 0 50%;
    border-radius: calc(2 * $border-width);
    animation: moveGradient 4s alternate infinite;
  }

  @keyframes moveGradient {
    50% {
      background-position: 100% 50%;
    }
  }
</style>
