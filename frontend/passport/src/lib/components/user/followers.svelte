<script lang="ts">
  import { DevFallbackImages } from "$lib/const"
  import PeopleGroupIconRaw from "$lib/assets/people-group.svg?raw"
  import People from "./people.svelte"
</script>

<People
  raw_icon={PeopleGroupIconRaw}
  label="Followers"
  images={[DevFallbackImages.VerticalOriented, DevFallbackImages.HorizontalOriented, DevFallbackImages.HorizontalOriented]} />
