<script lang="ts">
  import FriendsIconRaw from "$lib/assets/people.svg?raw"
  import People from "./people.svelte"
  import { getBaseUserInfoQuery, useUserInfo } from "$lib/query/user"
  import { page } from "$app/stores"
  import { useUserFriends } from "$lib/query/friends"
  import { derived } from "svelte/store"
  import { createQueries } from "@tanstack/svelte-query"

  const [user, userStore] = useUserInfo({ oneofKind: "username", username: $page.params.username })
  $: userStore.set({ getBy: { oneofKind: "username", username: $page.params.username } })

  const [userFriends, userFriendsStore] = useUserFriends($user?.data?.userId, 3)
  $: userFriendsStore.set({ userId: $user?.data?.userId || "", limit: 3 })

  const friends = createQueries({
    queries: derived(userFriends, userFriends => {
      return userFriends.data?.friends.map(id => getBaseUserInfoQuery({ oneofKind: "userId", userId: id })) || []
    })
  })
</script>

<People raw_icon={FriendsIconRaw} label="Friends" images={$friends.map(friend => friend.data?.picture)} />
