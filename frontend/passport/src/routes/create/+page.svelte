<script lang="ts">
  import { email as emailValidation, matchField, required, pattern } from 'svelte-forms/validators'
  import { field, form } from 'svelte-forms'
  import { Routes } from '$lib/const'
  import { onMount } from 'svelte'
  import { getToastStore, type  ToastSettings } from '@skeletonlabs/skeleton'

  const InputErrors = {
    "username.required": "Username is required",
    'username.pattern': "Username is not valid ASCII word",
    "email.not_an_email": "Email should be an email",
    "email.required": "Email is required",
    "password.required": "Password is required",
    "password_confirm.required": "Confirm your password",
    "password_confirm.match_field": "Passwords doesn't match"
  }

  const username = field("username", "", [required(), pattern(/[a-zA-z0-9]+/)])
  const email = field("email", "",  [emailValidation(), required()])
  const password= field("password", "", [required()])
  const password_confirm = field('password_confirm', "", [matchField(password), required()])

  const toastStore = getToastStore()
  const f = form(username, email, password, password_confirm)
  onMount(() => {
    f.validate()
  })

  const create_button_click = (e:  MouseEvent) => {
    // if form is valid
    if ($f.valid) {
      // invoke form submit
      return
    }

    e.preventDefault()

    const current_errors: Record<string, null | string> = {
      "username": null,
      "email": null,
      "password": null,
      "password_confirm": null
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
        background: "bg-gradient-to-br variant-gradient-primary-secondary",
      }
      toastStore.trigger(toast)
    }
  }


</script>

<main class="grid place-items-center min-h-[80vh] mt-10">
  <form class="max-w-[400px] w-full card p-10" action="?/create" method="POST">
    <h3 class="h3 mb-10 text-center">New <span class="text-primary-400 underline">Mafuyu</span> Account</h3>
    <input placeholder='Mafuyu Username' type="text" name="username" maxlength="30" autocomplete="off" bind:value={$username.value} class={`input`} />
    <hr class="my-3"/>
    <input placeholder='email@example.com' type="email" name="email" autocomplete="off" bind:value={$email.value} class={`input`} />
    <hr class="my-3"/>
    <input placeholder="password" type="password"  minlength="8" name="password" bind:value={$password.value} class="input" />
    <hr class="my-3"/>
    <input placeholder="confirm your password" type="password" bind:value={$password_confirm.value} class="input mb-2" />
    <div class="w-2/3 mx-auto">
      <button on:click={create_button_click} type="submit" class="variant-filled-primary btn w-full mt-8 transition-all">Create</button>
    </div>
    <div class="mt-4">
      <p class="text-center">
        <span>Already have an account?</span>
        <a href={Routes.Login} class="underline hover:text-primary-400 transition-all">
          Sign in!
        </a>
      </p>
    </div>
  </form>
</main>