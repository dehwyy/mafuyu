<script lang="ts">
  import { page } from '$app/stores'

  $: username = $page.params.username
</script>

<section class="my-7">
  <h2 class="text-center text-3xl">{username}'s Github Integration</h2>
</section>
