import { createMutation, createQuery, type CreateQueryOptions, useQueryClient } from "@tanstack/svelte-query"
import { GrpcWebClient } from "@makoto/grpc/web"
import { Toasts } from "$lib/utils/toast"
import { type GrpcClient, GrpcWeb } from "$lib/query/grpc"
import { createReactiveQuery } from "$lib/query-abstraction"

export const UserKeys = {
  "query.getUserInfo": "user.getUserInfo",
  "query.getBaseUserInfo": "user.getBaseUserInfo",
  "mutate.editUser": "user.editUser",
} as const

type GetUserBy = { oneofKind: "userId"; userId: string | undefined } | { oneofKind: "username"; username: string | undefined }

export const getUserInfoQuery = (getBy: GetUserBy, grpc: GrpcClient = GrpcWeb(0)) => {
  const value = getBy.oneofKind === "userId" ? getBy.userId : getBy.username
  return {
    queryKey: [UserKeys["query.getUserInfo"], value],
    retry: 1,
    staleTime: grpc.staleTime,
    refetchOnWindowFocus: false,
    enabled: !!value,
    queryFn: async () => {
      if (!value) return null
      const login =
        getBy.oneofKind === "userId" ? ({ oneofKind: "userId", userId: value } as const) : ({ oneofKind: "username", username: value } as const)

      const r = await grpc.client.getUser({ login }, { interceptors: grpc.interceptors })

      return r.response
    },
  } satisfies CreateQueryOptions
}

export const useUserInfo = (getBy: GetUserBy) => {
  return createReactiveQuery({ getBy }, ({ getBy }) => getUserInfoQuery(getBy))
}

export const getBaseUserInfoQuery = (getBy: GetUserBy, grpc: GrpcClient = GrpcWeb(0)) => {
  const value = getBy.oneofKind === "userId" ? getBy.userId : getBy.username
  return {
    queryKey: [UserKeys["query.getBaseUserInfo"], value],
    retry: 1,
    staleTime: grpc.staleTime,
    refetchOnWindowFocus: false,
    enabled: !!value,
    queryFn: async () => {
      if (!value) return null
      const login =
        getBy.oneofKind === "userId" ? ({ oneofKind: "userId", userId: value } as const) : ({ oneofKind: "username", username: value } as const)

      const r = await grpc.client.getUser({ login }, { interceptors: grpc.interceptors })

      return r.response
    },
  }
}

export const useBaseUserInfo = (getBy: GetUserBy) => {
  return createReactiveQuery({ getBy }, ({ getBy }) => getBaseUserInfoQuery(getBy))
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

  return createMutation({
    mutationFn: async (payload: EditUserPayload) => {
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

    onSuccess: async (_, payload) => {
      Toasts.success("Saved ")
      if (!payload.username) {
        query_client.invalidateQueries({ queryKey: [UserKeys["query.getBaseUserInfo"], payload.userId] })
        query_client.invalidateQueries({ queryKey: [UserKeys["query.getUserInfo"], payload.userId] })
      }
    },
    onError: error => {
      Toasts.error(`Failed to save changes. ${error}`)
    },
  })
}
