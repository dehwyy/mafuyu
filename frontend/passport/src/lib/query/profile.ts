import { useQuery, type UseQueryOptions } from "@sveltestack/svelte-query"
import { type GrpcClient, GrpcWeb } from "$lib/query/grpc"

export const USER_PROFILE_KEY = "current_profile_info"

export const getUserProfileQuery = (username: string, grpc: GrpcClient = GrpcWeb(Infinity)) => {
  return {
    queryKey: [USER_PROFILE_KEY, username],
    retry: 1,
    staleTime: grpc.staleTime,
    refetchOnWindowFocus: false,
    queryFn: async () => {
      try {
        const { response } = await grpc.client.getUser(
          {
            login: {
              oneofKind: "username",
              username,
            },
          },
          { interceptors: grpc.interceptors },
        )

        const scopes = await grpc.client.getUserProfileScopes(
          {
            userId: response.userId,
          },
          { interceptors: grpc.interceptors },
        )
        return {
          ...response,
          scopes: scopes.response,
        }
      } catch (e) {
        console.log(e)
        return null
      }
    },
  } satisfies UseQueryOptions
}

export const useUserProfile = (username: string) => {
  return useQuery(getUserProfileQuery(username))
}
