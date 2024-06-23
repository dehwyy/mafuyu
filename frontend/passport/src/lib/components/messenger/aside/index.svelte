<script lang="ts">
  import GearIconRaw from '$lib/assets/bars.svg?raw'

  import ChatPreview from './chat-preview.svelte'

  const TEST_avatar =
    'https://localhost:6996/v1/static/ba0cc45b-3249-4eda-9440-22a77e7f0d31.8add86fe-aaaf-4aa0-9dff-5e759641027b.webp'
</script>

<div class="relative">
  <aside>
    <section class="top">
      <div class="h-full cursor-pointer py-1">
        {@html GearIconRaw}
      </div>
      <div class="">
        <p>prob smth here</p>
      </div>
    </section>
    <section class="main">
      {#each Array(20) as _, i}
        <ChatPreview
          current={i == 0}
          name="baha"
          avatar_urL={TEST_avatar}
          lastMessageTimeUTC="17:04"
          lastMessage="valorant"
          lastMessageType=""
        />
      {/each}
    </section>
  </aside>
</div>

<style lang="scss">
  aside {
    max-height: calc(100vh - 64px);
    height: 100%;
    width: 100%;
    @apply flex flex-col overflow-scroll;
  }

  .top {
    height: 64px;
    z-index: 100;
    @apply flex items-center justify-between gap-x-3;
    @apply p-4;
    @apply fixed left-0 top-0 z-[51];
  }

  .main {
    @apply flex-1 px-3 py-2;
    @apply flex flex-col gap-y-5;
  }
</style>
