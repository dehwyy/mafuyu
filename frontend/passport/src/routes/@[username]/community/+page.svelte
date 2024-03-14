<script lang="ts">
  import { derived } from "svelte/store"

  import { useUserFollowedTo, useUserFollowers, useUserFriends } from "$lib/query/friends"
  import FriendsIconRaw from "$lib/assets/people.svg?raw"
  import FollowersIconRaw from "$lib/assets/people-group.svg?raw"

  import { page } from "$app/stores"
  import { TabGroup, TabAnchor } from "@skeletonlabs/skeleton"
  import { useUserInfo } from "$lib/query/user"
  import Friends from "./tabs/friends.svelte"
  import Followers from "./tabs/followers.svelte"
  import Followed from "./tabs/followed.svelte"
  import { CreateNavigation } from "$lib/const"
  import { authedUserStore } from "$lib/stores/user"

  // TODO: prefetch in +layout.server.ts

  $: section = $page.url.searchParams.get("section") || "friends" || "followed"
  $: username = $page.params.username

  $: isTabFriends = section === "friends"
  $: isTabFollowed = section === "followed"
  $: isTabFollowers = section === "followers"

  const [currentUser, currentUserStore] = useUserInfo({ oneofKind: "username", username: $page.params.username })
  $: currentUserStore.set({ getBy: { oneofKind: "username", username: $page.params.username } })

  const [friends, friendsStore] = useUserFriends($currentUser.data?.userId)
  $: friendsStore.set({ userId: $currentUser.data?.userId, limit: undefined })

  const [followed, followedStore] = useUserFollowedTo($currentUser.data?.userId)
  $: followedStore.set({ userId: $currentUser.data?.userId, limit: undefined })

  const [followers, followersStore] = useUserFollowers($currentUser.data?.userId)
  $: followersStore.set({ userId: $currentUser.data?.userId, limit: undefined })

  const friendsIDs = derived(friends, friends => {
    return friends.data?.friends || []
  })

  const followedIds = derived(followed, followed => {
    return followed.data?.followers || []
  })

  const followersIDs = derived(followers, friend => {
    return friend.data?.followers || []
  })

  $: isCurrentUser = $authedUserStore?.username === $page.params.username
</script>

<div class="flex flex-col gap-y-5 w-full">
  <TabGroup flex="flex-1" direction="vertical">
    <TabAnchor href={CreateNavigation.ToFriends(username)} selected={isTabFriends}>
      <div class="tab">
        <div class="icon-sm">
          {@html FriendsIconRaw}
        </div>
        <span> Friends </span>
      </div>
    </TabAnchor>
    <TabAnchor href={CreateNavigation.ToFollowed(username)} selected={isTabFollowed}>
      <div class="tab">
        <div class="icon-sm">
          {@html FollowersIconRaw}
        </div>
        <span> Followed to </span>
      </div>
    </TabAnchor>
    <TabAnchor href={CreateNavigation.ToFollowers(username)} selected={isTabFollowers}>
      <div class="tab">
        <div class="icon-sm">
          {@html FollowersIconRaw}
        </div>
        <span> Followers </span>
      </div>
    </TabAnchor>
    <svelte:fragment slot="panel">
      {#if isTabFriends}
        <Friends {friendsIDs} {isCurrentUser} />
      {:else if isTabFollowed}
        <Followed followedIDs={followedIds} />
      {:else if isTabFollowers}
        <Followers {followersIDs} />
      {/if}
    </svelte:fragment>
  </TabGroup>
</div>

<style lang="scss">
  .tab {
    @apply flex items-center gap-x-3 justify-center -ml-3;
    & > span {
      @apply font-[700];
    }
  }

  .icon {
    @apply btn-icon-base hover:bg-surface-600 transition-all p-2 rounded-xl;
  }
</style>
