<script lang="ts">
  import ShevronIconRaw from "$lib/assets/chevron.svg?raw"
  import { type Node, flattenNodes } from "./utils"

  const nodeHeight = 64

  export let nodes: Node[]

  let sectionHeight = 0
  let isOpen = false

  $: flattenNodesLenght = flattenNodes(nodes).length
  $: maxHeight = (flattenNodesLenght + 2) * nodeHeight + "px"
  $: openable = !!nodes.length
  $: lineHeight = Math.max(sectionHeight - 40, 0)
</script>

<section bind:clientHeight={sectionHeight} class="relative">
  <button class="p-5 flex gap-x-2" class:cursor-default={!openable} on:click={() => openable && (isOpen = !isOpen)}>
    <svg height="24px" viewBox="0 0 24 24" width="40px">
      <line stroke="#fff" x1="0" y1="13" x2="40" y2="13" />
      <line stroke-width="6" stroke="#fff" x1="27" y1="13" x2="33" y2="13" />
    </svg>
    <p class="whitespace-nowrap"><slot /></p>
    {#if openable}
      <div class={`${isOpen ? "rotate-180" : "rotate-0"} icon-sm transition-all`}>{@html ShevronIconRaw}</div>
    {/if}
  </button>
  <div class="">
    {#if openable}
      <div class:opacity-0={!isOpen} class="absolute top-[33px] left-[57.5px] z-10 transition-all">
        <svg height={lineHeight} width="24px">
          <line stroke="#fff" x1="0" y1="0" x2="0" y2={lineHeight} />
        </svg>
      </div>
      <div
        style:max-height={maxHeight}
        class={`${isOpen ? "" : "!max-h-[0px]"} px-10 flex flex-col gap-y-5 transition-all duration-500 ease-in-out overflow-hidden`}>
        {#each nodes as node}
          <svelte:self nodes={node.children}>
            {node.h}
          </svelte:self>
        {/each}
      </div>
    {/if}
  </div>
</section>
