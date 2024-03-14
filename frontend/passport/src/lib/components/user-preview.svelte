<script lang="ts">
  import { useUserInfo } from "$lib/query/user"
  import { authedUserStore } from "$lib/stores/user"
  import { DevFallbackImages } from "$lib/const"

  export let username: string

  const [user, userStore] = useUserInfo({ oneofKind: "username", username })
  $: userStore.set({ getBy: { oneofKind: "username", username } })

  const [currentUser, currentUserStore] = useUserInfo({ oneofKind: "userId", userId: $authedUserStore?.id })
  $: currentUserStore.set({ getBy: { oneofKind: "userId", userId: $authedUserStore?.id } })

  $: isCurrentUser = username === $authedUserStore?.username

  $: userImage = (isCurrentUser && $currentUser?.data?.picture) || $user.data?.picture || DevFallbackImages.HorizontalOriented
  $: userPseudonym = (isCurrentUser && $currentUser?.data?.pseudonym) || $user.data?.pseudonym
</script>

<section class="mb-7">
  <div class="w-[175px] h-[175px] object-cover overflow-hidden rounded-full mb-4 border-2 mx-auto">
    <img alt="account_image" class="w-full h-full object-cover" src={userImage} />
  </div>
  <div class="text-center">
    <h4 class="h4">{username}</h4>
    {#if userPseudonym}
      <h6 class="h6 opacity-50">{userPseudonym}</h6>
    {/if}
  </div>
</section>
