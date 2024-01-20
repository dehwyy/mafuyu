<script lang="ts">
  import CameraIconRaw from "$lib/assets/camera.svg?raw"
  import ImageUploader from "$lib/components/image-uploader/trigger.svelte"

  let images: string[] = []

  // prettier-ignore
  export let photo = "https://sun9-28.userapi.com/impg/nfm26MjdU4tRW3N-OwRaiLh996CjCTLh0vu8Dg/ENC1jP7-KJw.jpg?size=1347x1346&quality=95&sign=ed76ada3e9318d6d216d6b845421f402&type=album"
  $: {
    if (images.length > 0) {
      photo = images[0]
      images = []
    }
  }
</script>

<article class="w-full grid place-items-center select-none">
  <div class="wrapper">
    <ImageUploader bind:images max_images={1}>
      <div class="z-10 p-10 text-white absolute cursor-pointer">
        {@html CameraIconRaw}
      </div>
    </ImageUploader>
    <img src={photo} alt="profile" class="profile-image" />
  </div>
</article>

<style>
  .wrapper {
    @apply w-32 h-32 object-cover rounded-full relative overflow-hidden flex;
    @apply after:bg-black after:opacity-40 after:w-full after:h-full after:absolute;
  }
  .profile-image {
    @apply w-full h-full absolute top-0 left-0;
    -webkit-user-drag: none;
  }
</style>
