<script lang="ts">
  import InputWithLabel from "$lib/components/form/input.svelte"
  export let location = ""
</script>

<article class="w-full">
  <InputWithLabel bind:value={location} label_text="location" />
</article>
