<script lang="ts">
  import { fade } from "svelte/transition"

  export let label: string
  export let images: Array<string | undefined>
  export let raw_icon: string | undefined = undefined
  export let see_all_href: string | undefined = undefined
</script>

<div class="flex gap-x-5 pointer-events-none">
  {#if raw_icon}<p class="icon-sm mt-0.5">{@html raw_icon}</p>{/if}
  <div class="flex w-full items-center">
    <p class="block w-[70px]">{label}:</p>
    <div class="ml-6 flex">
      {#each images as image}
        <div class="ml-[-0.1rem] border-2 border-surface-800 h-[30px] w-[30px] rounded-full overflow-hidden">
          {#if image}
            <img transition:fade={{ duration: 100 }} src={image} alt={label} class="object-cover bg-surface-400 h-[30px] w-[30px]" />
          {:else}
            <div class="w-[30px] h-[30px] bg-surface-600" />
          {/if}
        </div>
      {/each}
    </div>
    {#if see_all_href}<a href={see_all_href} class="block ml-auto hover:underline transition-all">see all</a>{/if}
  </div>
</div>
