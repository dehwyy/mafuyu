<script lang="ts">
  import { Tab, TabGroup } from '@skeletonlabs/skeleton'
  import { page } from '$app/stores'
  import FollowersIconRaw from '$lib/assets/people-group.svg?raw'
  import FriendsIconRaw from '$lib/assets/people.svg?raw'
  import { useIsMounted } from '$lib/hooks/use-is-mounted'
  import {
    useUserFollowedTo,
    useUserFollowers,
    useUserFriends
  } from '$lib/query/friends'
  import { useUserInfo } from '$lib/query/user'
  import {
    CommunitySection,
    persistentDataStore,
    updatePersistentDataStore
  } from '$lib/stores/nav'
  import { authedUserStore } from '$lib/stores/user'
  import { derived } from 'svelte/store'

  import Followed from './tabs/followed.svelte'
  import Followers from './tabs/followers.svelte'
  import Friends from './tabs/friends.svelte'

  // TODO: prefetch in +layout.server.ts
  const [currentUser, currentUserStore] = useUserInfo({
    oneofKind: 'username',
    username: $page.params.username
  })
  $: currentUserStore.set({
    getBy: { oneofKind: 'username', username: $page.params.username }
  })

  const [friends, friendsStore] = useUserFriends($currentUser.data?.userId)
  $: friendsStore.set({ userId: $currentUser.data?.userId, limit: undefined })

  const [followed, followedStore] = useUserFollowedTo($currentUser.data?.userId)
  $: followedStore.set({ userId: $currentUser.data?.userId, limit: undefined })

  const [followers, followersStore] = useUserFollowers(
    $currentUser.data?.userId
  )
  $: followersStore.set({ userId: $currentUser.data?.userId, limit: undefined })

  const friendsIDs = derived(friends, (friends) => {
    return friends.data?.friends || []
  })

  const followedIDs = derived(followed, (followed) => {
    return followed.data?.followers || []
  })

  const followersIDs = derived(followers, (friend) => {
    return friend.data?.followers || []
  })

  const tabBySection = (section: string) => {
    return (
      {
        frineds: 0,
        followed: 1,
        followers: 2
      }[section] || 0
    )
  }

  $: isCurrentUser = $authedUserStore?.username === $page.params.username

  $: setVirtualPath = (tab: number) => {
    const sect = {
      0: CommunitySection.FRIENDS,
      1: CommunitySection.FOLLOWED,
      2: CommunitySection.FOLLOWERS
    }[tab]

    if (sect) {
      updatePersistentDataStore({ communitySection: sect })
    }
  }

  $: scrollElement = () => {
    elemCarousel.scroll(elemCarousel.clientWidth * tab, 0)
  }

  let tab = 0
  let innerWidth = 421
  let elemCarousel: HTMLElement
  const isMounted = useIsMounted()

  $: tab = tabBySection($persistentDataStore.communitySection)
  $: elemCarousel && scrollElement()
  console.log($friends.isFetching)
</script>

<svelte:window
  bind:innerWidth
  on:resize={() => scrollElement()}
/>

<div class="flex flex-col gap-y-5 w-full">
  <TabGroup
    justify={innerWidth < 500 && $isMounted ? 'flex-col' : 'flew-row'}
    flex="flex-1"
  >
    <Tab
      on:click={() => setVirtualPath(0)}
      bind:group={tab}
      value={0}
      name="friends"
    >
      <div class="tab">
        <div class="icon-sm">
          {@html FriendsIconRaw}
        </div>
        <span> Friends </span>
      </div>
    </Tab>
    <Tab
      on:click={() => setVirtualPath(1)}
      bind:group={tab}
      value={1}
      name="followedTo"
    >
      <div class="tab">
        <div class="icon-sm min-h-[24px] min-w-[24px]">
          {@html FollowersIconRaw}
        </div>
        <span> Followed to </span>
      </div>
    </Tab>
    <Tab
      on:click={() => setVirtualPath(2)}
      bind:group={tab}
      value={2}
      name="followers"
    >
      <div class="tab">
        <div class="icon-sm">
          {@html FollowersIconRaw}
        </div>
        <span> Followers </span>
      </div>
    </Tab>
    <svelte:fragment slot="panel">
      <div
        bind:this={elemCarousel}
        class="overflow-x-hidden snap-x snap-mandatory scroll-smooth"
      >
        <div class="grid grid-cols-3 gap-x-5 overflow-x-auto w-[300%]">
          <Friends
            isFetching={$friends.isFetching}
            {friendsIDs}
            {isCurrentUser}
          />
          <Followed
            isFetching={$followed.isFetching}
            {followedIDs}
          />
          <Followers
            isFetching={$followers.isFetching}
            {followersIDs}
          />
        </div>
      </div>
    </svelte:fragment>
  </TabGroup>
</div>

<style lang="scss">
  .tab {
    @apply flex items-center gap-x-3 justify-center -ml-3 font-[700] bg-surface-800 sm:bg-transparent rounded-2xl p-1 sm:p-0;
  }
</style>
