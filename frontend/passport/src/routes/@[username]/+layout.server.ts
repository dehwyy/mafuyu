import type { LayoutServerLoad } from "./$types"
import GrpcServerClient from "$lib/query/grpc/server"
import { queryClient } from "$lib/query-client"
import { getUserProfileQuery } from "$lib/query/profile"
import { dehydrate } from "@sveltestack/svelte-query"

export const load: LayoutServerLoad = async ({ cookies, params }) => {
  const username = params.username

  await queryClient.prefetchQuery({
    ...getUserProfileQuery(username, GrpcServerClient(0, cookies)),
  })

  return structuredClone({
    dehydrateState: dehydrate(queryClient, {
      dehydrateQueries: true,
    }),
  })
}
