<script lang="ts">
  import { getModalStore } from '@skeletonlabs/skeleton'
  import type { ModalSettings } from '@skeletonlabs/skeleton'

  import ImageUploaderComponent from './modal.svelte'

  export let images: string[]
  export let max_images: number = 5
  export let aspect_ratio: number = 0
  export let keep_ratio: boolean = false

  const modal_store = getModalStore()

  const image_uploader_modal_settings: ModalSettings = {
    title: 'ImageUploader',
    type: 'component',
    component: { ref: ImageUploaderComponent },
    meta: {
      max_images,
      aspect_ratio,
      keep_ratio
    },
    response: (v: string[] | undefined) => {
      if (v && v.length + images.length <= max_images)
        images = [...images, ...v]
    }
  }
</script>

<section
  aria-hidden="true"
  on:click={() =>
    modal_store.set([image_uploader_modal_settings, ...$modal_store])}
>
  <slot />
</section>
