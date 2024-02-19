<script lang="ts">
  import { DevFallbackImages, Routes } from "$lib/const"
  import NavCardAuthed from "./nav-card-authed.svelte"
  import NavCardUnauthed from "./nav-card-unauthed.svelte"

  import { type PopupSettings, popup } from "@skeletonlabs/skeleton"
  import { useCurrentUserInfo } from "$lib/query/user"
  import { authed_user_store, dyn_user_store } from "$lib/stores/user"

  const settingsClick: PopupSettings = {
    event: "click",
    placement: "bottom-end",
    target: "header-account-popup",
  }

  const user = useCurrentUserInfo($authed_user_store?.id)

  $: picture = ($authed_user_store?.id && ($dyn_user_store?.picture || $user?.data?.picture)) || DevFallbackImages.HorizontalOriented
</script>

<button class="h-[40px]" use:popup={settingsClick}>
  <div class="h-full max-w-[40px] overflow-hidden rounded-full border border-surface-500">
    <img src={picture} alt="user" class="h-full w-full object-cover" />
  </div>
</button>

<section data-popup="header-account-popup" class="">
  <div class="flex flex-col border border-surface-600 shadow-md shadow-primary-500 bg-surface-800 mt-5 rounded-3xl max-w-full w-[220px]">
    <div class="w-full h-[220px] overflow-hidden relative rounded-t-2xl">
      <img src={picture} alt="" class="opacity-70 w-full h-full object-cover" />
      <div class="absolute bottom-0 flex h-full w-full bg-gradient-to-t from-surface-800 from-15% to-transparent" />
    </div>
    <div class="relative -mt-16 mb-2">
      {#if $authed_user_store?.id}
        <NavCardAuthed />
      {:else}
        <NavCardUnauthed />
      {/if}
    </div>
  </div>
</section>
