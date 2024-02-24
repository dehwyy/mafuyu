<script lang="ts">
  import { authed_user_store } from "$lib/stores/user"
  import { page } from "$app/stores"
  import { ListBox, ListBoxItem } from "@skeletonlabs/skeleton"
  import UserPreview from "$lib/components/user-preview.svelte"
  import UserActions from "$lib/components/user-actions.svelte"
  import { CreateNavigation, DevFallbackImages } from "$lib/const"
  import useSyncedNavigation from "$lib/hooks/use-synced-navigation"
  import { useUserInfo } from "$lib/query/user"
  import { useHydrate } from "@tanstack/svelte-query"
  import { useUserProfileScopes } from "$lib/query/profile"

  export let data: import("./$types").LayoutData
  useHydrate(data.dehydrateState)

  const [user, userStore] = useUserInfo({ oneofKind: "username", username: $page.params.username })
  $: userStore.set({ getBy: {oneofKind: "username", username: $page.params.username }})

  const [userScopes, userScopesStore] = useUserProfileScopes($user.data?.userId)
  $: userScopesStore.set({ userId: $user.data?.userId })

  $: navigation = useSyncedNavigation({
    base_route: "/@[username]",
    current_route: $page.route.id!,
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
    },
  })

  $: is_current_user = $user.data?.userId === $authed_user_store?.id

  const [current_user, currentUserStore] = useUserInfo({ oneofKind: "userId", userId: is_current_user ? $authed_user_store?.id : undefined })
  $: currentUserStore.set({ getBy: { oneofKind: "userId", userId: is_current_user ? $authed_user_store?.id : undefined } })

  $: username = (is_current_user && $authed_user_store?.username) || ($user.data?.username as string)
  $: user_preview_image = (is_current_user && $current_user?.data?.picture) || $user.data?.picture
  $: user_preview_pseudonym = (is_current_user && $current_user?.data?.pseudonym) || $user.data?.pseudonym
</script>

<div class="page-layout-wrapper">
  <div class="page-layout">
    <nav>
      <UserPreview image={user_preview_image || DevFallbackImages.HorizontalOriented} pseudonym={user_preview_pseudonym} {username} />
      {#if !is_current_user && $user.data}
        <UserActions userId={$user.data.userId} username={$user.data.username} />
        <hr class="mt-4 mb-5" />
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
