<script lang="ts">
  import {email as emailValidation, required} from "svelte-forms/validators"
  import { field, form } from 'svelte-forms'
  import { Routes } from '$lib/const'

  const email = field("email", "",  [emailValidation(), required()], {
  })
  const password= field("password", "", [])

  const f = form(email)

</script>

<main class="grid place-items-center min-h-[80vh] mt-10">
  <form class="max-w-[400px] w-full card p-10" action="?/login" method="POST">
    <h3 class="h3 text-center mb-10">Sign in to <span class="text-primary-400 underline">Mafuyu</span></h3>
    <input placeholder='email@example.com' name="email" type="email"  bind:value={$email.value} class={`input`} />
    <hr class="my-3"/>
    <input placeholder="password" type="password" name="password"  minlength="8" bind:value={$password.value} class="input mb-2" />
    <a href={Routes.RecoverPassword} class=" text-end text-sm">
      <p class="hover:text-primary-400 underline transition-all">Forgot password?</p>
    </a>
    <div class="w-2/3 mx-auto">
      <button disabled={!$f.valid} type="submit" class="variant-filled-primary btn w-full mt-8">Login</button>
    </div>
    <div class="mt-4">
      <p class="text-center">
        <span>New to <span class="text-primary-400">Mafuyu?</span></span>
        <a href={Routes.Create} class="underline hover:text-primary-400 transition-all">
          Create an account!
        </a>
      </p>
    </div>
  </form>
</main>