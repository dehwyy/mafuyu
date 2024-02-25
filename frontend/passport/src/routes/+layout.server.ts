import type { LayoutServerLoad } from "./$types"
import { GrpcClient, Interceptors } from "@makoto/grpc"
import { GrpcCookiesKeys } from "@makoto/grpc/const"
import { queryClient } from "$lib/query-client"
import GrpcServerClient from "$lib/query/grpc/server"
import { getUserInfoQuery } from "$lib/query/user"
import { dehydrate } from "@tanstack/svelte-query"

export const load: LayoutServerLoad = async ({ cookies, url, setHeaders }) => {
  if (!cookies.get(GrpcCookiesKeys.AccessToken) && !cookies.get(GrpcCookiesKeys.RefreshToken))
    return {
      url: url.pathname,
    }

  try {
    const { response, headers } = await GrpcClient.signInWithToken(
      {
        token: "", // will be set in interceptor
      },
      {
        interceptors: [Interceptors.WithTokens(Interceptors.WithTokensPayload.CreateForSvelteKit(cookies))],
        timeout: 5000, // secs
      },
    )

    await queryClient.prefetchQuery(getUserInfoQuery({ oneofKind: "username", username: response.username }, GrpcServerClient(5000, cookies)))

    return {
      userId: response.userId,
      username: response.username,
      dehydrateState: structuredClone(dehydrate(queryClient)),
      url: url.pathname,
    }
  } catch (e) {
    console.log(e)

    return {
      url: url.pathname,
    }
  }
}
