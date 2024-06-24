<script lang="ts">
  import { MessageType } from '$lib/const'

  import MessageUTF8 from './message/utf8.svelte'

  export let wasSendByCurrentUser = false
  export let name: string = ''
  export let userImage: string | undefined = undefined
  export let messageType: MessageType
  export let messageContent: any = ''
  export let messageTime: string
  export let userColor: string = 'lightblue'

  const messageComputedStyles = {
    wrapperDynClasses: wasSendByCurrentUser
      ? 'flex-row-reverse self-end'
      : 'flex-row self-start'
  }
</script>

<section class={messageComputedStyles.wrapperDynClasses}>
  <!--  user picture-->
  {#if !wasSendByCurrentUser}
    <div class="message__user-image">
      <img
        src={userImage}
        alt="user_image"
      />
    </div>
  {/if}
  <!--  message content -->
  {#if messageType === MessageType.Text}
    <MessageUTF8
      {wasSendByCurrentUser}
      {userColor}
      {name}
      {messageTime}
      content={messageContent}
    />
  {/if}
</section>

<style lang="scss">
  section {
    @apply flex gap-x-1;
  }

  .message {
    &__user-image {
      @apply h-[40px] w-[40px] rounded-full overflow-hidden pointer-events-none select-none;
      & img {
        @apply w-full h-full object-cover;
      }
    }

    &__content {
      @apply bg-surface-800 rounded-xl px-4 py-1 text-sm;
      @apply flex flex-col;
      &__name {
        @apply select-none;
      }
    }
  }
</style>
