<script lang="ts">
  import { MostPopularLanguage } from "$lib/const"

  import LanguageIconRaw from "$lib/assets/language.svg?raw"
  export let languages: string[] = ["russian", "english", "japanese", "korean"]

  $: langs = MostPopularLanguage.filter(l => languages.includes(l.language.toLowerCase())).map(l => `${l.emoji_icon} ${l.language} `)

  const DisplayLanguage = (language: string, i: number) => {
    if (i == languages.length - 1) {
      return language
    }

    return language + ","
  }
</script>

<div class="flex gap-x-5">
  <p class="icon-sm">
    {@html LanguageIconRaw}
  </p>
  <p class="flex gap-x-5 -ml-0.5">
    <span>Languages:</span>
    <span class="flex flex-wrap gap-x-3">
      {#each langs as language, i}
        <span class="hover:underline cursor-default">{DisplayLanguage(language, i)}</span>
      {/each}
    </span>
  </p>
</div>
