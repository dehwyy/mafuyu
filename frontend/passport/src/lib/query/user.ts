import { useMutation, useQuery, useQueryClient, type UseQueryOptions } from "@sveltestack/svelte-query"
import { GrpcWebClient } from "@makoto/grpc/web"
import { Toasts } from "$lib/utils/toast"
import { USER_PROFILE_KEY } from "$lib/query/profile"
import { type GrpcClient, GrpcWeb } from "$lib/query/grpc"
import { navigating } from "$app/stores"
export const CURRENT_USER_KEY = "current_user_info"

export const getCurrentUserInfoQuery = (userId: string | undefined, grpc: GrpcClient = GrpcWeb(Infinity)) => {
  return {
    queryKey: [CURRENT_USER_KEY, userId],
    retry: 1,
    staleTime: grpc.staleTime,
    refetchOnWindowFocus: false,
    enabled: !!userId,
    queryFn: async () => {
      if (!userId) return null

      const r = await grpc.client.getUser(
        {
          login: {
            oneofKind: "userId",
            userId,
          },
        },
        { interceptors: grpc.interceptors },
      )

      return r.response
    },
  } satisfies UseQueryOptions
}
export const useCurrentUserInfo = (userId: string | undefined) => {
  return useQuery(getCurrentUserInfoQuery(userId))
}

interface EditUserPayload {
  username?: string
  userId: string
  pseudonym?: string
  bio?: string
  birthday?: bigint
  location?: string
  picture?: string
  languages: string[]
}
export const useEditUser = () => {
  const query_client = useQueryClient()

  return useMutation(
    async (payload: EditUserPayload) => {
      const r = GrpcWebClient.editUser({
        userId: payload.userId,
        pseudonym: payload.pseudonym,
        bio: payload.bio,
        birthday: payload.birthday,
        location: payload.location,
        picture: payload.picture,
        languages: payload.languages,
      })

      payload.username &&
        (await GrpcWebClient.updateUsername({
          userId: payload.userId,
          username: payload.username,
        }))

      await r
    },
    {
      onSuccess: async (_, payload) => {
        Toasts.success("Saved ")
        if (!payload.username) {
          query_client.invalidateQueries([CURRENT_USER_KEY, payload.userId])
          query_client.invalidateQueries(USER_PROFILE_KEY)
        }
      },
      onError: error => {
        Toasts.error(`Failed to save changes. ${error}`)
      },
    },
  )
}
