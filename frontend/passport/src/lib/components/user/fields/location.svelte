<script lang="ts">
  import LocationIconRaw from "$lib/assets/location.svg?raw"

  export let location: string | undefined
</script>

{#if location}
  <div class="flex gap-x-5">
    <p class="icon-sm">
      {@html LocationIconRaw}
    </p>
    <p>Location: &nbsp;<span class="text-primary-500 underline">{location}</span></p>
  </div>
{/if}
