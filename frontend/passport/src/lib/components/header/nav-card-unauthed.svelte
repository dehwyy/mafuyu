<script lang="ts">
  import { Routes } from "$lib/const"
  import ThemeSelector from "./custom-settings/theme-selector.svelte"

  import LoginIconRaw from "$lib/assets/user.svg?raw"
  import ThemeSelectIconRaw from "$lib/assets/theme-select.svg?raw"
</script>

<ul class="mt-5">
  <li>
    <a href={Routes.Login}>
      <button>
        <span>{@html LoginIconRaw}</span>
        <span>Log in</span>
      </button>
    </a>
  </li>
  <li>
    <div>
      <ThemeSelector>
        <span class="icon-sm block">{@html ThemeSelectIconRaw}</span>
      </ThemeSelector>
    </div>
  </li>
</ul>

<style lang="scss">
  ul li button {
    @apply max-h-[40px] btn hover:bg-surface-300/10 text-surface-200 hover:text-white w-full justify-start pl-4 font-medium;
    & span:first-child {
      @apply h-[24px] mr-2;
    }
  }
</style>
