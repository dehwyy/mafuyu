<script lang="ts">
  export let value: string
  export let label_text: string

  let is_focused = false
</script>

<label class="label relative w-full">
  <span
    class={`${
      value || is_focused
        ? '-top-[1.3rem] pt-0.5 px-0.5 opacity-70'
        : 'top-[1.1rem] opacity-50 cursor-text'
    } left-5 block absolute z-10 font-medium text-sm  rounded-t-xl transition-all duration-300`}
    >{label_text}</span
  >
  <input
    class="input px-6"
    on:focusin={() => (is_focused = true)}
    on:focusout={() => (is_focused = false)}
    type="text"
    bind:value
    {...$$restProps}
  />
</label>
