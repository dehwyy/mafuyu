<script lang="ts">
  import { DevFallbackImages } from "$lib/const"
  import FriendsIconRaw from "$lib/assets/people.svg?raw"
  import People from "./people.svelte"
</script>

<People
  raw_icon={FriendsIconRaw}
  label="Friends"
  images={[DevFallbackImages.VerticalOriented, DevFallbackImages.HorizontalOriented, DevFallbackImages.HorizontalOriented]} />
