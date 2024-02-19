<script lang="ts">
  export let image: string
  export let username: string
  export let pseudonym: string | undefined
</script>

<section class="mb-7">
  <div class="w-[175px] h-[175px] object-cover overflow-hidden rounded-full mb-4 border-2 mx-auto">
    <img alt="account_image" class="w-full h-full object-cover" src={image} />
  </div>
  <div class="text-center">
    <h4 class="h4">{username}</h4>
    {#if pseudonym}
      <h6 class="h6 opacity-50">{pseudonym}</h6>
    {/if}
  </div>
</section>
