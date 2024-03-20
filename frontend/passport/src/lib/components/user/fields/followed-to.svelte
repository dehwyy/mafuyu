<script lang="ts">
  import { derived } from "svelte/store"
  import { createQueries } from "@tanstack/svelte-query"
  import PeopleGroupIconRaw from "$lib/assets/people-group.svg?raw"
  import People from "./people.svelte"
  import { getBaseUserInfoQuery } from "$lib/query/user"
  import { useUserFollowedTo } from "$lib/query/friends"
  import { CreateNavigation } from "$lib/const"

  export let userId: string
  export let username: string

  const [userFollowers, userFollowersStore] = useUserFollowedTo(userId)
  $: userFollowersStore.set({ userId, limit: undefined })

  const followers = createQueries({
    queries: derived(userFollowers, userFollowers => {
      const aligned = new Array(3).fill(undefined).map((_, i) => userFollowers.data?.followers[i])
      return aligned.map(id => getBaseUserInfoQuery({ oneofKind: "userId", userId: id })) || []
    }),
  })
</script>

<People
  href={CreateNavigation.ToFollowed(username)}
  raw_icon={PeopleGroupIconRaw}
  label="Followed"
  images={$followers.map(follower => follower.data?.picture)} />
