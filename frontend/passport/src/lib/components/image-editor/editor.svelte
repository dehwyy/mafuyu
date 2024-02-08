<script lang="ts">
  import { Cropper, type CropperInstance } from "svelte-cropper"
  import EditorSettings from "$lib/components/image-editor/settings.svelte"

  let cropper: CropperInstance | null = null

  export let aspect_ratio: undefined | number = 0
  export let keep_ratio = false // lock image aspect ratio
  export let OnSaveImage: (new_image: string) => void = () => {}
  export let is_fullscreen: boolean = false
  export let image: string

  const SaveImage = () => {
    if (!cropper) return

    OnSaveImage(cropper.getCroppedCanvas().toDataURL("image/jpeg"))
  }

  // if image display size (fullscreen) settings is changed, render cropper
  $: {
    ;[is_fullscreen] // deps

    if (cropper && image) {
      cropper.reset().clear().replace(image)
    }
  }

  $: {
    aspect_ratio && cropper?.setAspectRatio(aspect_ratio)
  }
</script>

<section class="transition-all overflow-hidden flex flex-col gap-y-5 items-center pb-5 border-b-2 border-b-surface-500">
  <EditorSettings bind:is_fullscreen {aspect_ratio} {keep_ratio} {cropper} />
  <Cropper
    bind:cropper
    src={image}
    style={`width: ${!is_fullscreen ? "min(600px, 90vw)" : "min(600px, 90vw)"} ;height: ${!is_fullscreen ? "calc(100vh - 350px)" : ""}`}
    cropper_props={{
      viewMode: 2,
      dragMode: "crop",
    }} />
  <button class="btn variant-filled-surface w-full lg:w-1/2" on:click={SaveImage}>Save image</button>
</section>
