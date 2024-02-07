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
  import { user_store } from "$lib/stores/user"
  import { onMount } from "svelte"

  import InputWithLabel from "$lib/components/form/input.svelte"
  export let location: string
  const initialLocation = location

  export let photo =
    "https://sun9-28.userapi.com/impg/nfm26MjdU4tRW3N-OwRaiLh996CjCTLh0vu8Dg/ENC1jP7-KJw.jpg?size=1347x1346&quality=95&sign=ed76ada3e9318d6d216d6b845421f402&type=album"
  const initialPhoto = photo

  export let username: string
  const initialUsernmae = username

  export let pseudonym = ""
  const initiaPseudonym = pseudonym

  export let bio = ""
  const initialBio = bio

  export let selected_languages: string[] = []
  const initialSelctedLanguages = [...selected_languages]

  const SaveAll = useMutation(
    async () => {
      const user_id = $user_store?.id
      if (user_id === undefined) {
          window.location.href = "/"
      }

      await Routes["user/edit"].fetch({
        userId: user_id!,
        pseudonym,
        languages: selected_languages,
        bio,
        birthday: undefined,
        location: undefined,
        picture: photo
      })
    },
    {
      mutationKey: ["edit.user"],
    },
  )
  const DiscardAll = () => {
    console.log(selected_languages, initialSelctedLanguages)
    pseudonym = initiaPseudonym
    selected_languages = initialSelctedLanguages
    bio = initialBio
    photo = initialPhoto
    username = initialUsernmae
  }

  $: has_changes = username !== initialUsernmae || pseudonym !== initiaPseudonym || initialPhoto !== photo ||
  bio !== initialBio || JSON.stringify(selected_languages) !== JSON.stringify(initialSelctedLanguages)

</script>

<section class="col-span-2 flex flex-col gap-y-5 settings px-5">
  <div class={`${has_changes ? "bottom-5" : "-bottom-20"} fixed self-end  z-10 -mr-6 transition-all duration-200`}>
    <button on:click={() => $SaveAll.mutate()} class="w-[50px] h-[50px] variant-glass-success btn rounded-full px-3 border border-success-500">
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
  <article class:absolute={false} class="flex items-center gap-x-10">
    <p>Birthday</p>
    <button class="btn variant-glass-primary max-w-full w-[200px]">Pick</button>
    <!--    <Datepicker bind:store defaultTheme={themes.dark} end={new Date()} let:key let:send let:receive>-->
    <!--      <button in:receive|local={{ key }} out:send|local={{ key }} class="btn variant-glass-primary mx-auto block px-32">Pick</button>-->
    <!--    </Datepicker>-->
  </article>
  <article class="flex items-center gap-x-10 pt-2">
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
