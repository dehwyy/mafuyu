import { createMutation, createQuery } from "@tanstack/svelte-query"
import { GrpcWebClient } from "@makoto/grpc/web"
import { Toasts } from "$lib/utils/toast"
import { createReactiveQuery } from "$lib/query-abstraction"

export const FriendsKeys = {
  "query.friends": "friends.getFriends",
  "query.followers": "friends.getFollowers",
  "mutate.follow": "friends.followUser",
  "mutate.unfollow": "friends.unfollowUser",
} as const

export const useUserFriends = (userId: string | undefined, limit?: number) => {
  return createReactiveQuery({ userId, limit }, ({ userId, limit }) => ({
    queryKey: [FriendsKeys["query.friends"], userId, limit],
    enabled: !!userId,
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

export const useUserFollowers = (userId: string | undefined, limit?: number) => {
  return createReactiveQuery({ userId, limit }, ({ userId, limit }) => ({
    queryKey: [FriendsKeys["query.followers"], userId, limit],
    enabled: !!userId,
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

interface FollowUserPayload {
  userId: string
  successText: () => string
}

export const useFollowUser = () => {
  return createMutation({
    mutationKey: [FriendsKeys["mutate.follow"]],
    mutationFn: async (p: FollowUserPayload) => {
      const { response } = await GrpcWebClient.followUser({
        userId: p.userId,
      })

      return response
    },

    onSuccess: (_, p) => {
      Toasts.success(p.successText())
    },
  })
}

export const useUnfollowUser = () => {
  return createMutation({
    mutationKey: [FriendsKeys["mutate.unfollow"]],
    mutationFn: async (p: FollowUserPayload) => {
      const { response } = await GrpcWebClient.unfollowUser({
        userId: p.userId,
      })

      return response
    },

    onSuccess: (_, p) => {
      Toasts.success(p.successText())
    },
  })
}
