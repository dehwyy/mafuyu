<script lang="ts">
  import { type PopupSettings, popup } from "@skeletonlabs/skeleton"

  export let event: "click" | "hover" = "click"
  export let unique_target_name: string
  export let placement: "bottom" | "top" | "left" | "right" = "top"
  export let options: { icon: string; text: string; action?: () => void }[]

  $: options_popup_settings = {
    target: unique_target_name,
    event,
    placement,
  }
</script>

<section use:popup={options_popup_settings}>
  <slot />
</section>

<div data-popup={unique_target_name} class="card border border-surface-600">
  <ul class="list options-popup py-2 px-1 select-none">
    {#each options as opt}
      <li
        aria-hidden="true"
        on:click={() => (opt.action ? opt.action() : {})}
        class="px-5 py-2 cursor-pointer dark:hover:bg-surface-700 hover:surface-200 transition-all">
        <div class="icon-sm">
          {@html opt.icon}
        </div>
        <p class="font-[600]">{opt.text}</p>
      </li>
    {/each}
  </ul>
</div>
