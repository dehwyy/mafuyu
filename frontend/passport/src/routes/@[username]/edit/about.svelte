<script lang="ts">
  // import { Datepicker, themes } from "svelte-calendar"

  // let store: any
  // TODO: custom calendar theme, fix glitch effect, @see (https://6edesign.github.io/svelte-calendar)
  import LanguageSelector from "./settings/language-select.svelte"
  import ProfilePhoto from "./settings/profile-photo.svelte"
  import Username from "./settings/username.svelte"
  import Pseudonym from "./settings/pseudonym.svelte"
  import Bio from "./settings/bio.svelte"

  import CheckIconRaw from "$lib/assets/check.svg?raw"

  let selected_languages: string[] = []

  $: has_changes = false
</script>

<section class="col-span-2 flex flex-col gap-y-5 settings px-5">
  <div class={`${has_changes ? "bottom-5" : "-bottom-20"} fixed self-end  z-10 -mr-6 transition-all duration-200`}>
    <button class="w-[50px] h-[50px] variant-glass-primary btn rounded-full px-3 border border-primary-700">
      {@html CheckIconRaw}
    </button>
  </div>
  <div class="flex flex-col gap-y-7">
    <ProfilePhoto />
    <Username />
    <Pseudonym />
    <Bio />
  </div>
  <hr />
  <article class:absolute={false} class="flex items-center gap-x-10">
    <p>Birthday</p>
    <button class="btn variant-glass-primary max-w-full w-[200px]">Pick</button>
    <!--    <Datepicker bind:store defaultTheme={themes.dark} end={new Date()} let:key let:send let:receive>-->
    <!--      <button in:receive|local={{ key }} out:send|local={{ key }} class="btn variant-glass-primary mx-auto block px-32">Pick</button>-->
    <!--    </Datepicker>-->
  </article>
  <article class="flex items-center gap-x-10">
    <p>Location</p>
    <button class="btn variant-glass-primary max-w-full w-[200px]">Pick</button>
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
