<script lang="ts">
	import { fade } from 'svelte/transition';
  import { Birthday, Location, Friends, Followers, Languages } from "$lib/components/user/index"
  import { useBlockedUsers, useUserInfo } from "$lib/query/user"
  import { page } from "$app/stores"
  import { authed_user_store } from "$lib/stores/user"

  const [user, userStore] = useUserInfo({ oneofKind: "username", username: $page.params.username })
  $: userStore.set({ getBy: { oneofKind: "username", username: $page.params.username } })

  const [authedUserBlockedUsers, authedUserBlockedUsersStore] = useBlockedUsers($authed_user_store?.id)
  $: authedUserBlockedUsersStore.set({ userId: $authed_user_store?.id })

  $: userId = $user?.data?.userId as string
  $: username = $user?.data?.username as string
  $: isProfileInBlock = $authedUserBlockedUsers?.data?.blockedUsers.includes(userId) ?? false
</script>

  <section class="flex flex-col gap-y-4 pt-6">
{#if $user.data && (!isProfileInBlock || $authedUserBlockedUsers.isFetching)}
    <section transition:fade={{duration: 100}} class="card p-6 w-full flex flex-col gap-y-4">
      <Birthday />
      <Location location={$user.data?.location} />
      <hr />
      <Friends {userId} {username}/>
      <Followers {userId} {username} />
      <hr />
      <Languages languages={$user.data?.languages || []} />
    </section>
{:else if isProfileInBlock && $user.data}
    <section in:fade={{duration: 100, delay: 100}} class="card p-6 w-full">
      <p class="text-center"><strong class="font-extrabold underline">@{$user.data.username}</strong> is blocked</p>
    </section>
{/if}
  </section>
