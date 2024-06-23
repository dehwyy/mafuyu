<script lang="ts">
  import Card from '$lib/components/card.svelte'
  import {
    Birthday,
    FollowedTo,
    Followers,
    Friends,
    Languages,
    Location
  } from '$lib/components/user/fields'
  import { fade } from 'svelte/transition'

  import { usePage } from './hooks'

  const { user, isUserBlocked } = usePage()
</script>

{#if $user}
  <section class="information">
    {#if $isUserBlocked}
      <section
        in:fade={{ duration: 100, delay: 100 }}
        class="information__blocked card"
      >
        <p class="text-center">
          <strong class="font-extrabold underline">@{$user.username}</strong> is
          blocked
        </p>
      </section>
    {:else}
      <Card
        class="p-6 w-full flex flex-col gap-y-4 justify-betwee min-w-[250px]"
      >
        <Birthday />
        <Location location={$user.location} />
        <hr />
        <div class="flex-col flex gap-y-2">
          <Friends
            userId={$user.userId}
            username={$user.username}
          />
          <Followers
            userId={$user.userId}
            username={$user.username}
          />
          <FollowedTo
            userId={$user.userId}
            username={$user.username}
          />
        </div>
        {#if $user.languages.length}
          <hr />
          <Languages languages={$user.languages || []} />
        {/if}
      </Card>
    {/if}
  </section>
{/if}

<style lang="scss">
  .information {
    @apply flex flex-col gap-y-4 pt-6;

    &__blocked {
      @apply p-6 w-full;
    }
  }
</style>
