<script lang="ts">
  import SocietyPeopleInformation from "./society-people-information.svelte"
  import Writer from "$lib/components/writer.svelte"
  import Post from "$lib/components/post.svelte"

  import CakeIconRaw from "$lib/assets/cake.svg?raw"
  import LocationIconRaw from "$lib/assets/location.svg?raw"
  import FriendsIconRaw from "$lib/assets/people.svg?raw"
  import PeopleGroupIconRaw from "$lib/assets/people-group.svg?raw"
  import LanguageIconRaw from "$lib/assets/language.svg?raw"
</script>

<div class="flex pt-6 gap-x-5 items-start">
  <section class="w-[500px] flex flex-col gap-y-4">
    <section class="card p-6 w-full">
      <ul class="flex flex-col gap-y-4">
        <li class="information">
          <div class="icon pb-0.5">
            {@html CakeIconRaw}
          </div>
          <p>
            Birthday: &nbsp;<span class="text-primary-500 underline">June 21</span>,
            <span class="text-secondary-500 underline">1999</span>
          </p>
        </li>
        <li class="information">
          <div class="icon">
            {@html LocationIconRaw}
          </div>
          <p>Location: &nbsp; <span class="text-primary-500 underline">Tokyo, Japan</span></p>
        </li>
        <hr />
        <li class="information">
          <SocietyPeopleInformation label="Friends" value={40} see_all_href="/">
            <div class="icon pt-0.5">
              {@html FriendsIconRaw}
            </div>
          </SocietyPeopleInformation>
        </li>
        <li class="information">
          <SocietyPeopleInformation label="Following" value={4} see_all_href="/">
            <div class="icon">
              {@html PeopleGroupIconRaw}
            </div>
          </SocietyPeopleInformation>
        </li>
        <hr />
        <li class="information !items-start">
          <div class="icon">
            {@html LanguageIconRaw}
          </div>
          <p class="flex gap-x-5 -ml-0.5">
            Languages: &nbsp;
            <span class="flex flex-wrap gap-x-3">
              {#each ["Russian", "English", "Japanese"] as language, i}
                <a href={`/${language}`} class="block hover:underline">
                  {language + (i !== 2 ? "," : "")}
                </a>
              {/each}
            </span>
          </p>
        </li>
      </ul>
    </section>
    <section class="card w-full">
      <Writer placeholder="What's new?" button_text="Create" />
    </section>
    <section class="w-full flex flex-col gap-y-6">
      <Post />
      <Post />
      <Post />
    </section>
  </section>
</div>

<style>
  .information {
    @apply flex gap-x-5 items-center;
  }

  .information .icon {
    @apply w-[24px] h-[24px];
  }
</style>
