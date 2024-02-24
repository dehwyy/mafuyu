<script lang="ts">
  import FriendsIconRaw from "$lib/assets/people.svg?raw"
  import BlockIconRaw from "$lib/assets/block.svg?raw"

  import { popup, type PopupSettings } from "@skeletonlabs/skeleton"
  import { useFollowUser, useUnfollowUser, useUserFollowers, useUserFriends } from "$lib/query/friends"
  import { authed_user_store } from "$lib/stores/user"

  export let userId: string
  export let username: string

  const follow = useFollowUser()
  const unfollow = useUnfollowUser()

  const [authedUserFriends, authedUserFriendsStore] = useUserFriends($authed_user_store?.id)
  $: authedUserFriendsStore.set({ userId: $authed_user_store?.id || "", limit: undefined })

  const [authed_user_followers, authedUserFollowersStore] = useUserFollowers($authed_user_store?.id)
  $: authedUserFollowersStore.set({ userId: $authed_user_store?.id || "", limit: undefined})

  const [user_followers, userFollowersStore] = useUserFollowers(userId)
  $: userFollowersStore.set({ userId, limit: undefined })

  // ! TODO: dynamic change (maybe using `writable`)

  $: is_friends = $authedUserFriends?.data?.friends.includes(userId)
  $: is_followed_to_user = $user_followers?.data?.followers.includes($authed_user_store?.id!) ?? false
  $: is_user_followed_to_you = $authed_user_followers.data?.followers.includes(userId) ?? false

  $: innerWidth = 0
  $: placement = (innerWidth < 1023 ? "bottom" : "right") as "bottom" | "right"
  $: is_open = false

  $: user_actions_popup = {
    placement,
    target: "user_actions",
    event: "click",
    state: e => {
      is_open = e.state
    },
  } satisfies PopupSettings

  $: options = [
    {
      icon: FriendsIconRaw,
      text: () => {
        if (is_friends) {
          return "Remove from friends"
        }
        if (is_user_followed_to_you && !is_followed_to_user) {
          return "Add to friends"
        }
        if (!is_user_followed_to_you && !is_followed_to_user) {
          return "Follow"
        }
        if (!is_user_followed_to_you && is_followed_to_user) {
          return "Unfollow"
        }
      },
      action: () => {
        // Friend
        if (is_friends) {
          is_friends = false
          $unfollow.mutate({ userId, successText: () => `Removed ${username} from friends.` })
        }
        // May become friends as user was followed to you
        else if (is_user_followed_to_you && !is_followed_to_user) {
          is_friends = true
          $follow.mutate({ userId, successText: () => `Added ${username} to friends.` })
        }
        // Become follower
        else if (!is_user_followed_to_you && !is_followed_to_user) {
          is_followed_to_user = true
          $follow.mutate({ userId, successText: () => `Followed ${username}.` })
        }
        // Unfollow
        else if (!is_user_followed_to_you && is_followed_to_user) {
          is_followed_to_user = false
          $unfollow.mutate({ userId, successText: () => `Unfollowed ${username}.` })
        }
      },
    },
    {
      icon: BlockIconRaw,
      text: () => "Block user",
      action: () => {
        is_followed_to_user = false
        $unfollow.mutate({ userId, successText: () => `Blocked ${username}.` })
      },
    },
  ]
</script>

<svelte:window bind:innerWidth />
<button
  use:popup={user_actions_popup}
  class:!border-primary-500={is_open}
  class="btn hover:variant-soft w-full font-bold text-lg border border-transparent">Actions</button>
<div data-popup="user_actions" class="card border border-surface-600">
  <ul class="list select-none py-3 px-0.5">
    {#each options as option}
      <li>
        <button
          on:click={option.action}
          class="w-full flex rounded-container-token pl-5 pr-7 py-3 cursor-pointer dark:hover:bg-surface-700 hover:surface-200 transition-all">
          <span class="icon-sm block mr-3">
            {@html option.icon}
          </span>
          <span class="font-600">
            {option.text()}
          </span>
        </button>
      </li>
    {/each}
  </ul>
</div>
