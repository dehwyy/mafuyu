<script lang="ts">
  import UserIconRaw from "$lib/assets/user.svg?raw"
  import GearIconRaw from "$lib/assets/gear.svg?raw"
  import FriendsIconRaw from "$lib/assets/people-group.svg?raw"
  import CircleIconRaw from "$lib/assets/circle.svg?raw"
  import CollectionIconRaw from "$lib/assets/social.svg?raw"
  import NotesIconRaw from "$lib/assets/notes.svg?raw"
  import LogoutIconRaw from "$lib/assets/logout.svg?raw"

  import { CreateNavigation, Routes } from "$lib/const"
  import ThemeSelector from "./theme-selector.svelte"
  import { useUserInfo } from "$lib/query/user"
  import { authedUserStore, clear_user } from "$lib/stores/user"

  const [user, userStore] = useUserInfo({ oneofKind: "userId", userId: $authedUserStore?.id })
  $: userStore.set({ getBy: { oneofKind: "userId", userId: $authedUserStore?.id } })

  $: username = $authedUserStore?.username || ($user?.data?.username as string)
  $: user_href = `/@${username}`

  $: panels = [
    { icon: UserIconRaw, href: CreateNavigation.ToUser(username), placeholder: username },
    { icon: GearIconRaw, href: CreateNavigation.ToSettings(username), placeholder: "Settings" },
    { component: ThemeSelector },
    { icon: CircleIconRaw, href: Routes.Circle, placeholder: "Circle" },
    { icon: FriendsIconRaw, href: CreateNavigation.ToFriends(username), placeholder: "Friends" },
    // { icon: CollectionIconRaw, href: null , placeholder: "Collections" },
    {
      icon: LogoutIconRaw,
      href: Routes.Logout,
      placeholder: "Logout",
      preAction: async () => {
        // TODO: invalidate session
        await new Promise(resolve => {
          setTimeout(() => resolve(null), 0)
        })
      },
    },
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
          on:click={async e => {
            if (panel.preAction) {
              e.preventDefault()
              await panel.preAction()
            }
            panel.href && (window.location.href = panel.href)
          }}>
          <button>
            <span>{@html panel.icon}</span>
            <span>{panel.placeholder}</span>
          </button>
        </a>
      {/if}
    </li>
  {/each}
</ul>

<style lang="scss">
  ul li button {
    @apply max-h-[40px] btn hover:bg-surface-300/10 text-surface-200 hover:text-white w-full justify-start pl-4 font-medium;
    & span:first-child {
      @apply h-[24px] mr-2;
    }
  }
</style>
