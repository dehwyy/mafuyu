import { GrpcCookiesKeys, GrpcErrors } from '@makoto/grpc/const'
import { GrpcWebClient, RpcError } from '@makoto/grpc/web'
import { QueryCache, QueryClient } from '@tanstack/svelte-query'
import { browser } from '$app/environment'
import { ParseCookie } from '$lib/utils/parse-cookie'
import type { FinishedUnaryCall, RpcMetadata } from '@makoto/grpc/web'

export const queryClient = new QueryClient({
  queryCache: new QueryCache({
    onError: async (error, _) => {
      if (error instanceof RpcError) {
        const refresh_token = ParseCookie(document.cookie)[
          GrpcCookiesKeys.RefreshToken
        ]
        if (error.code === GrpcErrors.UNAUTHENTICATED && refresh_token) {
          console.log('trying to refresh')
          // Try to refresh token
          await GrpcWebClient.refreshTheToken({
            refreshToken: refresh_token
          })
          console.log('refreshed?')
        }
      }
    }
  }),
  defaultOptions: {
    queries: {
      enabled: browser
    }
  }
})
