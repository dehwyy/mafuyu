<script lang="ts">
  import GithubIconRaw from '$lib/assets/github.svg?raw'
  import GoogleIconRaw from '$lib/assets/google.svg?raw'
  import { CreateNavigation } from '$lib/const'
  import { fade } from 'svelte/transition'

  export let picture: string | undefined = undefined
  export let username: string | undefined = undefined
  export let pseudonym: string | undefined = undefined

  export let isLoading = false

  export let withIntegrations = true
  $: windowWidth = 0

  const sm = 640
</script>

<svelte:window bind:innerWidth={windowWidth} />

<button
  class="btn h-[86px] sm:grid grid-cols-5 gap-x-3 items-center card w-full overflow-hidden min-w-[200px]"
>
  <div class="col-span-1 select-none pointer-events-none h-[46px] w-[46px]">
    {#if picture && !isLoading}
      <img
        in:fade
        src={picture}
        alt="user"
        class="w-[46px] h-[46px] rounded-full object-cover"
      />
    {:else}
      <div
        class="w-[46px] h-[46px] rounded-full animate-pulse bg-surface-500"
      />
    {/if}
  </div>
  <div
    class="flex items-center col-span-2 sm:col-span-3 overflow-hidden relative h-full"
  >
    {#if isLoading}
      <div
        out:fade={{ duration: 100 }}
        class="bg-surface-500 animate-pulse h-[20px] w-full"
      ></div>
    {:else}
      <p
        in:fade={{ delay: 100 }}
        class="break-keep sm:absolute top-1/2 sm:-translate-y-1/2"
      >
        <span>{username}</span>&nbsp;
        {#if windowWidth > 360 && pseudonym}<span>({pseudonym})</span>{/if}
      </p>
    {/if}
  </div>
  {#if withIntegrations && windowWidth > sm && username}
    <div
      in:fade
      class="flex items-center gap-x-3"
    >
      <!-- Google's integration is kinda useless, I guess -->
      <!-- <a href={CreateNavigation.ToGoogleIntegration(username)}>
        <button on:click|stopPropagation class="icon">
          {@html GoogleIconRaw}
        </button>
      </a> -->
      <a
        class="ml-auto"
        href={CreateNavigation.ToGithubIntegration(username)}
      >
        <button
          on:click|stopPropagation
          class="icon"
        >
          {@html GithubIconRaw}
        </button>
      </a>
    </div>
  {/if}
</button>

<style lang="scss">
  .icon {
    @apply btn-icon-base hover:bg-surface-600 transition-all p-2 rounded-xl;
  }
</style>
