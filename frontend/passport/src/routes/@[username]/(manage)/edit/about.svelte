<script lang="ts">
  import LanguageSelector from "../../(manage)/edit/settings/language-select.svelte"
  import ProfilePhoto from "../../(manage)/edit/settings/profile-photo.svelte"
  import Username from "../../(manage)/edit/settings/username.svelte"
  import Pseudonym from "../../(manage)/edit/settings/pseudonym.svelte"
  import Bio from "../../(manage)/edit/settings/bio.svelte"
  import CheckIconRaw from "$lib/assets/check.svg?raw"
  import CloseIconRaw from "$lib/assets/close.svg?raw"
  import Datepicker from "$lib/components/form/datepicker.svelte"
  import { pushState } from "$app/navigation"
  import InputWithLabel from "$lib/components/form/input.svelte"
  import { useEditUser } from "$lib/query/user"
  import { DevFallbackImages } from "$lib/const"
  import { useUserInfo } from "$lib/query/user"
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
  const [user, userStore] = useUserInfo({ oneofKind: "username", username: $page.params.username })
  $: userStore.set({ getBy: { oneofKind: "username", username: $page.params.username } })

  const Save = async () => {
    const userId = $user.data?.userId
    if (!userId) {
      Toasts.error("[INTERNAL]: <br/> User_id not found.")
      return
    }

    await $editUserMutation.mutateAsync({
      userId,
      updateUsername: initialUsername !== username,
      username,
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

  $: unchangedUsername = username === initialUsername

  $: has_changes =
    !unchangedUsername ||
    pseudonym !== initialPseudonym ||
    photo !== initialPhoto ||
    location !== initialLocation ||
    bio !== initialBio ||
    JSON.stringify(selected_languages) !== JSON.stringify(initialSelectedLanguages)

  let isPending = false
  let isRejected = false
</script>

<section class="col-span-2 flex flex-col gap-y-5 settings px-5">
  <div class={`${has_changes ? "bottom-5" : "-bottom-20"} fixed self-end  z-10 -mr-6 transition-all duration-200`}>
    <button
      disabled={(isPending || isRejected) && !unchangedUsername}
      on:click={() => Save()}
      class={`${
        isPending && !unchangedUsername
          ? "variant-glass-secondary border-blue-400"
          : isRejected && !unchangedUsername
            ? "variant-glass-error border-error-500"
            : "variant-glass-success border-success-500"
      } w-[50px] h-[50px]  btn rounded-full px-3 border `}>
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
    <Username bind:isPending bind:isRejected bind:username />
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
