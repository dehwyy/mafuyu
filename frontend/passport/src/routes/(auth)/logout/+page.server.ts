import { redirect } from "@sveltejs/kit"
import { Routes } from "$lib/const"
import type { PageServerLoad } from "../../../../.svelte-kit/types/src/routes/$types"

export const load: PageServerLoad = async ({ cookies }) => {
  // cookies.delete(GrpcCookiesKeys.AccessToken, {
  // 	path: "/"
  // })
  // cookies.delete(GrpcCookiesKeys.RefreshToken, {
  // 	path: "/"
  // })
}
