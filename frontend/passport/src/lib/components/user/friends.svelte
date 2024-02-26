<script lang="ts">
  import FriendsIconRaw from "$lib/assets/people.svg?raw"
  import People from "./people.svelte"
  import { derived } from "svelte/store"
  import { getBaseUserInfoQuery } from "$lib/query/user"
  import { useUserFriends } from "$lib/query/friends"
  import { createQueries } from "@tanstack/svelte-query"
  import { CreateNavigation } from "$lib/const"

  export let userId: string
  export let username: string

  const [userFriends, userFriendsStore] = useUserFriends(userId, undefined)
  $: userFriendsStore.set({ userId, limit: undefined })

  const friends = createQueries({
    queries: derived(userFriends, userFriends => {
      const aligned = new Array(3).fill(undefined).map((_, i) => userFriends.data?.friends[i])
      return aligned.map(id => getBaseUserInfoQuery({ oneofKind: "userId", userId: id })) || []
    }),
  })
</script>

<People href={CreateNavigation.ToFriends(username)} raw_icon={FriendsIconRaw} label="Friends" images={$friends.map(friend => friend.data?.picture)} />
