<script lang="ts">
  import { popup } from '@skeletonlabs/skeleton'
  import { DevFallbackImages, Routes } from '$lib/const'
  import { useUserInfo } from '$lib/query/user'
  import { authedUserStore, dyn_user_store } from '$lib/stores/user'
  import type { PopupSettings } from '@skeletonlabs/skeleton'

  import NavCardAuthed from './nav-card-authed.svelte'
  import NavCardUnAuthed from './nav-card-unauthed.svelte'

  const settingsClick: PopupSettings = {
    event: 'click',
    placement: 'bottom-end',
    target: 'header-account-popup'
  }

  const [user, userStore] = useUserInfo({
    oneofKind: 'username',
    username: $authedUserStore?.username
  })
  $: userStore.set({
    getBy: { oneofKind: 'username', username: $authedUserStore?.username }
  })

  $: picture =
    ($authedUserStore?.id &&
      ($dyn_user_store?.picture || $user?.data?.picture)) ||
    DevFallbackImages.HorizontalOriented
</script>

<button
  class="h-[40px]"
  use:popup={settingsClick}
>
  <div
    class="h-[40px] w-[40px] overflow-hidden rounded-full border border-surface-500"
  >
    <img
      src={picture}
      alt="user"
      class="h-full w-full object-cover"
    />
  </div>
</button>

<section
  data-popup="header-account-popup"
  class=""
>
  <div
    class="flex flex-col border border-surface-600 shadow-md shadow-primary-500 bg-surface-800 mt-5 rounded-3xl max-w-full w-[220px]"
  >
    <div class="w-full h-[220px] overflow-hidden relative rounded-t-2xl">
      <img
        src={picture}
        alt=""
        class="opacity-70 w-full h-full object-cover"
      />
      <div
        class="absolute bottom-0 flex h-full w-full bg-gradient-to-t from-surface-800 from-15% to-transparent"
      />
    </div>
    <div class="relative -mt-16 mb-2">
      {#if $authedUserStore?.id}
        <NavCardAuthed />
      {:else}
        <NavCardUnAuthed />
      {/if}
    </div>
  </div>
</section>
