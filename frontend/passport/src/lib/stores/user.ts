import { writable } from "svelte/store"

interface IUser {
  id: string
  username: string
  picture?: string
  pseudonym?: string
}

export const user_store = writable<IUser | null>(null)
