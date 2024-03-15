import { type GrpcClient, GrpcWeb } from "$lib/query/grpc"
import { createReactiveQuery, type CreateQueryOptions } from "$lib/query-abstraction"

export const ProfileKeys = {
  "query.userProfileScopes": "profile.getQueryProfileScopes",
} as const

export const getUserProfileScopesQuery = (userId: string | undefined, grpc: GrpcClient = GrpcWeb(Infinity)) => {
  return {
    queryKey: [ProfileKeys["query.userProfileScopes"], userId],
    retry: 1,
    staleTime: grpc.staleTime,
    refetchOnWindowFocus: false,
    queryFn: async () => {
      if (!userId) return null

      const { response } = await grpc.client.getUserProfileScopes(
        {
          userId,
        },
        { interceptors: grpc.interceptors },
      )

      return response
    },
  } satisfies CreateQueryOptions
}

export const useUserProfileScopes = (userId: string | undefined) => {
  return createReactiveQuery({ userId }, ({ userId }) => getUserProfileScopesQuery(userId))
}
