import type { LayoutServerLoad } from "./$types"
import { GrpcClient, Interceptors } from "@makoto/grpc"
import { GrpcCookiesKeys } from "@makoto/grpc/const"
import { queryClient } from "$lib/query-client"
import GrpcServerClient from "$lib/query/grpc/server"
import { getUserInfoQuery } from "$lib/query/user"
import { dehydrate } from "@tanstack/svelte-query"

const defaultVal = (url: URL) => ({ url: url.pathname })

export const load: LayoutServerLoad = async ({ cookies, url }) => {
  const accessToken = cookies.get(GrpcCookiesKeys.AccessToken)

  if (!accessToken && !cookies.get(GrpcCookiesKeys.RefreshToken)) {
    return defaultVal(url)
  }

  try {
    const { response } = await GrpcClient.signInWithToken(
      {
        token: accessToken ?? "",
      },
      {
        interceptors: [Interceptors.WithTokens(Interceptors.WithTokensPayload.CreateForSvelteKit(cookies))],
      },
    )

    await queryClient.prefetchQuery(getUserInfoQuery({ oneofKind: "username", username: response.username }, GrpcServerClient(5000, cookies)))

    return {
      userId: response.userId,
      username: response.username,
      dehydrateState: structuredClone(dehydrate(queryClient)),
      url: url.pathname,
    }
  } catch (_) {
    return defaultVal(url)
  }
}
