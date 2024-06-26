<script lang="ts">
  import CircleIconRaw from '$lib/assets/circle.svg?raw'
  import LogoutIconRaw from '$lib/assets/logout.svg?raw'
  import FriendsIconRaw from '$lib/assets/people-group.svg?raw'
  import UserIconRaw from '$lib/assets/user.svg?raw'
  import { Button, Icon } from '$lib/components/header/nav-items'
  import SearchBarTrigger from '$lib/components/search-bar/trigger.svelte'
  import { CreateNavigation, Routes } from '$lib/const'
  import { useUserInfo } from '$lib/query/user'
  import { CommunitySection, updatePersistentDataStore } from '$lib/stores/nav'
  import { authedUserStore } from '$lib/stores/user'

  import CustomSettings from './custom-settings.svelte'

  const [user, userStore] = useUserInfo({
    oneofKind: 'userId',
    userId: $authedUserStore?.id
  })
  $: userStore.set({
    getBy: { oneofKind: 'userId', userId: $authedUserStore?.id }
  })

  $: username = $authedUserStore?.username || ($user?.data?.username as string)

  $: panels = [
    {
      icon: UserIconRaw,
      href: CreateNavigation.ToUser(username),
      placeholder: username
    },
    // { icon: GearIconRaw, href: CreateNavigation.ToSettings(username), placeholder: "Settings" },
    { component: CustomSettings },
    { icon: CircleIconRaw, href: Routes.Circle, placeholder: 'Circle' },
    {
      icon: FriendsIconRaw,
      href: CreateNavigation.ToCommunity(username),
      placeholder: 'Friends',
      preAction: () =>
        updatePersistentDataStore({
          communitySection: CommunitySection.FRIENDS
        })
    },
    {
      icon: LogoutIconRaw,
      href: Routes.Logout,
      placeholder: 'Logout',
      preAction: async () => authedUserStore.set(null)
    }
  ]
</script>

<ul>
  {#each panels as panel}
    <li>
      {#if panel.component}
        <svelte:component this={panel.component} />
      {:else}
        <a
          href={panel.href}
          on:click={async (e) => {
            if (panel.preAction) {
              e.preventDefault()
              await panel.preAction()
            }
            panel.href && (window.location.href = panel.href)
          }}
        >
          <Button>
            <Icon rawIcon={panel.icon} />
            <span>{panel.placeholder}</span>
          </Button>
        </a>
      {/if}
    </li>
  {/each}
</ul>
