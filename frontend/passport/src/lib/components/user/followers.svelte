<script lang="ts">
  import { derived } from "svelte/store"
  import { createQueries } from "@tanstack/svelte-query"
  import PeopleGroupIconRaw from "$lib/assets/people-group.svg?raw"
  import People from "./people.svelte"
  import { getBaseUserInfoQuery} from "$lib/query/user"
  import { useUserFollowers } from "$lib/query/friends"

  export let userId: string

  const [userFollowers, userFollowersStore] = useUserFollowers(userId)
  $: userFollowersStore.set({ userId, limit: undefined })

  const followers = createQueries({
    queries: derived(userFollowers, userFollowers => {
      const aligned = new Array(3).fill(undefined).map((_, i) => userFollowers.data?.followers[i])
      return aligned.map(id => getBaseUserInfoQuery({ oneofKind: "userId", userId: id })) || []
    }),
  })
</script>

<People raw_icon={PeopleGroupIconRaw} label="Followers" images={$followers.map(follower => follower.data?.picture)} />
