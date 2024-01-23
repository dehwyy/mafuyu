<script lang="ts">
  import RegistrationForm from "$lib/components/form/registration-form.svelte"
  import ConfirmationMail from "$lib/components/form/confirmation-mail.svelte"

  let step = 1
  let set_next_step = () => {
    step += 1
  }
</script>

{#if step === 1}
  <RegistrationForm onFormSubmit={set_next_step} />
{:else if step === 2}
  <ConfirmationMail />
{/if}
