<script lang="ts">
  import { MessageType } from '$lib/const'

  import Message from './message.svelte'

  const im =
    'https://localhost:6996/v1/static/ba0cc45b-3249-4eda-9440-22a77e7f0d31.8add86fe-aaaf-4aa0-9dff-5e759641027b.webp'
</script>

<div>
  <main class="">
    <div class="messages-area">
      {#each Array(100) as _, i}
        <Message
          name="dehwyy"
          userImage={im}
          wasSendByCurrentUser={i % 5 === 0}
          messageTime="18:03"
          messageType={MessageType.Text}
          messageContent="it's dehwyy!! I am developer of Mafuyu!"
        />
      {/each}
    </div>
    <!--    user input area-->
    <div class="input-area">
      <input class="input" />
    </div>
  </main>
</div>

<style lang="scss">
  main {
    max-height: calc(100vh - 64px);
    @apply flex flex-col gap-y-5;
  }

  .messages-area {
    overflow-y: scroll;
    overflow-x: hidden;
    @apply px-5 py-3 flex-1;
    @apply flex flex-col items-start gap-y-3;
  }

  .input-area {
    min-height: 100px;
    @apply px-5;
  }
</style>
