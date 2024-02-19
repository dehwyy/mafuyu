<script lang="ts">
  import { authed_user_store, dyn_user_store } from "$lib/stores/user"
  import { page } from "$app/stores"
  import { ListBox, ListBoxItem } from "@skeletonlabs/skeleton"
  import UserPreview from "$lib/components/user-preview.svelte"
  import { CreateNavigation, DevFallbackImages } from "$lib/const"
  import useSyncedNavigation from "$lib/hooks/use-synced-navigation"
  import { useCurrentUserInfo } from "$lib/query/user"

  $: navigation = useSyncedNavigation({
    base_route: "/@[username]",
    current_route: $page.route.id!,
    navigations: {
      "/": {
        placeholder: "Overview",
      },
      "/statistics": {
        placeholder: "Statistics",
      },
      "/edit": {
        placeholder: "Settings",
      },
    },
  })

  export let data: import("./$types").LayoutData
  $: is_current_user = data.userId === $authed_user_store?.id

  $: current_user = useCurrentUserInfo(is_current_user ? $authed_user_store?.id : undefined)

  $: username = (is_current_user && $authed_user_store?.username) || (data.username as string)
  $: user_preview_image = (is_current_user && $current_user?.data?.picture) || data.picture
  $: user_preview_pseudonym = (is_current_user && $current_user?.data?.pseudonym) || data.pseudonym
</script>

<div class="page-layout-wrapper">
  <div class="page-layout">
    <nav>
      <UserPreview image={user_preview_image || DevFallbackImages.HorizontalOriented} pseudonym={user_preview_pseudonym} {username} />
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
