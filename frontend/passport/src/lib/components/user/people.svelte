<script lang="ts">
  export let label: string
  export let images: string[]
  export let raw_icon: string | undefined = undefined
  export let see_all_href: string | undefined = undefined

  $: images_to_render = images.length > 3 ? images.slice(0, 3) : images
</script>

<div class="flex gap-x-5">
  {#if raw_icon}<p class="icon-sm">{@html raw_icon}</p>{/if}
  <p class="flex w-full items-center">
    <span class="block w-[70px]">{label}:</span>
    <span class="ml-6 flex">
      {#each images_to_render as image}
        <span class="block ml-[-0.25rem] border-2 border-surface-100 dark:border-surface-800 h-[30px] w-[30px] rounded-full overflow-hidden">
          <img src={image} alt={label} class="min-h-full min-w-screen object-cover" />
        </span>
      {/each}
    </span>
    {#if see_all_href}<a href={see_all_href} class="block ml-auto hover:underline transition-all">see all</a>{/if}
  </p>
</div>
