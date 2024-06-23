<script lang="ts">
  import { fade } from 'svelte/transition'

  export let label: string
  export let images: Array<string | null>
  export let isLoading = true
  export let raw_icon: string | undefined = undefined
  export let href: string = '#'
  export let onClick: () => void = () => {}

  $: aligned_images = new Array(3)
    .fill(undefined)
    .map((_, i) => images[i] || undefined)
  $: isNotLoading = typeof window === 'undefined' ? false : !isLoading
  $: hasAny = !isNotLoading || images.length !== 0
</script>

<a
  {href}
  on:click={onClick}
  class={`${
    hasAny ? 'max-h-[30px]' : 'max-h-[0]'
  } overflow-hidden transition-all cursor-pointer`}
>
  <div class="flex gap-x-5 pointer-events-none h-[30px]">
    {#if raw_icon}<p class="icon-sm mt-0.5">{@html raw_icon}</p>{/if}
    <div class="flex w-full items-center">
      <p class="block w-[100px]">{label}:</p>
      <div class="ml-2 flex">
        {#each aligned_images as image, i}
          <div
            class="ml-[-0.1rem] border-2 border-surface-800 h-[30px] w-[30px] rounded-full overflow-hidden"
          >
            {#if image}
              <img
                in:fade={{ duration: 200 }}
                loading="lazy"
                height="30"
                width="30"
                src={image}
                alt={label}
                class="object-cover bg-surface-400 max-h-[30px] max-w-[30px] object-center relative left-[-0.05rem]"
              />
            {:else if images[i] === undefined && isNotLoading}
              <!-- transparent -->
              <div
                in:fade={{ duration: 100 }}
                class="w-[30px] h-[30px] bg-transparent"
              ></div>
            {:else}
              <div
                transition:fade={{ duration: 100 }}
                class="w-[30px] h-[30px] bg-surface-700"
              ></div>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  </div>
</a>
