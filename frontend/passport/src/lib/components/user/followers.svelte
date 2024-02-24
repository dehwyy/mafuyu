<script lang="ts">
	import { derived } from 'svelte/store';
	import { createQueries } from '@tanstack/svelte-query';
  import PeopleGroupIconRaw from "$lib/assets/people-group.svg?raw"
  import People from "./people.svelte"
  import { getBaseUserInfoQuery, useUserInfo } from "$lib/query/user"
  import { page } from "$app/stores"
  import { useUserFollowers } from "$lib/query/friends"

  const [user, userStore] = useUserInfo({ oneofKind: "username", username: $page.params.username })
  $: userStore.set({ getBy: { oneofKind: "username", username: $page.params.username } })

  const [userFollowers, userFollowersStore] = useUserFollowers($user?.data?.userId)
  $: userFollowersStore.set({ userId: $user?.data?.userId || "", limit: undefined })

  const followers = createQueries({
    queries: derived(userFollowers, userFollowers => {
      return userFollowers.data?.followers.map(id => getBaseUserInfoQuery({ oneofKind: "userId", userId: id })) || []
    })
  })

</script>

<People raw_icon={PeopleGroupIconRaw} label="Followers" images={$followers.map(follower => follower.data?.picture)} />
