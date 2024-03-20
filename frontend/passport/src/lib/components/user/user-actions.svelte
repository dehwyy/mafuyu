<script lang="ts">
  import { fade } from "svelte/transition"
  import { popup, type PopupSettings } from "@skeletonlabs/skeleton"
  import useUserProfileActions, { UserStatus } from "$lib/hooks/use-user-profile-actions"
  import { writable } from "svelte/store"

  let userIdProp: string
  let usernameProp: string
  export { userIdProp as userId, usernameProp as username }

  const userId = writable(userIdProp)
  $: userId.set(userIdProp)

  const username = writable(usernameProp)
  $: username.set(usernameProp)

  const { options, status } = useUserProfileActions({ userId, username })

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
<div class={`${$status === UserStatus.None ? "!max-h-[0px]" : "max-h-[30px]"} transition-all grid duration-500 mb-5 text-center text-lg`}>
  {#if $status === UserStatus.Friend}
    <p in:fade={{ duration: 100 }} class="text-green-500 underline">Friend</p>
  {:else if $status === UserStatus.Followed}
    <p in:fade={{ duration: 100 }} class="text-blue-500 underline">Followed</p>
  {:else if $status === UserStatus.FollowedToYou}
    <p in:fade={{ duration: 100 }} class="text-blue-500 underline">Followed to you</p>
  {:else if $status === UserStatus.Blocked}
    <p in:fade={{ duration: 100 }} class="text-red-500 underline">Blocked</p>
  {:else}
    <div class="h-[28px]" />
  {/if}
</div>
<hr />
<button
  disabled={!$options.length}
  use:popup={user_actions_popup}
  class:!border-primary-500={is_open}
  class="btn hover:variant-soft w-full font-bold text-lg border border-transparent my-2">Actions</button>
<div data-popup="user_actions" class="card border border-surface-600 z-10">
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
