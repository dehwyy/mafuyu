<script lang="ts">
  import ArrowsHorizontalRaw from '$lib/assets/arrows-horizontal.svg?raw'
  import ArrowsVerticalRaw from '$lib/assets/arrows-vertical.svg?raw'
  import FullscreenIconRaw from '$lib/assets/fullscreen.svg?raw'
  import type { CropperInstance } from 'svelte-cropper'

  export let is_fullscreen = false
  export let aspect_ratio: number | undefined
  export let keep_ratio: boolean
  export let cropper: CropperInstance | null = null

  let scale_x = 1
  let scale_y = 1

  const SetAspectRatio = (ratio: number | undefined) => {
    aspect_ratio = ratio
    cropper?.setAspectRatio(ratio || 0)
  }

  const MirrorHorizontal = () => {
    if (scale_x == 1) {
      cropper?.scaleX(-1)
      scale_x = -1
    } else {
      cropper?.scaleX(1)
      scale_x = 1
    }
  }

  const MirrorVertical = () => {
    if (scale_y == 1) {
      cropper?.scaleY(-1)
      scale_y = -1
    } else {
      cropper?.scaleY(1)
      scale_y = 1
    }
  }
</script>

<ul class="settings">
  <li class="settings-icon">
    <button
      on:click={() => (is_fullscreen = !is_fullscreen)}
      class:opacity-30={!is_fullscreen}
    >
      {@html FullscreenIconRaw}
    </button>
  </li>
  {#if !keep_ratio}
    <li class="flex gap-x-3">
      <button
        on:click={() => SetAspectRatio(16 / 9)}
        class:opacity-30={aspect_ratio !== 16 / 9}>16:9</button
      >
      <button
        on:click={() => SetAspectRatio(1)}
        class:opacity-30={aspect_ratio !== 1}>1:1</button
      >
      <button
        on:click={() => SetAspectRatio(undefined)}
        class:opacity-30={aspect_ratio !== undefined}>Free</button
      >
    </li>
  {/if}
  <li class="">
    <button
      on:click={MirrorVertical}
      class="settings-button-icon"
    >
      {@html ArrowsVerticalRaw}
    </button>
    <button
      on:click={MirrorHorizontal}
      class="settings-button-icon"
    >
      {@html ArrowsHorizontalRaw}
    </button>
  </li>
</ul>

<style lang="scss">
  .settings {
    @apply flex items-center gap-x-10 w-full px-5;
    & li {
      & button {
        outline: none !important;
        @apply transition-all;
      }
    }
    &-icon {
      @apply transition-all w-[20px] h-[20px];
    }
    &-button-icon {
      @apply btn w-[30px] h-[30px] p-[5px] rounded-full variant-glass-secondary;
    }
  }
</style>
