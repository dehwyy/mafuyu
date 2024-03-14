<script lang="ts">
  import InputWithLabel from "$lib/components/form/input.svelte"
  export let pseudonym = ""
</script>

<article class="w-full">
  <InputWithLabel bind:value={pseudonym} label_text="Alternative name" />
</article>
