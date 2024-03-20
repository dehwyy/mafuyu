<script lang="ts">
  import ResetPasswordIconRaw from "$lib/assets/reset-lock.svg?raw"
  import { getToastStore, type ToastSettings } from "@skeletonlabs/skeleton"
  import { Routes } from "$lib/const"
  import OptionsPopup from "$lib/components/popups/options.svelte"

  const toast_store = getToastStore()
  const toast: ToastSettings = {
    message: "Email was sent to your inbox! Check your email for next steps!",
    background: "variant-glass-primary",
  }

  const popup_options = [{ icon: ResetPasswordIconRaw, text: "reset password", action: () => toast_store.trigger(toast) }]

  import { email as emailValidation, required } from "svelte-forms/validators"
  import { field, form } from "svelte-forms"

  const email = field("email", "", [emailValidation(), required()], {})
  const password = field("password", "", [])

  const f = form(email)
</script>

<form action="?/login" method="POST">
  <h3 class="h2 sm:h3 text-center mb-10">Sign in to <span class="text-primary-400 underline">Mafuyu</span></h3>
  <input placeholder="email@example.com" name="email" type="email" bind:value={$email.value} class={`input`} />
  <hr class="my-3" />
  <input placeholder="password" type="password" name="password" minlength="8" bind:value={$password.value} class="input mb-2" />
  <div class="w-fit ml-auto pr-5">
    <OptionsPopup unique_target_name="forgot-password-popup" options={popup_options}>
      <button on:click|preventDefault class="hover:text-primary-400 underline transition-all"> Forgot password? </button>
    </OptionsPopup>
  </div>
  <div class="w-2/3 mx-auto">
    <button disabled={!$f.valid} type="submit" class="variant-filled-primary btn w-full my-4">Login</button>
  </div>
  <div class="">
    <p class="text-center">
      <span>New to <span class="text-primary-400">Mafuyu?</span></span>
      <a href={Routes.Create} class="underline hover:text-primary-400 transition-all"> Create an account! </a>
    </p>
  </div>
</form>
