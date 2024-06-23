<script lang="ts">
  import Card from '$lib/components/card.svelte'

  // state
  export let current: boolean

  // data
  export let name: string
  export let avatar_urL: string
  export let lastMessageTimeUTC: string
  export let lastMessage: unknown
  export let lastMessageType: string
</script>

<Card
  disabledRgb={!current}
  class="cursor-pointer hover:!bg-surface-500 transition-all"
>
  <div class="preview">
    <div class="image">
      <img
        src={avatar_urL}
        alt="avatar"
      />
    </div>
    <div class="content">
      <div class="message">
        <p class="font-bold">{name}</p>
        <p class="text-sm text-neutral-400">{lastMessage}</p>
      </div>
      <div class="time">
        <p>{lastMessageTimeUTC}</p>
      </div>
    </div>
  </div>
</Card>

<style lang="scss">
  .preview {
    @apply flex items-center px-3 py-2 gap-x-3;
  }

  .image {
    --image-size: 50px;
    width: var(--image-size);
    height: var(--image-size);
    @apply overflow-hidden rounded-full;
    & img {
      @apply h-full w-full object-cover;
    }
  }

  .content {
    @apply flex-1 flex justify-between;
    & .message {
      @apply flex flex-col;
    }

    & .time {
      @apply mt-1;
    }
  }
</style>
