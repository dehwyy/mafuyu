import { QueryClient } from "@sveltestack/svelte-query"
import { ParseCookie } from "$lib/utils/parse-cookie"
import { GrpcCookiesKeys, GrpcErrors } from "@makoto/grpc/const"
import { GrpcWebClient, type FinishedUnaryCall, type RpcMetadata, RpcError } from "@makoto/grpc/web"

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      async onError(error) {
        if (error instanceof RpcError) {
          const refresh_token = ParseCookie(document.cookie)[GrpcCookiesKeys.RefreshToken]
          if (error.code === GrpcErrors.UNAUTHENTICATED && refresh_token) {
            // Try to refresh token
            const r = await GrpcWebClient.refreshTheToken({
              refreshToken: refresh_token,
            })

            // setTokensCookies(r.headers)
          }
        }
      },
      async onSuccess(data: FinishedUnaryCall<object, object> | unknown) {
        // @ts-ignore
        // data?.headers && setTokensCookies(data.headers)
      },
    },
  },
})
