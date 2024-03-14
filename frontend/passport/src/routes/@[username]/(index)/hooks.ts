import { useBlockedUsers, useUserInfo } from "$lib/query/user"
import { page } from "$app/stores"
import { authedUserStore } from "$lib/stores/user"
import { derived, get } from "svelte/store"

export const usePage = () => {
  const initialUsername = get(page).params.username
  const initialAuthedUserId = get(authedUserStore)?.id

  const [user, userStore] = useUserInfo({ oneofKind: "username", username: initialUsername })
  const [authedUserBlockedUsers, authedUserBlockedUsersStore] = useBlockedUsers(initialAuthedUserId)

  derived(page, pageValue => {
    userStore.set({ getBy: { oneofKind: "username", username: pageValue.params.username } })
  })

  derived(authedUserStore, authedUser => {
    authedUserBlockedUsersStore.set({ userId: authedUser?.id })
  })

  const userData = derived(user, user => user?.data)

  const isUserBlocked = derived([authedUserBlockedUsers, userData], ([authedUserBlockedUsers, userData]) => {
    return userData?.userId ? authedUserBlockedUsers?.data?.blockedUsers.includes(userData.userId) : false
  })

  return {
    user: userData,
    isUserBlocked,
  }
}
