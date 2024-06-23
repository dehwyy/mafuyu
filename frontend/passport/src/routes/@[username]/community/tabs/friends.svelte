<script lang="ts">
  import { createQueries } from '@tanstack/svelte-query'
  import { browser } from '$app/environment'
  import {
    UserExplore,
    UserPanel,
    UserPanelFallback
  } from '$lib/components/user'
  import { CreateNavigation } from '$lib/const'
  import { getBaseUserInfoQuery } from '$lib/query/user'
  import { derived } from 'svelte/store'
  import { fade, slide } from 'svelte/transition'
  import type { Readable } from 'svelte/store'

  export let isCurrentUser = false
  export let isFetching = false
  export let friendsIDs: Readable<string[]>

  const friends = createQueries({
    queries: derived(friendsIDs, (friendsIDs) => {
      return (
        friendsIDs.map((friendID) =>
          getBaseUserInfoQuery({ oneofKind: 'userId', userId: friendID })
        ) || []
      )
    })
  })

  console.log(isFetching, isCurrentUser)

  $: atLeastOne =
    $friends.reduce((acc, v) => acc + (v.data || v.isFetching ? 1 : 0), 0) > 0
</script>

{#if atLeastOne}
  <div class="panel-container">
    {#each $friends as friend}
      <a href={CreateNavigation.ToUser(friend.data?.username || '')}>
        <UserPanel
          isLoading={friend.isFetching}
          username={friend.data?.username}
          pseudonym={friend.data?.pseudonym}
          picture={friend.data?.picture}
          withIntegrations={true}
        />
      </a>
    {/each}
  </div>
{:else if isFetching || !browser}
  <div class="panel-container">
    {#each Array(4) as _}
      <UserPanel isLoading={true} />
    {/each}
  </div>
{:else if isCurrentUser}
  <UserExplore />
{:else}
  <p
    in:fade={{ duration: 100 }}
    class="text-center text-xl mt-10"
  >
    No Friends
  </p>
{/if}

<style lang="scss">
  .panel-container {
    @apply flex flex-col gap-y-5 py-5;
  }
</style>
