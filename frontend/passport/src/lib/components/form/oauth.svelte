<script lang="ts">
  import GoogleIconRaw from "$lib/assets/google.svg?raw"
  import GithubIconRaw from "$lib/assets/github.svg?raw"
  import { useCreateOAuth2RedirectUrl } from "$lib/query/auth"

  const oauth2Redirect = useCreateOAuth2RedirectUrl()
</script>

<div class="relative my-7">
  <div class="absolute -top-2.5 left-1/2 -translate-x-1/2 text-sm bg-surface-800 px-3">or</div>
  <hr />
</div>
<div class="flex flex-col gap-y-3">
  <button on:click={() => $oauth2Redirect.mutate({ provider: "google" })} class="oauth-button">
    <span class="oauth-icon">
      {@html GoogleIconRaw}
    </span>
    <span>Continue with Google</span>
  </button>
  <button on:click={() => $oauth2Redirect.mutate({ provider: "github" })} class="oauth-button">
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
