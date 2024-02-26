<script lang="ts">
  import { fade } from "svelte/transition"

  import { getBaseUserInfoQuery } from "$lib/query/user"
  import UserPanel from "$lib/components/user-panel.svelte"
  import { CreateNavigation } from "$lib/const"
  import { createQueries } from "@tanstack/svelte-query"
  import { derived, type Readable } from "svelte/store"

  export let friendsIDs: Readable<string[]>

  const friends = createQueries({
    queries: derived(friendsIDs, friendsIDs => {
      return friendsIDs.map(friendID => getBaseUserInfoQuery({ oneofKind: "userId", userId: friendID })) || []
    }),
  })

  let atLeastOne = true
  $: atLeastOne = $friends.reduce((acc, v) => acc + (v.data || v.isFetching ? 1 : 0), 0) > 0
</script>

{#if atLeastOne}
  <div class="flex flex-col gap-y-5 py-5">
    {#each $friends as friend}
      {#if friend.data}
        <a in:fade={{ duration: 100 }} href={CreateNavigation.ToUser(friend.data.username)}>
          <UserPanel username={friend.data.username} pseudonym={friend.data.pseudonym} picture={friend.data.picture} withIntegrations={true} />
        </a>
      {/if}
    {/each}
  </div>
{:else}
  <p transition:fade={{ duration: 100 }} class="text-center text-xl mt-10">No friends</p>
{/if}
