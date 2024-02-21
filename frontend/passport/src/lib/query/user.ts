import { useMutation, useQuery, useQueryClient } from "@sveltestack/svelte-query"
import { GrpcWebClient } from "@makoto/grpc/web"
import { Toasts } from "$lib/utils/toast"

export const CURRENT_USER_KEY = "current_user_info"

export const useCurrentUserInfo = (userId: string | undefined) => {
  return useQuery({
    queryKey: [CURRENT_USER_KEY, userId],
    retry: 1,
    staleTime: 1000 * 10,
    refetchOnWindowFocus: false,
    enabled: !!userId,
    queryFn: async () => {
      if (!userId) return null

      const r = await GrpcWebClient.getUser({
        login: {
          oneofKind: "userId",
          userId,
        },
      })

      return r.response
    },
  })
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
      onSuccess: (_, payload) => {
        Toasts.success("Saved ")
        return query_client.invalidateQueries([CURRENT_USER_KEY, payload.userId])
      },
      onError: error => {
        Toasts.error(`Failed to save changes. ${error}`)
      },
    },
  )
}
