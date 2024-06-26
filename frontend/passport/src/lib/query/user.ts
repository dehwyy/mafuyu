import { GrpcWebClient } from '@makoto/grpc/web'
import {
  createMutation,
  createQuery,
  useQueryClient
} from '@tanstack/svelte-query'
import { Time } from '$lib/const'
import { createReactiveQuery } from '$lib/query-abstraction'
import { GrpcWeb } from '$lib/query/grpc'
import { Toasts } from '$lib/utils/toast'
import type { CreateQueryOptions } from '@tanstack/svelte-query'
import type { GrpcClient } from '$lib/query/grpc'

export const UserKeys = {
  'query.getUserInfo': 'user.getUserInfo',
  'query.getBaseUserInfo': 'user.getBaseUserInfo',
  'query.getBlockedUsers': 'user.getBlockedUsers',
  'query.getUsers': 'user.getUsers',
  'query.getUsersIDs': 'user.getUsers',
  'mutate.editUser': 'user.editUser',
  'mutate.blockUser': 'user.blockUser',
  'mutate.unblockUser': 'user.unblockUser'
} as const

type GetUserBy =
  | { oneofKind: 'userId'; userId: string | undefined }
  | { oneofKind: 'username'; username: string | undefined }

export const getUserInfoQuery = (
  getBy: GetUserBy,
  grpc: GrpcClient = GrpcWeb(5 * Time.MINUTE)
) => {
  const value = getBy.oneofKind === 'userId' ? getBy.userId : getBy.username
  return {
    queryKey: [UserKeys['query.getUserInfo'], value],
    retry: 1,
    staleTime: grpc.staleTime,
    refetchOnWindowFocus: false,
    queryFn: async () => {
      if (!value) return null
      const login =
        getBy.oneofKind === 'userId'
          ? ({ oneofKind: 'userId', userId: value } as const)
          : ({ oneofKind: 'username', username: value } as const)

      const r = await grpc.client.getUser(
        { login },
        { interceptors: grpc.interceptors }
      )

      return r.response
    }
  } satisfies CreateQueryOptions
}

export const useUserInfo = (getBy: GetUserBy) => {
  return createReactiveQuery({ getBy }, ({ getBy }) => getUserInfoQuery(getBy))
}

type GetUsersPayload = {
  userId?: string
  pattern?: string
  limit?: bigint
  offset?: bigint
  excludeLanguages?: boolean
}
export const getUsers = (
  payload: GetUsersPayload,
  grpc: GrpcClient = GrpcWeb(5 * Time.MINUTE)
) => {
  return {
    queryKey: [UserKeys['query.getUsers'], payload],
    staleTime: grpc.staleTime,
    queryFn: async () => {
      const r = await grpc.client.getUsers(
        {
          userId: payload.userId,
          limit: payload.limit,
          offset: payload.offset,
          pattern: payload.pattern,
          excludeLanguages: payload.excludeLanguages
        },
        { interceptors: grpc.interceptors }
      )

      return r.response
    }
  } satisfies CreateQueryOptions
}

export const useUsers = (payload: GetUsersPayload) => {
  return createReactiveQuery({ payload }, ({ payload }) => getUsers(payload))
}

export const getUsersIDs = (
  payload: GetUsersPayload,
  grpc: GrpcClient = GrpcWeb(5 * Time.MINUTE)
) => {
  return {
    queryKey: [UserKeys['query.getUsersIDs'], payload],
    staleTime: grpc.staleTime,
    queryFn: async () => {
      const r = await grpc.client.getUsersIDs(
        {
          userId: payload.userId,
          limit: payload.limit,
          offset: payload.offset,
          pattern: payload.pattern,
          excludeLanguages: payload.excludeLanguages
        },
        { interceptors: grpc.interceptors }
      )

      return r.response
    }
  } satisfies CreateQueryOptions
}

export const useUsersIDs = (payload: GetUsersPayload) => {
  return createReactiveQuery({ payload }, ({ payload }) => getUsersIDs(payload))
}

export const getBaseUserInfoQuery = (
  getBy: GetUserBy,
  grpc: GrpcClient = GrpcWeb(5 * Time.MINUTE)
) => {
  const value = getBy.oneofKind === 'userId' ? getBy.userId : getBy.username
  return {
    queryKey: [UserKeys['query.getBaseUserInfo'], value],
    retry: 1,
    staleTime: grpc.staleTime,
    refetchOnWindowFocus: false,
    queryFn: async () => {
      if (!value) return null
      const login =
        getBy.oneofKind === 'userId'
          ? ({ oneofKind: 'userId', userId: value } as const)
          : ({ oneofKind: 'username', username: value } as const)

      const r = await grpc.client.getUser(
        { login },
        { interceptors: grpc.interceptors }
      )

      return r.response
    }
  }
}

export const useBaseUserInfo = (getBy: GetUserBy) => {
  return createReactiveQuery({ getBy }, ({ getBy }) =>
    getBaseUserInfoQuery(getBy)
  )
}

interface EditUserPayload {
  username: string
  updateUsername: boolean // will produce page reload (`window.location.reload()`)
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
        languages: payload.languages
      })

      payload.username &&
        (await GrpcWebClient.updateUsername({
          userId: payload.userId,
          username: payload.username
        }))

      await r
    },

    onSuccess: async (_, payload) => {
      Toasts.success('Saved ')
      if (!payload.updateUsername) {
        query_client.invalidateQueries({
          queryKey: [UserKeys['query.getBaseUserInfo'], payload.userId]
        })
        query_client.invalidateQueries({
          queryKey: [UserKeys['query.getUserInfo'], payload.userId]
        })
        query_client.invalidateQueries({
          queryKey: [UserKeys['query.getUserInfo'], payload.username]
        })
      }
    },
    onError: (error) => {
      Toasts.error(`Failed to save changes. ${error}`)
    }
  })
}

// Block User section

export const getBlockedUsersQuery = (
  userId: string | undefined,
  grpc: GrpcClient = GrpcWeb(Time.MINUTE)
) => {
  return {
    queryKey: [UserKeys['query.getBlockedUsers'], userId],
    staleTime: grpc.staleTime,
    refetchOnWindowFocus: true,
    gcTime: 30 * Time.MINUTE,
    queryFn: async () => {
      if (!userId) return null
      const r = await grpc.client.getBlockedUsers(
        { userId },
        { interceptors: grpc.interceptors }
      )
      return r.response
    }
  } satisfies CreateQueryOptions
}

export const useBlockedUsers = (userId: string | undefined) => {
  return createReactiveQuery({ userId }, ({ userId }) =>
    getBlockedUsersQuery(userId)
  )
}

interface BlockUserPayload {
  requesterId: string
  userId: string
}

export const useBlockUser = () => {
  const qc = useQueryClient()
  return createMutation({
    mutationFn: async ({ userId }: BlockUserPayload) => {
      const r = await GrpcWebClient.blockUser({ userId })
      return r.response
    },
    onSuccess: (_, { requesterId }) => {
      qc.invalidateQueries({
        queryKey: [UserKeys['query.getBlockedUsers'], requesterId]
      })
    }
  })
}

export const useUnblockUser = () => {
  const qc = useQueryClient()
  return createMutation({
    mutationFn: async ({ userId }: BlockUserPayload) => {
      const r = await GrpcWebClient.unblockUser({ userId })
      return r.response
    },
    onSuccess: (_, { requesterId }) => {
      qc.invalidateQueries({
        queryKey: [UserKeys['query.getBlockedUsers'], requesterId]
      })
    }
  })
}
