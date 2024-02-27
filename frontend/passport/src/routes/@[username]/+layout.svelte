<script lang="ts">
  import { authed_user_store } from "$lib/stores/user"
  import { page } from "$app/stores"
  import { ListBox, ListBoxItem } from "@skeletonlabs/skeleton"
  import UserPreview from "$lib/components/user-preview.svelte"
  import UserActions from "$lib/components/user-actions.svelte"
  import { CreateNavigation } from "$lib/const"
  import useSyncedNavigation from "$lib/hooks/use-synced-navigation"
  import { useBlockedUsers, useUserInfo } from "$lib/query/user"
  import { useHydrate } from "@tanstack/svelte-query"
  import { useUserProfileScopes } from "$lib/query/profile"

  export let data: import("./$types").LayoutData
  $: useHydrate(data.dehydrateState)

  const [user, userStore] = useUserInfo({ oneofKind: "username", username: $page.params.username })
  $: userStore.set({ getBy: { oneofKind: "username", username: $page.params.username } })

  const [userScopes, userScopesStore] = useUserProfileScopes($user.data?.userId)
  $: userScopesStore.set({ userId: $user.data?.userId })

  const [userBlockedUsers, userBlockedUsersStore] = useBlockedUsers($user.data?.userId)
  $: userBlockedUsersStore.set({ userId: $user.data?.userId })

  $: navigation = useSyncedNavigation({
    base_route: `/@${$page.params.username}`,
    current_route: $page.url.pathname,
    navigations: {
      "/": {
        placeholder: "Overview",
        isActive: $userScopes.data?.viewInfo ?? false,
      },
      "/statistics": {
        placeholder: "Statistics",
        isActive: $userScopes.data?.viewStatistics ?? false,
      },
      "/edit": {
        placeholder: "Settings",
        isActive: $userScopes.data?.edit ?? false,
      },
      "/self": {
        placeholder: "Self",
        isActive: $userScopes.data?.edit ?? false, // TODO
      },
    },
  })

  $: is_current_user = $page.params.username === $authed_user_store?.username
  $: username = $user.data?.username as string
  $: isAuthedUserBlocked = $userBlockedUsers.data?.blockedUsers.includes($authed_user_store?.id!) ?? false
</script>

{#if isAuthedUserBlocked}
  <div class="absolute top-1/2 -translate-y-1/2 z-10 left-1/2 -translate-x-1/2">
    <p class="[&>strong]:underline text-3xl">
      <strong>@{username}</strong>
      was blocked
      <strong>You</strong>
    </p>
    <hr class="my-5 border-b-2 !border-b-primary-700" />
    <p class="text-center">
      <button on:click={() => window.history.back()} class="variant-glass-secondary btn-xl rounded-token">Go back</button>
    </p>
  </div>
{/if}

<div class={`${isAuthedUserBlocked && "blur-md select-none pointer-events-none pb-5 relative"} page-layout-wrapper`}>
  <div class="page-layout">
    <nav>
      {#if $user.data}
        <UserPreview {username} />
      {/if}
      {#if !is_current_user && $user.data}
        <UserActions userId={$user.data.userId} {username} />
        <hr class="mb-5" />
      {/if}
      <ListBox>
        {#each navigation.iter() as n}
          <a href={CreateNavigation.ToUser(username, n.value)} class="navigation_links">
            <ListBoxItem bind:group={navigation.current_value} name="user_navigation" value={n.value}>{n.placeholder}</ListBoxItem>
          </a>
        {/each}
      </ListBox>
    </nav>
    <main>
      <slot />
    </main>
  </div>
</div>

<style lang="scss">
  .page-layout {
    @apply w-full lg:w-[800px] px-5 lg:px-0 mx-auto pt-14 flex flex-col lg:flex-row gap-8 gap-x-16;
  }
  .page-layout-wrapper {
    margin-right: calc(100% - 100vw);
    max-width: 1450px;
  }
  nav {
    min-width: 230px;
  }
  main {
    @apply w-full pb-10;
  }
  .navigation_links {
    @apply text-center text-lg mb-2 block;
  }
</style>
