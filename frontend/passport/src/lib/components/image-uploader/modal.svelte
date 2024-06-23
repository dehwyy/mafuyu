<script lang="ts">
  import { FileButton, getModalStore } from '@skeletonlabs/skeleton'
  import ImageEditor from '$lib/components/image-editor/editor.svelte'
  import { FileReaderTool } from '$lib/utils/file_reader'

  interface Image {
    data: string
    type: 'image/webp' | 'image/gif'
  }

  const modal_store = getModalStore()
  $: MaxImages = $modal_store[0]?.meta['max_images'] || 10
  $: AspectRatio = $modal_store[0]?.meta['aspect_ratio']
  $: KeepRatio = $modal_store[0]?.meta['keep_ratio'] ?? false

  let selected_file_images: FileList

  let unprocessed_images: Image[] = []
  let processed_images: Image[] = []
  let current_image: Image | null = null

  const OnFileSelect = async (e: Event) => {
    const input = e.target as HTMLInputElement
    if (!input.files) {
      console.log('no files provided')
      return
    }

    const files_count = input.files.length
    const loaded_images: Array<Image | undefined> = new Array(files_count).fill(
      undefined
    ) // base64[]
    for (let i = 0; i < files_count; i++) {
      const image_to_load = input.files.item(i)! // should exist
      console.log(image_to_load)
      // image_to_load.size > 10 * 8 // TODO: add size

      if (
        ['image/jpeg', 'image/png', 'image/webp', 'image/gif'].includes(
          image_to_load.type
        )
      ) {
        loaded_images[files_count - i - 1] = {
          data: await FileReaderTool.AsBase64(image_to_load),
          type: image_to_load.type === 'image/gif' ? 'image/gif' : 'image/webp'
        }
      }
    }

    const images = loaded_images.filter((v) => v !== undefined) as Image[]

    //
    if (loaded_images.length + processed_images.length <= MaxImages) {
      unprocessed_images = current_image
        ? [...unprocessed_images, current_image, ...images]
        : [...unprocessed_images, ...images]
      current_image = unprocessed_images.pop()!
    }
  }

  const SaveImage = (image: Image) => {
    // get modal
    const modal = $modal_store[0]
    if (!modal || !modal.response) return

    // add images to processed
    const new_processed_images = [...processed_images, image]

    // if there isn't any unprocessed image AND the maximum images limit is not reached yet
    // => assign variables and continue processing
    if (
      unprocessed_images.length > 0 &&
      new_processed_images.length !== MaxImages
    ) {
      processed_images = new_processed_images
      current_image = unprocessed_images.pop()!
      return
    }

    // send response to modal_handler
    modal.response(new_processed_images.map((v) => v.data))
    modal_store.close()
  }

  $: OnSaveImage = (type_callback: (type: string) => string) => {
    if (!current_image) return

    SaveImage({
      data: type_callback(current_image.type),
      type: current_image.type
    })
  }
</script>

<section class="card p-6 flex flex-col gap-y-5">
  <h4 class="h4">Choose photo</h4>
  {#if current_image}
    <ImageEditor
      aspect_ratio={AspectRatio}
      keep_ratio={KeepRatio}
      bind:image={current_image.data}
      isGif={current_image.type === 'image/gif'}
      OnSaveImage={(callback) => OnSaveImage(callback)}
    />
  {/if}
  <FileButton
    multiple={MaxImages !== 1}
    button={`btn variant-soft-primary w-full mx-auto block ${
      current_image ? 'lg:w-1/2' : ''
    }`}
    name="image-uploader"
    bind:files={selected_file_images}
    on:change={OnFileSelect}
  />
</section>
