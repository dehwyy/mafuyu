<script lang="ts">
  import InputWithLabel from "$lib/components/form/input.svelte"
  import { useUserInfo } from "$lib/query/user"
  export let username = ""
  let debounced = username
  let debounceTimeout = 0
  export let isPending = false
  export let isRejected = false

  const [user, userStore] = useUserInfo({ oneofKind: "username", username })
  $: userStore.set({ getBy: { oneofKind: "username", username: debounced } })

  $: {
    isPending = true
    clearTimeout(debounceTimeout)

    debounceTimeout =
      typeof window !== "undefined"
        ? window.setTimeout(() => {
            debounced = username
          }, 300)
        : 0
  }

  $: {
    isPending = $user.isFetching
    if (!$user.isFetching) {
      isRejected = !(/^[a-zA-Z0-9_.]+$/.exec(debounced) && !$user.data)
    } else {
      isRejected = false
    }
  }
</script>

<article class="w-full">
  <InputWithLabel bind:value={username} label_text="Username" />
</article>
