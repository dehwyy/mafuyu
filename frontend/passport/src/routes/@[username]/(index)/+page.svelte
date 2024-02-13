<script lang="ts">
  import { Birthday, Location, Friends, Followers, Languages } from "$lib/components/user/index"

  import type { LayoutData } from "../$types"

  export let data: LayoutData
</script>

<section class="flex flex-col gap-y-4 pt-6">
  <section class="card p-6 w-full flex flex-col gap-y-4">
    <Birthday />
    <Location location={data.location} />
    <hr />
    <Friends />
    <Followers />
    <hr />
    <Languages languages={data.languages} />
  </section>
</section>
