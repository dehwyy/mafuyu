<script lang="ts">
  import { fade, fly } from "svelte/transition"
  import Input from "$lib/components/form/input.svelte"
  import { useConfirmEmailByCode, useSendConfirmationEmail } from "$lib/query/auth"

  export let email: string

  const sendConfirmationEmail = useSendConfirmationEmail()
  const confirmEmailByCode = useConfirmEmailByCode()
  const email_sender = "makoto.web3@gmail.com"

  let email_send_times = 0
  let code = ""

  const SendEmail = () => {
    $sendConfirmationEmail.mutate(email)
    email_send_times += 1
  }

  const ConfirmEmail = () => {
    $confirmEmailByCode.mutate({ code, email })
  }

  $: send_at_least_once = email_send_times != 0
</script>

<section class="flex flex-col gap-y-5">
  <h3 class="h3 text-center underline mb-5">Confirm your email address</h3>
  <p class="text-sm text-center">
    Check your inbox for an email from <span class="font-medium">{email_sender}</span> and click the link inside to confirm your email address
  </p>
  {#key send_at_least_once}
    <button in:fade={{ duration: 150 }} on:click={SendEmail} class="btn variant-soft-primary">{send_at_least_once ? "Resend" : "Send"} email</button>
  {/key}
  <div class={`${send_at_least_once ? "max-h-[30px]" : "max-h-0"} overflow-hidden duration-300 transition-all`}>
    <p class="text-center text-success-500 font-bold">Email was sent! Please check your inbox!</p>
  </div>
  <hr class="mb-3" />
  <Input label_text="Enter activation code" bind:value={code} disabled={!send_at_least_once} />
  <button disabled={!send_at_least_once} on:click={ConfirmEmail} class="btn variant-filled-primary">Confirm</button>
</section>
