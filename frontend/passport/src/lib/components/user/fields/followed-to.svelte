<script lang="ts">
  import { derived } from "svelte/store"
  import { createQueries } from "@tanstack/svelte-query"
  import PeopleGroupIconRaw from "$lib/assets/people-group.svg?raw"
  import People from "./people.svelte"
  import { getBaseUserInfoQuery } from "$lib/query/user"
  import { useUserFollowedTo } from "$lib/query/friends"
  import { updatePersistentDataStore, CommunitySection } from "$lib/stores/nav"
  import { CreateNavigation } from "$lib/const"

  export let userId: string
  export let username: string

  const [userFollowers, userFollowersStore] = useUserFollowedTo(userId)
  $: userFollowersStore.set({ userId, limit: undefined })

  const followers = createQueries({
    queries: derived(userFollowers, userFollowers => {
      const followers = userFollowers.data?.followers || []

      return followers.map(id => getBaseUserInfoQuery({ oneofKind: "userId", userId: id })) || []
    }),
  })
</script>

<People
  href={CreateNavigation.ToCommunity(username)}
  onClick={() => updatePersistentDataStore({ communitySection: CommunitySection.FOLLOWED })}
  raw_icon={PeopleGroupIconRaw}
  label="Followed"
  isLoading={$userFollowers.isLoading}
  images={$followers.map(follower => follower.data?.picture || null)} />
