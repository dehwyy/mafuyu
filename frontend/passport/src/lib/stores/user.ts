import { writable } from "svelte/store"

interface IUser {
  id: string
  username: string
}

interface IDynUser {
  picture?: string
  pseudonym?: string
}

export const authedUserStore = writable<IUser | null>(null)
export const dyn_user_store = writable<IDynUser | null>(null)

export const clear_user = () => {
  authedUserStore.set(null)
  dyn_user_store.set(null)
}
