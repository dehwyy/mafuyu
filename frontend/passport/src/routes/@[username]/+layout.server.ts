import type { LayoutServerLoad } from "./$types"
import GrpcServerClient from "$lib/query/grpc/server"
import { queryClient } from "$lib/query-client"
import { getUserInfoQuery } from "$lib/query/user"
import { getUserProfileScopesQuery } from "$lib/query/profile"
import { dehydrate } from "@tanstack/svelte-query"

export const load: LayoutServerLoad = async ({ cookies, params, parent }) => {
  const username = params.username

  const r = await queryClient.fetchQuery({
    ...getUserInfoQuery({ oneofKind: "username", username }, GrpcServerClient(0, cookies)),
  })

  await queryClient.prefetchQuery({
    ...getUserProfileScopesQuery(r?.userId, GrpcServerClient(0, cookies)),
  })

  return structuredClone({
    dehydrateState: dehydrate(queryClient),
  })
}
