<script lang="ts">
	import { Routes } from '$lib/const';
  import UserIconRaw from "$lib/assets/user.svg?raw"
  import GearIconRaw from "$lib/assets/gear.svg?raw"
  import FriendsIconRaw from "$lib/assets/people-group.svg?raw"
  import CollectionIconRaw from "$lib/assets/social.svg?raw"
  import NotesIconRaw from "$lib/assets/notes.svg?raw"
  import LogoutIconRaw from  "$lib/assets/logout.svg?raw"
  import ThemeSelectIconRaw from "$lib/assets/theme-select.svg?raw"

  import {type PopupSettings, popup } from "@skeletonlabs/skeleton"
  import ThemeSelector from './theme-selector.svelte'


  const settingsClick: PopupSettings = {
    event: "click",
    placement: "bottom-end",
    target: "header-account-popup",
  }

</script>

<button class="h-[40px]" use:popup={settingsClick}>
  <div class="max-h-full max-w-[40px]  overflow-hidden rounded-2xl">
    <img src="/images/r.jpg" alt="user" class="w-full"/>
  </div>
</button>

<section data-popup="header-account-popup" class="" >
  <div class="flex flex-col border border-surface-600 bg-surface-800 mt-5 rounded-3xl max-w-full w-[220px]">
  <div class="w-full h-[220px] overflow-hidden relative rounded-t-2xl">
    <img src="/images/hana.png" alt="user" class="opacity-70 w-full h-full object-cover"/>
    <div class="absolute bottom-0 flex h-full w-full bg-gradient-to-t from-surface-800 from-15% to-transparent " />
  </div>
  <div class="relative -mt-16 mb-2">
    <ul>
      <li>
        <a href="/@dehwyy">
        <button>
          <span>{@html UserIconRaw}</span>
          <span>dehwyy</span>
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
        <div class="flex items-center h-[40px] hover:bg-surface-300/10 text-surface-200 hover:text-white w-full pl-4 gap-x-2 font-medium">
          <span class="icon-sm block">{@html ThemeSelectIconRaw}</span>
          <ThemeSelector />
        </div>
      </li>
      <li>
        <button>
          <span>{@html FriendsIconRaw}</span>
          <span>Friends</span>
        </button>
      </li>
      <li>
        <a href="/@dehwyy/collections">
        <button>
          <span>{@html CollectionIconRaw}</span>
          <span>Collections</span>
        </button>
        </a>
      </li>
      <li>
        <a href="/@dehwyy/collections">
        <button>
          <span>{@html NotesIconRaw}</span>
          <span>Notes</span>
        </button>
        </a>
      </li>
      <li>
        <a href={Routes.Logout}>
        <button>
          <span>{@html LogoutIconRaw}</span>
          <span>Logout</span>
        </button>
        </a>
      </li>
    </ul>
  </div>

  </div>
  </section>


<style lang="scss">
  ul li button {
    @apply max-h-[40px] btn hover:bg-surface-300/10 text-surface-200 hover:text-white w-full justify-start pl-4 font-medium;
    & span:first-child {
      @apply h-[24px] mr-2;
    }
  }
</style>
