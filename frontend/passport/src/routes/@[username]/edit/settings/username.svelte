<script lang="ts">
  import InputWithLabel from "$lib/components/form/input.svelte"
  export let username = ""
</script>

<article class="w-full">
  <InputWithLabel bind:value={username} label_text="Username" />
</article>
