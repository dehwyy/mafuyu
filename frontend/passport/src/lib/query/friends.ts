import { QueryClient, createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query"
import { GrpcWebClient } from "@makoto/grpc/web"
import { Toasts } from "$lib/utils/toast"
import { createReactiveQuery } from "$lib/query-abstraction"
import { queryClient as qc } from "$lib/query-client"
import { Time } from "$lib/const"

export const FriendsKeys = {
  "query.friends": "friends.getFriends",
  "query.followers": "friends.getFollowers",
  "query.followedTo": "friends.getFollowedTo",
  "mutate.follow": "friends.followUser",
  "mutate.unfollow": "friends.unfollowUser",
} as const

const invalidateUserFriends = (q: QueryClient, keys: unknown[]) => {
  keys.forEach(key => {
    q.invalidateQueries({ queryKey: [FriendsKeys["query.friends"], key] })
  })
}

export const useUserFriends = (userId: string | undefined, limit?: number) => {
  return createReactiveQuery({ userId, limit }, ({ userId, limit }) => ({
    queryKey: [FriendsKeys["query.friends"], userId, limit],
    staleTime: 5 * Time.MINUTE,
    queryFn: async () => {
      if (!userId) return null

      const { response } = await GrpcWebClient.getUserFriends({
        userId,
        limit,
      })
      return response
    },
  }))
}

const invalidateUserFollowers = (q: QueryClient, keys: unknown[]) => {
  keys.forEach(key => {
    q.invalidateQueries({ queryKey: [FriendsKeys["query.followers"], key] })
  })
}

export const useUserFollowers = (userId: string | undefined, limit?: number) => {
  return createReactiveQuery({ userId, limit }, ({ userId, limit }) => ({
    queryKey: [FriendsKeys["query.followers"], userId, limit],
    staleTime: 5 * Time.MINUTE,
    queryFn: async () => {
      if (!userId) return null
      const { response } = await GrpcWebClient.getUserFollowers({
        userId,
        limit,
      })
      return response
    },
  }))
}

const invalidateUserFollowedTo = (q: QueryClient, userId: string, limit?: number) => {
  q.invalidateQueries({ queryKey: [FriendsKeys["query.followedTo"], userId, limit] })
}

export const useUserFollowedTo = (userId: string | undefined, limit?: number) => {
  return createReactiveQuery({ userId, limit }, ({ userId, limit }) => ({
    queryKey: [FriendsKeys["query.followedTo"], userId, limit],
    staleTime: 5 * Time.MINUTE,
    queryFn: async () => {
      if (!userId) return null
      const { response } = await GrpcWebClient.getUserFollowedTo({
        userId,
        limit,
      })
      return response
    },
  }))
}

interface FollowUserPayload {
  reqUserId: string // only for query invalidation purposes
  userId: string
  getSuccessText: () => string
}

export const useFollowUser = () => {
  const queryClient = useQueryClient(qc)
  return createMutation({
    mutationKey: [FriendsKeys["mutate.follow"]],
    mutationFn: async (p: FollowUserPayload) => {
      const { response } = await GrpcWebClient.followUser({
        userId: p.userId,
      })

      return response
    },

    onSuccess: (_, p) => {
      Toasts.success(p.getSuccessText())
      invalidateUserFollowedTo(queryClient, p.userId)
      invalidateUserFollowers(queryClient, [p.userId, p.reqUserId])
      invalidateUserFriends(queryClient, [p.userId, p.reqUserId])
    },
  })
}

export const useUnfollowUser = () => {
  const queryClient = useQueryClient(qc)
  return createMutation({
    mutationKey: [FriendsKeys["mutate.unfollow"]],
    mutationFn: async (p: FollowUserPayload) => {
      const { response } = await GrpcWebClient.unfollowUser({
        userId: p.userId,
      })

      return response
    },

    onSuccess: (_, p) => {
      Toasts.success(p.getSuccessText())
      invalidateUserFollowers(queryClient, [p.userId, p.reqUserId])
      invalidateUserFollowedTo(queryClient, p.userId)
      invalidateUserFriends(queryClient, [p.userId, p.reqUserId])
    },
  })
}
