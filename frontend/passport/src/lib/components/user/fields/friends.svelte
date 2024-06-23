<script lang="ts">
  import { createQueries } from '@tanstack/svelte-query'
  import FriendsIconRaw from '$lib/assets/people.svg?raw'
  import { CreateNavigation } from '$lib/const'
  import { useUserFriends } from '$lib/query/friends'
  import { getBaseUserInfoQuery } from '$lib/query/user'
  import { CommunitySection, updatePersistentDataStore } from '$lib/stores/nav'
  import { derived } from 'svelte/store'

  import People from './people.svelte'

  export let userId: string
  export let username: string

  const [userFriends, userFriendsStore] = useUserFriends(userId, undefined)
  $: userFriendsStore.set({ userId, limit: undefined })

  const friends = createQueries({
    queries: derived(userFriends, (userFriends) => {
      const friends = userFriends.data?.friends || []

      return (
        friends.map((id) =>
          getBaseUserInfoQuery({ oneofKind: 'userId', userId: id })
        ) || []
      )
    })
  })
</script>

<People
  href={CreateNavigation.ToCommunity(username)}
  onClick={() =>
    updatePersistentDataStore({ communitySection: CommunitySection.FRIENDS })}
  raw_icon={FriendsIconRaw}
  label="Friends"
  isLoading={$userFriends.isLoading}
  images={$friends.map((friend) => friend.data?.picture || null)}
/>
