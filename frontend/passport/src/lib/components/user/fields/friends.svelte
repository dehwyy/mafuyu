<script lang="ts">
  import FriendsIconRaw from "$lib/assets/people.svg?raw"
  import People from "./people.svelte"
  import { derived } from "svelte/store"
  import { getBaseUserInfoQuery } from "$lib/query/user"
  import { useUserFriends } from "$lib/query/friends"
  import { createQueries } from "@tanstack/svelte-query"
  import { updatePersistentDataStore, CommunitySection } from "$lib/stores/nav"
  import { CreateNavigation } from "$lib/const"

  export let userId: string
  export let username: string

  const [userFriends, userFriendsStore] = useUserFriends(userId, undefined)
  $: userFriendsStore.set({ userId, limit: undefined })

  const friends = createQueries({
    queries: derived(userFriends, userFriends => {
      const friends = userFriends.data?.friends || []

      return friends.map(id => getBaseUserInfoQuery({ oneofKind: "userId", userId: id })) || []
    }),
  })
</script>

<People
  href={CreateNavigation.ToCommunity(username)}
  onClick={() => updatePersistentDataStore({ communitySection: CommunitySection.FRIENDS })}
  raw_icon={FriendsIconRaw}
  label="Friends"
  isLoading={$userFriends.isLoading}
  images={$friends.map(friend => friend.data?.picture || null)} />
