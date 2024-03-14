<script lang="ts">
  import { fade } from "svelte/transition"
  import { CreateNavigation } from "$lib/const"
  import { useUsersIDs, getUserInfoQuery } from "$lib/query/user"
  import { getModalStore } from "@skeletonlabs/skeleton"
  import UserPanel from "../user-panel.svelte"
  import { authedUserStore } from "$lib/stores/user"
  import { createQueries } from "@tanstack/svelte-query"
  import { derived } from "svelte/store"
  import { localStorageStore } from "@skeletonlabs/skeleton"
  import { onMount } from "svelte"

  export let parent: unknown
  const modalStore = getModalStore()

  const debouncedSearch = localStorageStore("searchbar_input", "")
  let timer: number

  const searchWithDebounce = (v: string) => {
    clearTimeout(timer)
    timer = window.setTimeout(() => {
      debouncedSearch.set(v)
    }, 250)
  }

  const [usersIDs, usersIDsStore] = useUsersIDs({
    userId: $authedUserStore?.id,
  })

  $: usersIDsStore.set({ payload: { userId: $authedUserStore?.id, pattern: $debouncedSearch } })

  const users = createQueries({
    queries: derived(usersIDs, IDs => {
      return IDs.data?.userIds.map(userId => getUserInfoQuery({ oneofKind: "userId", userId })) || []
    }),
  })

  let input_el: HTMLInputElement
  onMount(() => {
    if (input_el) {
      input_el.value = $debouncedSearch
    }
  })
</script>

<section class="w-[600px] max-w-[95%] flex flex-col gap-y-3">
  <!-- Search ifself -->
  <div class="py-2">
    <input
      bind:this={input_el}
      on:keyup={event => searchWithDebounce(event.target?.value)}
      class="input variant-filled-surface border-secondary-400-500-token caret-secondary-500 focus:border-primary-500" />
  </div>
  <!-- Suggestions  -->
  <div
    class="p-5 w-full max-h-[600px] flex-1 card rounded-3xl [&>article]:overflow-x-hidden [&>article]:overflow-y-auto [&>article]:max-h-[520px] [&>article]:h-[520px] [&>article]:p-3">
    <h4 class="h4 text-center pb-5">Users</h4>
    <article>
      {#if $usersIDs.isFetching}
        <div in:fade={{ duration: 200 }} class="flex flex-col gap-y-3 h-full">
          <!-- todo: add good fallback -->
        </div>
      {:else}
        <div in:fade={{ duration: 200 }} class="flex flex-col gap-y-3">
          {#each $users as user}
            {#if user.isFetching}
              <div in:fade={{ duration: 200 }}>
                <UserPanel username={""} withIntegrations={false} />
              </div>
            {:else if user.data}
              <a in:fade={{ duration: 200 }} on:click={() => modalStore.close()} href={CreateNavigation.ToUser(user.data.username)}>
                <UserPanel username={user.data.username} pseudonym={user.data.pseudonym} picture={user.data.picture} withIntegrations={false} />
              </a>
            {/if}
          {/each}
        </div>
      {/if}
    </article>
    <!-- <h4 class="h4 text-center py-5">Users</h4>
    <article class="px-2">
      <div class="flex flex-col gap-y-3">
        {#each [$user, $user, $user, $user] as u}
          <UserPanel username={u.data?.username || ""} pseudonym={u.data?.pseudonym} picture={u.data?.picture} withIntegrations={false} />
        {/each}
      </div>
    </article> -->
  </div>
</section>
