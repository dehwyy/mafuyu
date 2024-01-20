<script lang="ts">
  import CameraIcon from "svelte-icons/fa/FaCamera.svelte"
  import ImageUploader from "./image-uploader/trigger.svelte"
  import AdaptiveImageGrid from "$lib/components/adaptive-image-grid.svelte"
  import Textarea from "$lib/components/form/textarea.svelte"
  export let button_text: string
  export let placeholder: string
  export let padding: string = "1.5rem"

  let text = ""
  let images: string[] = []
  let is_focused = false
</script>

<div aria-hidden="true" class="w-full writer overflow-hidden focus-visible:!outline-none">
  <Textarea bind:text {placeholder} bind:is_focused {padding} />
  <!--  <div-->
  <!--    aria-hidden="true"-->
  <!--    style={`top: ${padding};left: ${padding}`}-->
  <!--    class={`${text && "hidden"} ${!is_focused && "cursor-text"} absolute top-6 left-6 opacity-60 select-none pointer-events-none`}>-->
  <!--    {placeholder}-->
  <!--  </div>-->
  <!--  <div style={`padding:${padding}`} class={`writer-input !outline-none`} bind:innerText={text} contenteditable />-->
  <AdaptiveImageGrid max_height="600px" bind:images with_delete />
  <div class={`${is_focused || text || images.length ? "max-h-[100px]" : "max-h-0"} transition-all ease-linear duration-300 `}>
    <hr class="mb-4 mx-5" />
    <div class="w-full flex items-center pl-8 pr-6 pb-4">
      <div class="">
        <ImageUploader bind:images max_images={10}>
          <button class="icon-sm">
            <CameraIcon />
          </button>
        </ImageUploader>
      </div>
      <button disabled={!(text || images.length)} class="text-sm ml-auto block btn variant-filled-surface select-none">{button_text}</button>
    </div>
  </div>
</div>
