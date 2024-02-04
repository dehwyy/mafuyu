<script lang="ts">
  import GoogleIconRaw from "$lib/assets/google.svg?raw"
  import GithubIconRaw from "$lib/assets/github.svg?raw"
  import { useMutation } from "@sveltestack/svelte-query"
  import { Routes } from "$lib/utils/typed-fetch"


  const oauth_redirect = useMutation(
    async (provider: string) => {
      const response = await Routes["oauth"].fetch({
        provider
      })


      // const response = await fetch("/api/v1/oauth", {
      //   method: "POST",
      //   body: JSON.stringify({ provider }),
      // })

      // window.location.href = (await response.json())["redirect_url"]!
      window.location.href = response.redirect_url
    },
    {
      onError: e => {
        console.log(e)
      },
    },
  )
</script>

<div class="relative my-7">
  <div class="absolute -top-2.5 left-1/2 -translate-x-1/2 text-sm bg-surface-800 px-3">or</div>
  <hr />
</div>
<div class="flex flex-col gap-y-3">
  <button class="oauth-button">
    <span class="oauth-icon">
      {@html GoogleIconRaw}
    </span>
    <span>Continue with Google</span>
  </button>
  <button on:click={() => $oauth_redirect.mutate("github")} class="oauth-button">
    <span class="oauth-icon">
      {@html GithubIconRaw}
    </span>
    <span>Continue with Github</span>
  </button>
</div>

<style>
  .oauth-button {
    @apply btn variant-glass-primary w-full relative;
  }
  .oauth-icon {
    @apply h-[20px] absolute left-5;
  }
</style>
