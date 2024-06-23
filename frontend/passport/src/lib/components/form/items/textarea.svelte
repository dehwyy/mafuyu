<script lang="ts">
  export let padding = '1.5rem'
  export let placeholder: string
  export let text: string
  export let is_focused = false
</script>

<svelte:window on:click={() => (is_focused = false)} />
<section
  class="relative"
  on:click|stopPropagation={() => (is_focused = true)}
  aria-hidden="true"
>
  <div
    style={`top: ${padding};left: ${padding}`}
    class={`${text && 'hidden'} ${
      !is_focused && 'cursor-text'
    } absolute top-6 left-6 opacity-60 select-none pointer-events-none`}
  >
    {placeholder}
  </div>
  <div
    style={`padding:${padding}`}
    class={`writer-input !outline-none`}
    bind:innerText={text}
    contenteditable
  />
</section>
