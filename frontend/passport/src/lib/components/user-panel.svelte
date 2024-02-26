<script lang="ts">
  import GoogleIconRaw from "$lib/assets/google.svg?raw"
  import GithubIconRaw from "$lib/assets/github.svg?raw"
  import { CreateNavigation } from "$lib/const"

  export let picture: string | undefined = undefined
  export let username: string
  export let pseudonym: string | undefined = undefined

  export let withIntegrations = true
  $: windowWidth = 0

  const sm = 640
</script>

<svelte:window bind:innerWidth={windowWidth} />

<button class="btn h-[86px] sm:grid grid-cols-8 gap-x-3 items-center card w-full overflow-hidden min-w-[200px]">
  <div class="col-span-1 select-none pointer-events-none h-[46px] w-[46px]">
    {#if picture}
      <img src={picture} alt="user" class="max-w-[46px] max-h-[46px] rounded-full object-cover" />
    {:else}
      <div class="w-[46px] h-[46px] animate-pulse bg-surface-500" />
    {/if}
  </div>
  <div class="flex items-center col-span-4 sm:col-span-5 overflow-hidden relative h-full">
    <p class="break-keep sm:absolute top-1/2 sm:-translate-y-1/2">
      <span>{username}</span>&nbsp;
      {#if windowWidth > 360 && pseudonym}<span>({pseudonym})</span>{/if}
    </p>
  </div>
  {#if withIntegrations && windowWidth > sm}
    <div class="sm:cols-span-3 ml-auto flex items-center gap-x-3">
      <a href={CreateNavigation.ToGoogleIntegration(username)}>
        <button on:click|stopPropagation class="icon">
          {@html GoogleIconRaw}
        </button>
      </a>
      <a href={CreateNavigation.ToGithubIntegration(username)}>
        <button on:click|stopPropagation class="icon">
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
