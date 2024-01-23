<script lang="ts">
  import { email as emailValidation, matchField, required, pattern } from "svelte-forms/validators"
  import { field, form } from "svelte-forms"
  import { Routes } from "$lib/const"
  import { onMount } from "svelte"
  import { getToastStore, type ToastSettings } from "@skeletonlabs/skeleton"

  const InputErrors = {
    "username.required": "Username is required",
    "username.pattern": "Username is not valid ASCII word",
    "email.not_an_email": "Email should be an email",
    "email.required": "Email is required",
    "password.required": "Password is required",
  }

  const username = field("username", "", [required(), pattern(/[a-zA-z0-9]+/)])
  const email = field("email", "", [emailValidation(), required()])
  const password = field("password", "", [required()])

  const toastStore = getToastStore()
  const f = form(username, email, password)

  export let onFormSubmit: () => void

  onMount(() => {
    f.validate()
  })

  const create_button_click = (e: MouseEvent) => {
    // if form is valid
    if ($f.valid) {
      onFormSubmit()
      // invoke form submit
      return
    }

    e.preventDefault()

    const current_errors: Record<string, null | string> = {
      username: null,
      email: null,
      password: null,
      password_confirm: null,
    }

    for (const error of $f.errors) {
      const error_message = InputErrors[error as keyof typeof InputErrors]

      // cannot find proper error_message -> skip
      if (!error_message) continue

      const field = error.split(/[._]/)[0]

      // field already has error => skip
      if (!error_message || current_errors[field]) continue

      //
      current_errors[field] = error_message

      const toast: ToastSettings = {
        message: error_message,
        timeout: 5000,
        background: "variant-glass-primary",
      }
      toastStore.trigger(toast)
    }
  }
</script>

<form class="" action="?/create" method="POST">
  <h3 class="h2 sm:h3 mb-10 text-center">New <span class="text-primary-400 underline">Mafuyu</span> Account</h3>
  <div class="flex flex-col gap-y-5">
    <input placeholder="Mafuyu Username" type="text" name="username" maxlength="30" autocomplete="off" bind:value={$username.value} class={`input`} />
    <input placeholder="email@example.com" type="email" name="email" autocomplete="off" bind:value={$email.value} class={`input`} />
    <input placeholder="password" type="password" minlength="8" name="password" bind:value={$password.value} class="input" />
  </div>
  <hr class="my-3" />
  <div class="w-full mx-auto">
    <button on:click={create_button_click} type="submit" class="variant-filled-primary btn w-full my-4 transition-all">Create</button>
  </div>
  <div class="">
    <p class="text-center">
      <span>Already have an account?</span>
      <a href={Routes.Login} class="underline hover:text-primary-400 transition-all"> Sign in! </a>
    </p>
  </div>
</form>
