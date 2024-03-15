import { createMutation } from "@tanstack/svelte-query"
import { GrpcWebClient } from "@makoto/grpc/web"
import { Toasts } from "$lib/utils/toast"

const AuthKeys = {
  signUp: "auth.signUp",
  createOAuth2RedirectUrl: "auth.createOAuth2RedirectUrl",
} as const

export const useSignUp = () => {
  return createMutation({
    mutationKey: [AuthKeys.signUp],

    mutationFn: async ({ email, password, username }: { email: string; password: string; username: string }) => {
      const { response } = await GrpcWebClient.signUp({
        email,
        username,
        password,
      })

      return response
    },
  })
}

export const useCreateOAuth2RedirectUrl = () => {
  return createMutation({
    mutationKey: [AuthKeys.createOAuth2RedirectUrl],
    mutationFn: async ({ provider }: { provider: string }) => {
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
