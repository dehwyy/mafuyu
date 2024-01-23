import { redirect } from "@sveltejs/kit"
import { Routes } from "$lib/const"
import type { PageServerLoad } from "../../../../.svelte-kit/types/src/routes"
import { GrpcCookiesKeys } from "@makoto/grpc/dist/const.ts"

export const load: PageServerLoad = async ({ cookies }) => {
  console.log(1)
  // cookies.delete(GrpcCookiesKeys.AccessToken, {
  // 	path: "/"
  // })
  // cookies.delete(GrpcCookiesKeys.RefreshToken, {
  // 	path: "/"
  // })

  redirect(302, Routes.Login)
}
