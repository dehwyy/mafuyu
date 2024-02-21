import { QueryClient } from "@sveltestack/svelte-query"
import { ParseCookie } from "$lib/utils/parse-cookie"
import { GrpcCookiesKeys, GrpcErrors } from "@makoto/grpc/const"
import { GrpcWebClient, type FinishedUnaryCall, type RpcMetadata, RpcError } from "@makoto/grpc/web"

const setTokensCookies = (rpc_headers: RpcMetadata) => {
  if (rpc_headers[GrpcCookiesKeys.AccessToken] && typeof rpc_headers[GrpcCookiesKeys.AccessToken] === "string") {
    document.cookie = `${GrpcCookiesKeys.AccessToken}=${rpc_headers[GrpcCookiesKeys.AccessToken]};path=/`
  }
  if (rpc_headers[GrpcCookiesKeys.RefreshToken] && typeof rpc_headers[GrpcCookiesKeys.RefreshToken] === "string") {
    document.cookie = `${GrpcCookiesKeys.RefreshToken}=${rpc_headers[GrpcCookiesKeys.RefreshToken]};path=/`
  }
}

export const createQueryClient = () =>
  new QueryClient({
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

              setTokensCookies(r.headers)
            }
          }
        },
        async onSuccess(data: FinishedUnaryCall<object, object> | unknown) {
          // @ts-ignore
          data?.headers && setTokensCookies(data.headers)
        },
      },
    },
  })
