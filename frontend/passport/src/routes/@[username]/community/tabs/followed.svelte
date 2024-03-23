<script lang="ts">
  import { browser } from "$app/environment"
  import { fade } from "svelte/transition"
  import { getBaseUserInfoQuery } from "$lib/query/user"
  import UserPanel from "$lib/components/user/user-panel.svelte"
  import { CreateNavigation } from "$lib/const"
  import { createQueries } from "@tanstack/svelte-query"
  import { derived, type Readable } from "svelte/store"

  export let followedIDs: Readable<string[]>
  export let isFetching = false

  const followers = createQueries({
    queries: derived(followedIDs, followersIDs => {
      return followersIDs.map(followedId => getBaseUserInfoQuery({ oneofKind: "userId", userId: followedId })) || []
    }),
  })

  $: atLeastOne = $followers.reduce((acc, v) => acc + (v.data || v.isFetching ? 1 : 0), 0) > 0
</script>

{#if atLeastOne}
  <div class="panel-container">
    {#each $followers as followed}
      <a href={CreateNavigation.ToUser(followed.data?.username || "")}>
        <UserPanel
          isLoading={followed.isFetching}
          username={followed.data?.username}
          pseudonym={followed.data?.pseudonym}
          picture={followed.data?.picture} />
      </a>
    {/each}
  </div>
{:else if isFetching || !browser}
  <div class="panel-container">
    {#each Array(4) as _}
      <UserPanel isLoading={true} />
    {/each}
  </div>
{:else}
  <p in:fade={{ duration: 100 }} class="text-center text-xl mt-10">No Followers</p>
{/if}

<style lang="scss">
  .panel-container {
    @apply flex flex-col gap-y-5 py-5;
  }
</style>
