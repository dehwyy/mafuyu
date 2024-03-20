<script lang="ts">
  import { Textarea } from "$lib/components/form/items"
  export let bio = "Hello! My name is Egor. My dreams are becoming Google developer and visiting Japan!"

  let is_focused = false
</script>

<article
  class={`${
    is_focused ? "dark:border-primary-500 border-primary-400" : "dark:border-surface-500 border-surface-300 bg-surface-200"
  } w-full border-[3px] rounded-3xl  dark:bg-surface-700 transition-all text-sm font-medium`}>
  <Textarea bind:text={bio} bind:is_focused placeholder="Short bio" padding="1rem" />
</article>
