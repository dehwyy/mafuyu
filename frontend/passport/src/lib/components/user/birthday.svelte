<script lang="ts">
  import CakeIconRaw from "$lib/assets/cake.svg?raw"

  export let date = new Date("1998-11-21")

  $: month_and_day = new Date(date).toLocaleDateString("en", {
    month: "long",
    day: "numeric",
  })
  $: year = new Date(date).getFullYear()
</script>

<div class="flex gap-x-5">
  <p class="icon-sm pb-0.5">
    {@html CakeIconRaw}
  </p>
  <p>
    Birthday: &nbsp;<span class="text-primary-500 underline">{month_and_day}</span>,
    <span class="text-secondary-500 underline">{year}</span>
  </p>
</div>
