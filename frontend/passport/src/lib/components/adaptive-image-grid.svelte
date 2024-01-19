<script lang="ts">
  import CloseIconRaw from "$lib/assets/close.svg?raw"
  export let images: string[]

  export let max_height: string
  export let with_delete: boolean

  const RemoveImage = (e: Event, i: number) => {
    e.preventDefault()
    images = images.filter((_, idx) => idx != i)
  }

  $: grid_rows = Math.floor(images.length / 4) + 1
</script>

<section
  style:max-height={max_height}
  style:grid-template-rows={`repeat(${grid_rows}, minmax(0, 1fr))`}
  class:py-2={images.length}
  class={`w-full px-5 grid grid-flow-col-dense gap-2`}>
  {#each images as image, i}
    <div class="overflow-hidden relative">
      {#if with_delete}
        <button
          on:click={e => RemoveImage(e, i)}
          class="absolute right-2 top-2 rounded-full font-[500] bg-surface-200 dark:bg-surface-800 dark:hover:bg-transparent hover:bg-transparent cursor-pointer hover:text-red-400 transition-all">
          {@html CloseIconRaw}
        </button>{/if}
      <img src={image} alt="layout" class="object-cover rounded-xl w-full" />
    </div>
  {/each}
</section>
