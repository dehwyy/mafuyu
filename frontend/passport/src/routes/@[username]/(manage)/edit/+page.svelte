<script lang="ts">
  import { page } from '$app/stores'
  import GithubIconRaw from '$lib/assets/github.svg?raw'
  import GoogleIconRaw from '$lib/assets/google.svg?raw'
  import SlideWindow from '$lib/components/transitions/slide-window.svelte'
  import { useUserInfo } from '$lib/query/user'

  import About from './about.svelte'

  const Integrations = [
    { icon: GoogleIconRaw, text: 'Connect', is_connected: false, data: '' },
    {
      icon: GithubIconRaw,
      text: 'Connect',
      is_connected: true,
      data: 'dehwyy'
    },
    { icon: GithubIconRaw, text: 'Connect', is_connected: false, data: '' }
  ]

  const [user, userStore] = useUserInfo({
    oneofKind: 'username',
    username: $page.params.username
  })
  $: userStore.set({
    getBy: { oneofKind: 'username', username: $page.params.username }
  })

  const clone = $user.data
</script>

<div class="flex flex-col gap-y-5">
  <SlideWindow>
    <svelte:fragment slot="heading">Integration</svelte:fragment>
    <svelte:fragment slot="content">
      {#each Integrations as integration}
        <article
          class={`${
            integration.is_connected
              ? 'dark:bg-surface-500 bg-surface-400'
              : 'dark:bg-surface-900 bg-surface-700'
          }  p-5 rounded-xl flex justify-between items-center`}
        >
          <p class="w-[36px] h-[36px] text-white">{@html integration.icon}</p>
          {#if !integration.is_connected}
            <button class="btn variant-glass-primary !text-secondary-50"
              >{integration.text}</button
            >
          {:else}
            <p class="font-[600] text-lg text-surface-900 dark:text-surface-50">
              {integration.data}
            </p>
          {/if}
        </article>
      {/each}</svelte:fragment
    >
  </SlideWindow>
  <SlideWindow max_height="1200px">
    <svelte:fragment slot="heading">About</svelte:fragment>
    <svelte:fragment slot="content">
      <About
        username={clone?.username || ''}
        pseudonym={clone?.pseudonym}
        photo={clone?.picture}
        bio={clone?.bio}
        selected_languages={clone?.languages}
        location={clone?.location}
      />
    </svelte:fragment>
  </SlideWindow>
</div>
