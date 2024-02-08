<script lang="ts">
  import { MostPopularLanguage } from "$lib/const"
  import { ListBox, ListBoxItem } from "@skeletonlabs/skeleton"

  const Capitalize = (s: String) => {
    if (!s.length) return

    return s[0].toUpperCase() + s.slice(1)
  }

  export let selected_languages: string[] = []
  export let language_filter: string = ""

  $: unsensitive_to_case_language_filter = language_filter.toLowerCase()
  // TODO: maybe should implement Carousel instead of simple scroll;
  //  @see (https://www.skeleton.dev/elements/scroll-containers#carousels)
</script>

<article class="flex flex-col gap-y-5">
  <p class="text-center font-medium text-xl">Languages</p>
  <input class="input px-5" type="text" bind:value={language_filter} placeholder="Russian, Japanese..." />
  <div class={`${!selected_languages.length ? "max-h-0 opacity-0" : " max-h-[45px] opacity-100/i"} transition-all duration-300`}>
    <div class="snap-x h-[40px] min-h-[5px] scroll-px-6 snap-mandatory scroll-smooth flex gap-x-5 select-none overflow-x-auto px-5">
      {#each selected_languages as language}
        <p class="snap-align-none">{Capitalize(language)}</p>
      {/each}
    </div>
  </div>
  <div class="h-[200px] overflow-y-scroll select-none mt-[-0.5rem]">
    <ListBox multiple active="variant-glass-primary">
      {#each MostPopularLanguage as language}
        {#if language.language.toLowerCase().match(unsensitive_to_case_language_filter)}
          <ListBoxItem bind:group={selected_languages} name="medium" value={language.language.toLowerCase()}>
            <span>{language.emoji_icon}</span>
            <span>{language.language}</span>
          </ListBoxItem>
        {/if}
      {/each}
    </ListBox>
  </div>
</article>
