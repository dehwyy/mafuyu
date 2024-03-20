<script lang="ts">
  import RegistrationForm from "$lib/components/form/registration.svelte"
  import ConfirmationMail from "$lib/components/form/confirmation-mail.svelte"
  import { useSignUp } from "$lib/query/auth"

  const signUp = useSignUp()

  let user_email = ""
  let step = 1

  let setNextStep = (email: string, username: string, password: string) => {
    step += 1
    user_email = email
    $signUp.mutate({
      email,
      username,
      password,
    })
  }
</script>

{#if step === 1}
  <RegistrationForm onFormSubmit={setNextStep} />
{:else if step === 2}
  <ConfirmationMail email={user_email} />
{/if}
