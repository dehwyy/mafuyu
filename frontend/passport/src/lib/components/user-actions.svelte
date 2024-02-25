<script lang="ts">

  import { popup, type PopupSettings } from "@skeletonlabs/skeleton"
  import useUserProfileActions from "$lib/hooks/use-user-profile-actions"
  import { writable } from "svelte/store"

  let userIdProp: string
  let usernameProp: string
  export { userIdProp as userId, usernameProp as username }

  const userId = writable(userIdProp)
  $: userId.set(userIdProp)

  const username = writable(usernameProp)
  $: username.set(usernameProp)

  const options = useUserProfileActions({userId, username})

  $: innerWidth = 0
  $: placement = (innerWidth < 1023 ? "bottom" : "right") as "bottom" | "right"
  $: is_open = false

  $: user_actions_popup = {
    placement,
    target: "user_actions",
    event: "click",
    state: e => {
      is_open = e.state
    },
  } satisfies PopupSettings
</script>

<svelte:window bind:innerWidth />
<button
  disabled={!$options.length}
  use:popup={user_actions_popup}
  class:!border-primary-500={is_open}
  class="btn hover:variant-soft w-full font-bold text-lg border border-transparent">Actions</button>
<div data-popup="user_actions" class="card border border-surface-600">
  <ul class="list select-none py-3 px-0.5">
    {#each $options as option}
      <li>
        <button
          on:click={option.onClickAction}
          class="w-full flex rounded-container-token pl-5 pr-7 py-3 cursor-pointer dark:hover:bg-surface-700 hover:surface-200 transition-all">
          <span class="icon-sm block mr-3">
            {@html option.icon}
          </span>
          <span class="font-600">
            {option.getText()}
          </span>
        </button>
      </li>
    {/each}
  </ul>
</div>
