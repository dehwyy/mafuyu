<script lang="ts">
  import ImageEditor from "$lib/components/image-uploader/trigger.svelte"
  let images: string[] = []
</script>

<div class="grid place-items-center h-32">
  <ImageEditor max_images={1} bind:images _preload={true}>
    <button>123</button>
  </ImageEditor>
</div>
