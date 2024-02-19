<script lang="ts">
  import CameraIconRaw from "$lib/assets/camera.svg?raw"
  import ImageUploader from "$lib/components/image-uploader/trigger.svelte"

  let images: string[] = []

  // prettier-ignore
  export let photo: string
  $: {
    if (images.length > 0) {
      photo = images[0]
      images = []
    }
  }
</script>

<article class="w-full grid place-items-center select-none">
  <div class="wrapper">
    <ImageUploader bind:images max_images={1} keep_ratio aspect_ratio={1}>
      <div class="z-10 p-10 text-white absolute cursor-pointer">
        {@html CameraIconRaw}
      </div>
    </ImageUploader>
    <img src={photo} alt="profile" class="profile-image" />
  </div>
</article>

<style lang="scss">
  .wrapper {
    @apply w-32 h-32 object-cover rounded-full relative overflow-hidden flex;
    @apply after:bg-black after:opacity-40 after:w-full after:h-full after:absolute;
  }
  .profile-image {
    @apply w-full h-full absolute top-0 left-0 object-cover;
    -webkit-user-drag: none;
  }
</style>
