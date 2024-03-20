<script lang="ts">
  import { fade } from "svelte/transition"
  import Input from "$lib/components/form/items/input.svelte"
  import { useConfirmEmailByCode, useSendConfirmationEmail } from "$lib/query/email"

  export let email: string

  const sendConfirmationEmail = useSendConfirmationEmail()
  const confirmEmailByCode = useConfirmEmailByCode()
  const emailSender = "makoto.web3@gmail.com"

  let emailSendTimes = 0
  let code = ""

  const SendEmail = () => {
    $sendConfirmationEmail.mutate({ email })
    emailSendTimes += 1
  }

  const ConfirmEmail = () => {
    $confirmEmailByCode.mutate({ code, email })
  }

  $: sendAtLeastOnce = emailSendTimes != 0
</script>

<section class="wrapper">
  <h3>Confirm your email address</h3>
  <p class="text-sm text-center">
    Check your inbox for an email from <span>{emailSender}</span> and click the link inside to confirm your email address
  </p>
  {#key sendAtLeastOnce}
    <button in:fade={{ duration: 150 }} on:click={SendEmail} class="btn variant-soft-primary">{sendAtLeastOnce ? "Resend" : "Send"} email</button>
  {/key}
  <div class={`${sendAtLeastOnce ? "max-h-[30px]" : "max-h-0"} overflow-hidden duration-300 transition-all`}>
    <p class="text-center text-success-500 font-bold">Email was sent! Please check your inbox!</p>
  </div>
  <hr class="mb-3" />
  <Input disabled={!sendAtLeastOnce} bind:value={code} label_text="Enter activation code" />
  <button disabled={!sendAtLeastOnce} on:click={ConfirmEmail} class="btn variant-filled-primary">Confirm</button>
</section>

<style lang="scss">
  h3 {
    @apply h3 text-center underline mb-5;
  }
  span {
    @apply font-medium;
  }

  .wrapper {
    @apply flex flex-col gap-y-5;
  }
</style>
