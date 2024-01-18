<script lang="ts">
  export let images: string[] = ["/images/r.jpg", "/images/r.jpg", "/images/r.jpg", "/images/r.jpg", "/images/r.jpg"]

  export let max_height: string
  export let with_delete: boolean

  const RemoveImage = (e: Event, i: number) => {
    e.preventDefault()
    images = images.filter((_, idx) => idx != i)
  }
</script>

<section class={`${images.length && "py-2"} w-full px-5 grid grid-flow-col-dense gap-2`} style={`max-height: ${max_height};`}>
  {#each images as image, i}
    <div class="overflow-hidden relative">
      {#if with_delete}
        <button on:click={e => RemoveImage(e, i)} class="absolute font-[500] right-1 -top-1 text-2xl cursor-pointer hover:text-red-300 transition-all"
          >x</button
        >{/if}
      <img src={image} alt="layout" class="object-cover rounded-xl w-full" />
    </div>
  {/each}
</section>
