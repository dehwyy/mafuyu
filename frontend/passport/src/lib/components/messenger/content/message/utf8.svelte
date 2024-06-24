<script lang="ts">
  import MessageLeftCornerRawSvg from '$lib/assets/message_leftcorner.svg?raw'
  import MessageRightCornerRawSvg from '$lib/assets/message_rightcorner.svg?raw'

  // todo: highlight code blocks parser
  export let wasSendByCurrentUser: boolean
  export let name: string
  export let content: string
  export let messageTime: string
  export let userColor = 'lightblue'

  const msgDynUI = {
    cornerSvg: wasSendByCurrentUser
      ? MessageRightCornerRawSvg
      : MessageLeftCornerRawSvg,
    boxDynClasses: wasSendByCurrentUser ? 'flex-row-reverse' : 'flex-row',
    contentDynClasses: wasSendByCurrentUser
      ? '!rounded-br-none'
      : '!rounded-bl-none',
    nameDynStyle: `color:${userColor};`
  }
</script>

<!--  flexbox to align `messageCorner` and `messageBox`-->
<div class={`flex items-end ${msgDynUI.boxDynClasses}`}>
  <!--    `messageCornerSvg`-->
  {@html msgDynUI.cornerSvg}
  <!--    `messageBox`-->
  <div class={`content ${msgDynUI.contentDynClasses}`}>
    {#if !wasSendByCurrentUser}
      <p style={msgDynUI.nameDynStyle}>
        {name}
      </p>
    {/if}
    <div class="flex gap-x-5 items-end">
      <p>{content}</p>
      <p class="relative top-1 text-neutral-400 text-[12px] break-keep">
        {messageTime}
      </p>
    </div>
  </div>
</div>

<style lang="scss">
  .content {
    @apply bg-surface-800 rounded-2xl px-4 py-2 text-sm;
    @apply flex flex-col;
    max-width: 30rem;
    word-break: break-word;

    & p:first-child {
      @apply select-none;
    }
  }
</style>
