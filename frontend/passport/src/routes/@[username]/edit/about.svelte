<script lang="ts">
  import LanguageSelector from "./settings/language-select.svelte"
  import ProfilePhoto from "./settings/profile-photo.svelte"
  import Username from "./settings/username.svelte"
  import Pseudonym from "./settings/pseudonym.svelte"
  import Bio from "./settings/bio.svelte"
  import CheckIconRaw from "$lib/assets/check.svg?raw"
  import CloseIconRaw from "$lib/assets/close.svg?raw"
  import { useMutation } from "@sveltestack/svelte-query"
  import { Routes } from "$lib/utils/typed-fetch"
  import { authed_user_store, dyn_user_store } from "$lib/stores/user"
  import Datepicker from "$lib/components/form/datepicker.svelte"
  import { replaceState, invalidateAll, pushState } from "$app/navigation"
  import InputWithLabel from "$lib/components/form/input.svelte"
  import { useEditUser } from "$lib/query/user"
  import { DevFallbackImages } from "$lib/const"
  import { useUserProfile } from "$lib/query/profile"
  import { page } from "$app/stores"
  import { Toasts } from "$lib/utils/toast"

  export let location = ""
  let initialLocation = location

  export let photo = DevFallbackImages.HorizontalOriented as string
  let initialPhoto = photo

  export let username: string
  let initialUsername = username

  export let pseudonym = ""
  let initialPseudonym = pseudonym

  export let bio = ""
  let initialBio = bio

  export let selected_languages: string[] = []
  let initialSelectedLanguages = [...selected_languages]

  let is_datepicker_open = false

  const editUserMutation = useEditUser()
  const user = useUserProfile($page.params.username)

  const Save = async () => {
    const userId = $user.data?.userId
    if (!userId) {
      Toasts.error("[INTERNAL]: <br/> User_id not found.")
      return
    }

    if (userId === $authed_user_store?.id) {
      authed_user_store.set({
        id: userId,
        username,
      })
    }

    await $editUserMutation.mutateAsync({
      userId,
      username: initialUsername !== username ? username : undefined,
      location,
      picture: photo,
      pseudonym,
      bio,
      birthday: undefined,
      languages: selected_languages,
    })

    pushState("/@" + username + "/edit", "")
    if (username !== initialUsername) {
      window.location.reload()
    }

    if ($editUserMutation.isError) {
      return
    }

    initialUsername = username
    initialPseudonym = pseudonym
    initialSelectedLanguages = [...selected_languages]
    initialBio = bio
    initialLocation = location
    initialPhoto = photo
  }

  const DiscardAll = () => {
    console.log(selected_languages, initialSelectedLanguages)
    pseudonym = initialPseudonym
    selected_languages = [...initialSelectedLanguages]
    bio = initialBio
    photo = initialPhoto
    username = initialUsername
  }

  $: has_changes =
    username !== initialUsername ||
    pseudonym !== initialPseudonym ||
    photo !== initialPhoto ||
    location !== initialLocation ||
    bio !== initialBio ||
    JSON.stringify(selected_languages) !== JSON.stringify(initialSelectedLanguages)
</script>

<section class="col-span-2 flex flex-col gap-y-5 settings px-5">
  <div class={`${has_changes ? "bottom-5" : "-bottom-20"} fixed self-end  z-10 -mr-6 transition-all duration-200`}>
    <button on:click={() => Save()} class="w-[50px] h-[50px] variant-glass-success btn rounded-full px-3 border border-success-500">
      {@html CheckIconRaw}
    </button>
  </div>
  <div class={`${has_changes ? "bottom-5" : "-bottom-20"} fixed self-end z-10 mr-10 transition-all duration-200 delay-100`}>
    <button on:click={() => DiscardAll()} class="w-[50px] h-[50px] variant-glass-error btn rounded-full px-3 border border-error-500">
      {@html CloseIconRaw}
    </button>
  </div>

  <div class="flex flex-col gap-y-7">
    <ProfilePhoto bind:photo />
    <Username bind:username />
    <Pseudonym bind:pseudonym />
    <Bio bind:bio />
  </div>
  <hr />
  <article class:absolute={false} class="flex items-center gap-x-10 opacity-10 line-through select-none">
    <p>Birthday</p>
    <button on:click|stopPropagation={() => {}} class="btn variant-glass-primary max-w-full w-[200px]">Pick</button>
    <Datepicker bind:is_open={is_datepicker_open} />
  </article>
  <article class="flex items-center gap-x-10 pt-4">
    <p>Location</p>
    <InputWithLabel bind:value={location} label_text="Country, city" />
  </article>
  <hr />
  <LanguageSelector bind:selected_languages />
  <hr />
</section>

<style>
  .settings article > p {
    @apply font-[500] text-xl;
  }
</style>
