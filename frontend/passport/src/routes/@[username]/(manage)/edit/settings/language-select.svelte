<script lang="ts">
  import { ListBox, ListBoxItem } from '@skeletonlabs/skeleton'
  import { MostPopularLanguage } from '$lib/const'

  const Capitalize = (s: String) => {
    if (!s.length) return

    return s[0].toUpperCase() + s.slice(1)
  }

  const LanguageString = (language: string) => {
    return `${
      MostPopularLanguage.find((l) => l.language.toLowerCase() == language)
        ?.emoji_icon
    } ${Capitalize(language)}`
  }

  export let selectedLanguages: string[] = []
  export let languageFilter: string = ''

  $: unsensitive_to_case_language_filter = languageFilter.toLowerCase()

  let elemMovies: HTMLDivElement

  const multiColumnLeft = () => {
    let x = elemMovies.scrollWidth

    if (elemMovies.scrollLeft !== 0)
      x = elemMovies.scrollLeft - elemMovies.clientWidth
    elemMovies.scroll(x, 0)
  }

  const multiColumnRight = () => {
    let x = 0

    if (
      elemMovies.scrollLeft <
      elemMovies.scrollWidth - elemMovies.clientWidth - 1
    )
      x = elemMovies.scrollLeft + elemMovies.clientWidth
    elemMovies.scroll(x, 0)
  }
</script>

<article class="flex flex-col gap-y-5">
  <p class="text-center font-medium text-xl">Languages</p>
  <input
    class="input px-5"
    type="text"
    bind:value={languageFilter}
    placeholder="Russian, Japanese..."
  />

  <div
    class={`${
      !selectedLanguages.length
        ? 'max-h-0 opacity-0'
        : ' max-h-[45px] opacity-100'
    } transition-all duration-300 flex justify-between gap-x-5 items-center`}
  >
    <button
      disabled={!selectedLanguages.length}
      type="button"
      class="btn-icon-sm variant-glass-surface"
      on:click={multiColumnLeft}
    >
      {`<`}
    </button>
    <div
      bind:this={elemMovies}
      class="snap-x h-[40px] min-h-[5px] snap-mandatory scroll-smooth flex gap-x-5 select-none overflow-hidden"
    >
      {#each selectedLanguages as language}
        <p class="snap-align-none text-nowrap flex items-center">
          {LanguageString(language)}
        </p>
      {/each}
    </div>
    <button
      type="button"
      class="btn-icon-sm variant-glass-surface"
      on:click={multiColumnRight}
    >
      {`>`}
    </button>
  </div>

  <div class="h-[200px] overflow-y-scroll select-none mt-[-0.5rem]">
    <ListBox
      multiple
      active="variant-glass-primary"
    >
      {#each MostPopularLanguage as language}
        {#if language.language
          .toLowerCase()
          .match(unsensitive_to_case_language_filter)}
          <ListBoxItem
            bind:group={selectedLanguages}
            name="medium"
            value={language.language.toLowerCase()}
          >
            <span>{language.emoji_icon}</span>
            <span>{language.language}</span>
          </ListBoxItem>
        {/if}
      {/each}
    </ListBox>
  </div>
</article>
