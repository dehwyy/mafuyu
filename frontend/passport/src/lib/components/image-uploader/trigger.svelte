<script lang="ts">
  import { getModalStore, type ModalSettings } from "@skeletonlabs/skeleton"
  import ImageUploaderComponent from "./modal.svelte"
  import { onMount } from "svelte"

  export let _preload = false

  export let images: string[]
  export const max_images: number = 5

  const modal_store = getModalStore()

  const image_uploader_modal_settings: ModalSettings = {
    title: "ImageUploader",
    type: "component",
    component: { ref: ImageUploaderComponent },
    meta: {
      max_images,
    },
    response: (v: string[] | undefined) => {
      if (v && v.length + images.length <= max_images) images = [...images, ...v]
    },
  }
  onMount(() => {
    if (_preload) {
      modal_store.set([image_uploader_modal_settings, ...$modal_store])
    }
  })
</script>

<section aria-hidden="true" on:click={() => modal_store.set([image_uploader_modal_settings, ...$modal_store])}>
  <slot />
</section>
