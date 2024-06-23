<script lang="ts">
  import { createQueries } from '@tanstack/svelte-query'
  import PeopleGroupIconRaw from '$lib/assets/people-group.svg?raw'
  import { CreateNavigation } from '$lib/const'
  import { useUserFollowers } from '$lib/query/friends'
  import { getBaseUserInfoQuery } from '$lib/query/user'
  import { CommunitySection, updatePersistentDataStore } from '$lib/stores/nav'
  import { derived } from 'svelte/store'

  import People from './people.svelte'

  export let userId: string
  export let username: string

  const [userFollowers, userFollowersStore] = useUserFollowers(userId)
  $: userFollowersStore.set({ userId, limit: undefined })

  const followers = createQueries({
    queries: derived(userFollowers, (userFollowers) => {
      const followers = userFollowers.data?.followers || []

      return (
        followers.map((id) =>
          getBaseUserInfoQuery({ oneofKind: 'userId', userId: id })
        ) || []
      )
    })
  })
</script>

<People
  onClick={() =>
    updatePersistentDataStore({ communitySection: CommunitySection.FOLLOWERS })}
  href={CreateNavigation.ToCommunity(username)}
  raw_icon={PeopleGroupIconRaw}
  label="Followers"
  isLoading={$userFollowers.isLoading}
  images={$followers.map((follower) => follower.data?.picture || null)}
/>
