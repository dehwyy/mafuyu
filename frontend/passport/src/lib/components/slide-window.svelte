<script lang="ts">
  export let max_height = "300px"

  let is_open = false
</script>

<section class="card overflow-hidden">
  <button on:click={() => (is_open = !is_open)} class="h3 mx-auto block py-5">
    <slot name="heading" />
  </button>
  <div style:max-height={max_height} class={`${is_open ? "mt-3 p-5" : "!max-h-[0px]"} grid lg:grid-cols-2 gap-5 transition-all duration-500`}>
    <slot name="content" />
  </div>
</section>
