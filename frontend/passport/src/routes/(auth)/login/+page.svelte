<script lang="ts">
  import { page } from '$app/stores'
  import LoginForm from '$lib/components/form/login.svelte'
  import OAuthForm from '$lib/components/form/oauth.svelte'
  import { Toasts } from '$lib/utils/toast'
  import { onMount } from 'svelte'

  onMount(() => {
    const err = $page.url.searchParams.get('error')
    const errorDescription = $page.url.searchParams.get('error_desc')
    err && Toasts.error(errorDescription)
  })
</script>

<LoginForm />
<OAuthForm />
