import { Toasts } from "$lib/utils/toast"
import { GrpcWebClient } from "@makoto/grpc/web"
import { createMutation } from "@tanstack/svelte-query"

const EmailKeys = {
  sendEmail: "email.mutate.sendEmail",
  confirmEmail: "email.mutate.confirmEmail",
} as const

const useSendConfirmationEmail = () => {
  return createMutation({
    mutationKey: [EmailKeys.sendEmail],
    mutationFn: async ({ email }: { email: string }) => {
      const r = await GrpcWebClient.sendEmailVerificationCode({ email })
      return r.response
    },
    onSuccess: (_, { email }) => {
      Toasts.success(`Email to ${email} was send.`)
    },
    onError: (_, { email }) => {
      Toasts.error(`Failed to send confirmation email to ${email}.`)
    },
  })
}

const useConfirmEmailByCode = () => {
  return createMutation({
    mutationKey: [EmailKeys.confirmEmail],

    mutationFn: async ({ email, code }: { email: string; code: string }) => {
      const { response } = await GrpcWebClient.verifyEmailCode({
        email,
        code,
      })

      return response.isOk
    },
    onSuccess: isOk => {
      if (!isOk) {
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

export { useConfirmEmailByCode, useSendConfirmationEmail }
