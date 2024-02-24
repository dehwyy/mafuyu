import { createMutation } from "@tanstack/svelte-query"
import { GrpcWebClient } from "@makoto/grpc/web"
import { Toasts } from "$lib/utils/toast"

const AuthKeys = {
  sendEmail: "auth.sendEmailVerificationCode",
  confirmEmail: "auth.confirmEmailByCode",
  signUp: "auth.signUp",
  createOAuth2RedirectUrl: "auth.createOAuth2RedirectUrl",
} as const

interface SendEmailPayload {
  email: string
}
export const useSendConfirmationEmail = () =>
  createMutation({
    mutationKey: [AuthKeys.sendEmail],
    mutationFn: async ({ email }: SendEmailPayload) => {
      const r = await GrpcWebClient.sendEmailVerificationCode({ email })
      return r.response
    },
    onSuccess: (_, email) => {
      Toasts.success(`Email to ${email} was send.`)
    },
    onError: (_, email) => {
      Toasts.error(`Failed to send confirmation email to ${email}.`)
    },
  })

interface ConfirmEmailPayload {
  email: string
  code: string
}
export const useConfirmEmailByCode = () => {
  return createMutation({
    mutationKey: [AuthKeys.confirmEmail],

    mutationFn: async ({ email, code }: ConfirmEmailPayload) => {
      const { response } = await GrpcWebClient.verifyEmailCode({
        email,
        code,
      })

      return response.isOk
    },
    onSuccess: is_ok => {
      if (!is_ok) {
        Toasts.error("Failed to confirm email. Wrong code.")
        return
      }

      Toasts.success("Email was confirmed.")
      // for smoother transition
      setTimeout(() => {
        window.location.href = "/"
      }, 1000)
    },
  })
}

interface SignUpPayload {
  email: string
  username: string
  password: string
}
export const useSignUp = () => {
  return createMutation({
    mutationKey: [AuthKeys.signUp],

    mutationFn: async ({ email, password, username }: SignUpPayload) => {
      const { response } = await GrpcWebClient.signUp({
        email,
        username,
        password,
      })

      return response
    },
  })
}

interface CreateOAuth2RedirectUrlPayload {
  provider: string
}
export const useCreateOAuth2RedirectUrl = () => {
  return createMutation({
    mutationKey: [AuthKeys.createOAuth2RedirectUrl],
    mutationFn: async ({ provider }: CreateOAuth2RedirectUrlPayload) => {
      const r = await GrpcWebClient.createOAuth2RedirectUrl({
        provider,
      })
      return r.response
    },
    onSuccess: data => {
      window.location.href = data.redirectUrl
    },
  })
}
