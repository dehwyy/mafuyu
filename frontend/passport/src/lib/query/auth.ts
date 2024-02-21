import { useMutation } from "@sveltestack/svelte-query"
import { GrpcWebClient } from "@makoto/grpc/web"
import { Toasts } from "$lib/utils/toast"

export const useSendConfirmationEmail = () =>
  useMutation(
    async (email: string) => {
      try {
        const r = await GrpcWebClient.sendEmailVerificationCode({
          email,
        })
        return r.response
      } catch (e) {
        console.log("Failed to send confirmation email", e)
        return null
      }
    },
    {
      mutationKey: "auth.send_confirmation_email",
      onSuccess: (_, email) => {
        Toasts.success(`Email to ${email} was send.`)
      },
      onError: (_, email) => {
        Toasts.error(`Failed to send confirmation email to ${email}.`)
      },
    },
  )

interface ConfirmEmailPayload {
  email: string
  code: string
}

export const useConfirmEmailByCode = () => {
  return useMutation(
    async ({ email, code }: ConfirmEmailPayload) => {
      const { response } = await GrpcWebClient.verifyEmailCode({
        email,
        code,
      })

      return response.isOk
    },
    {
      mutationKey: "auth.confirm_email_by_code",
      onSuccess: is_ok => {
        if (is_ok) {
          Toasts.success("Email was confirmed.")
          // for smoother transition
          setTimeout(() => {
            window.location.href = "/"
          }, 1000)
        } else {
          Toasts.error("Failed to confirm email. Wrong code.")
        }
      },
    },
  )
}

interface SignUpPayload {
  email: string
  username: string
  password: string
}
export const useSignUp = () => {
  return useMutation(
    async ({ email, password, username }: SignUpPayload) => {
      const { response } = await GrpcWebClient.signUp({
        email,
        username,
        password,
      })

      return response
    },
    {
      mutationKey: "auth.sign_up",
    },
  )
}
