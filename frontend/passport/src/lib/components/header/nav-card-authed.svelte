<script lang="ts">
  import UserIconRaw from "$lib/assets/user.svg?raw"
  import GearIconRaw from "$lib/assets/gear.svg?raw"
  import FriendsIconRaw from "$lib/assets/people-group.svg?raw"
  import CollectionIconRaw from "$lib/assets/social.svg?raw"
  import NotesIconRaw from "$lib/assets/notes.svg?raw"
  import LogoutIconRaw from "$lib/assets/logout.svg?raw"
  import ThemeSelectIconRaw from "$lib/assets/theme-select.svg?raw"

  import { Routes } from "$lib/const"
  import ThemeSelector from "./theme-selector.svelte"
  import { useCurrentUserInfo } from "$lib/query/user"
  import { authed_user_store, clear_user } from "$lib/stores/user"

  const user = useCurrentUserInfo($authed_user_store?.id)

  $: username = $authed_user_store?.username || $user?.data?.username
  $: user_href = `/@${username}`
</script>

<ul>
  <li>
    <a href={user_href}>
      <button>
        <span>{@html UserIconRaw}</span>
        <span>{username}</span>
      </button>
    </a>
  </li>
  <li>
    <button>
      <span>{@html GearIconRaw}</span>
      <span>Settings</span>
    </button>
  </li>
  <li>
    <div class="">
      <ThemeSelector>
        <span class="icon-sm block">{@html ThemeSelectIconRaw}</span>
      </ThemeSelector>
    </div>
  </li>
  <li>
    <button>
      <span>{@html FriendsIconRaw}</span>
      <span>Friends</span>
    </button>
  </li>
  <li>
    <a href={`${user_href}/collections`}>
      <button>
        <span>{@html CollectionIconRaw}</span>
        <span>Collections</span>
      </button>
    </a>
  </li>
  <li>
    <a href={`${user_href}/collections`}>
      <button>
        <span>{@html NotesIconRaw}</span>
        <span>Notes</span>
      </button>
    </a>
  </li>
  <li>
    <a href={Routes.Logout}>
      <button on:click={() => clear_user()}>
        <span>{@html LogoutIconRaw}</span>
        <span>Logout</span>
      </button>
    </a>
  </li>
</ul>

<style lang="scss">
  ul li button {
    @apply max-h-[40px] btn hover:bg-surface-300/10 text-surface-200 hover:text-white w-full justify-start pl-4 font-medium;
    & span:first-child {
      @apply h-[24px] mr-2;
    }
  }
</style>
