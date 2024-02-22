<script lang="ts">
  import { Birthday, Location, Friends, Followers, Languages } from "$lib/components/user/index"
  import { useUserProfile } from "$lib/query/profile"
  import { page } from "$app/stores"

  const user = useUserProfile($page.params.username)
</script>

<section class="flex flex-col gap-y-4 pt-6">
  <section class="card p-6 w-full flex flex-col gap-y-4">
    <Birthday />
    <Location location={$user.data?.location} />
    <hr />
    <Friends />
    <Followers />
    <hr />
    <Languages languages={$user.data?.languages || []} />
  </section>
</section>
