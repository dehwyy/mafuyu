<script lang="ts">
	import { user_store } from '$lib/stores/user';
  import { ListBox, ListBoxItem } from "@skeletonlabs/skeleton"
  import { Routes } from "$lib/const"
  import { page } from "$app/stores"
  import type { LayoutData } from "./$types"

  const navigation = {
    "/": "Overview",
    "/statistics": "Statistics",
    "/edit": "Settings",
  }

  let current_navigation: keyof typeof navigation = "/"
  $: {
    current_navigation = ($page.route.id?.split("/@[username]").at(-1) || "/") as keyof typeof navigation
  }

  export let data: LayoutData
  $: is_current_user = data.userId === $user_store?.id
  $: image = is_current_user ? ($user_store?.picture || data?.picture || "/images/r.jpg") : (data?.picture || "/images/r.jpg")
  $: username = is_current_user ? ($user_store?.username || data?.username) : data?.username
  $: pseudonym = is_current_user ? ($user_store?.pseudonym || data?.pseudonym) : data?.pseudonym
</script>

<div style="margin-right: calc(100% - 100vw);" class="max-w-[1450px]">
  <div class="w-full lg:w-[800px] px-5 lg:px-0 mx-auto pt-14 flex flex-col lg:flex-row gap-8 gap-x-16">
    <nav class="min-w-[230px]">
      <section class="mb-7">
        <div class="w-[175px] h-[175px] object-cover overflow-hidden rounded-full mb-4 border-2 mx-auto">
          <img alt="account_image" src={image} />
        </div>
        <div class="text-center">
          <h4 class="h4">{username}</h4>
          {#if data.pseudonym}
            <h6 class="h6 opacity-50">{pseudonym}</h6>
          {/if}
        </div>
      </section>
      <ListBox>
        {#each Object.entries(navigation) as entry}
          <a href={Routes.Account + `@${username}` + entry[0]} class="text-center text-lg mb-2 block">
            <ListBoxItem bind:group={current_navigation} name="theme" value={entry[0]}>{entry[1]}</ListBoxItem>
          </a>
        {/each}
      </ListBox>
    </nav>
    <main class="w-full pb-10">
      <slot />
    </main>
  </div>
</div>
