<script lang="ts">
  import { FileButton, getModalStore } from "@skeletonlabs/skeleton"
  import ImageEditor from "$lib/components/image-editor/editor.svelte"
  import { FileReaderTool } from "$lib/utils/file_reader"

  const modal_store = getModalStore()
  $: MaxImages = $modal_store[0]?.meta["max_images"] || 10
  $: AspectRatio = $modal_store[0]?.meta["aspect_ratio"]
  $: KeepRatio = $modal_store[0]?.meta["keep_ratio"] ?? false

  let selected_file_images: FileList

  let unprocessed_images: string[] = []
  let processed_images: string[] = []
  let current_image: string | null = null

  const OnFileSelect = async (e: Event) => {
    const input = e.target as HTMLInputElement
    if (!input.files) {
      console.log("no files provided")
      return
    }

    const files_count = input.files.length
    const loaded_images: string[] = new Array(files_count) // base64[]
    for (let i = 0; i < files_count; i++) {
      const image_to_load = input.files.item(i)! // should exist
      // image_to_load.size > 10 * 8 // TODO: add size
      // ["image/jpeg", "image/png"].includes(image_to_load.type) // TODO: add .ext validation

      loaded_images[files_count - i - 1] = await FileReaderTool.AsBase64(image_to_load)
    }

    //
    if (loaded_images.length + processed_images.length <= MaxImages) {
      unprocessed_images = current_image ? [...unprocessed_images, current_image, ...loaded_images] : [...unprocessed_images, ...loaded_images]
      current_image = unprocessed_images.pop()!
    }
  }

  const SaveImage = (image: string) => {
    // get modal
    const modal = $modal_store[0]
    if (!modal || !modal.response) return

    // add images to processed
    const new_processed_images = [...processed_images, image]

    // if there isn't any unprocessed image AND the maximum images limit is not reached yet
    // => assign variables and continue processing
    if (unprocessed_images.length > 0 && new_processed_images.length !== MaxImages) {
      processed_images = new_processed_images
      current_image = unprocessed_images.pop()!
      return
    }

    // send response to modal_handler
    modal.response(new_processed_images)
    modal_store.close()
  }
</script>

<section class="card p-6 flex flex-col gap-y-5">
  <h4 class="h4">Choose photo</h4>
  {#if current_image}
    <ImageEditor aspect_ratio={AspectRatio} keep_ratio={KeepRatio} bind:image={current_image} OnSaveImage={new_image => SaveImage(new_image)} />
  {/if}
  <FileButton
    multiple={MaxImages !== 1}
    button={`btn variant-soft-primary w-full mx-auto block ${current_image ? "lg:w-1/2" : ""}`}
    name="image-uploader"
    bind:files={selected_file_images}
    on:change={OnFileSelect} />
</section>
