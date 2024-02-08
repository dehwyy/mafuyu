<script lang="ts">
  import { page } from "$app/stores"
  import Error404 from "$lib/assets/404.svg?raw"
</script>

{#if $page.status === 404}
  <section class="min-h-[90vh] grid place-items-center">
    <article class="min-w-[200px]">
      <div class="px-5 mt-[-8rem] sm:mt-0">
        {@html Error404}
      </div>
      <div class="p-5 text-center flex flex-col gap-y-5 sm:mt-5">
        <h3 class="h3">The stuff you were looking for doesn't exist</h3>
        <button on:click={() => history.back()} class="btn variant-filled-surface max-w-full min-w-[175px] w-1/3 mx-auto"> Go back </button>
      </div>
    </article>
  </section>
{:else}
  <div>error occurred</div>
{/if}
