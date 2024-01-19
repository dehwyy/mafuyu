<script lang="ts">
  // import { Datepicker, themes } from "svelte-calendar"

  // let store: any
  // TODO: custom calendar theme, fix glitch effect, @see (https://6edesign.github.io/svelte-calendar)

  import { ListBox, ListBoxItem } from "@skeletonlabs/skeleton"

  let valueMultiple: string[] = []
</script>

<section class="col-span-2 flex flex-col gap-y-5 settings px-5">
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
  <article class="flex flex-col gap-y-5">
    <p class="text-center">Languages</p>
    <input class="input" title="Input (text)" type="text" placeholder="Russian, Japanese..." />
    <ListBox multiple active="variant-glass-primary">
      <ListBoxItem bind:group={valueMultiple} name="medium" value="jap">Japanese</ListBoxItem>
      <ListBoxItem bind:group={valueMultiple} name="medium" value="rus">Russian</ListBoxItem>
      <ListBoxItem bind:group={valueMultiple} name="medium" value="eng">English</ListBoxItem>
    </ListBox>
  </article>
  <hr />
</section>

<style>
  .settings article > p {
    @apply font-[500] text-xl;
  }
</style>
