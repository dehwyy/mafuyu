<script lang="ts">
  import { fade } from "svelte/transition"
  import { getBaseUserInfoQuery } from "$lib/query/user"
  import UserPanel from "$lib/components/user-panel.svelte"
  import { CreateNavigation } from "$lib/const"
  import { createQueries } from "@tanstack/svelte-query"
  import { derived, type Readable } from "svelte/store"

  export let followersIDs: Readable<string[]>

  const followers = createQueries({
    queries: derived(followersIDs, followersIDs => {
      return followersIDs.map(followerId => getBaseUserInfoQuery({ oneofKind: "userId", userId: followerId })) || []
    }),
  })

  $: atLeastOne = $followers.reduce((acc, v) => acc + (v.data ? 1 : 0), 0) > 0
</script>

{#if atLeastOne}
  <div in:fade={{ duration: 100 }} class="flex flex-col gap-y-5 py-5">
    {#each $followers as follower}
      {#if follower.data}
        <a in:fade={{ duration: 100 }} href={CreateNavigation.ToUser(follower.data.username)}>
          <UserPanel username={follower.data.username} pseudonym={follower.data.pseudonym} picture={follower.data.picture} />
        </a>
      {/if}
    {/each}
  </div>
{:else}
  <p in:fade={{ duration: 100 }} class="text-center text-xl mt-10">No Followers</p>
{/if}
