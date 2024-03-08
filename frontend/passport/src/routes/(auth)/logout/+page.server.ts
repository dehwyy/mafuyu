import { redirect } from "@sveltejs/kit"
import { Routes } from "$lib/const"
import type { PageServerLoad } from "../../../../.svelte-kit/types/src/routes/$types"
import { GrpcClient } from "@makoto/grpc"
import { GrpcCookiesKeys } from "@makoto/grpc/const"

export const load: PageServerLoad = async ({ cookies }) => {
  const token = cookies.get(GrpcCookiesKeys.AccessToken)

  if (token) {
    const { response } = await GrpcClient.signOut({
      accessToken: cookies.get(GrpcCookiesKeys.AccessToken)!,
    })

    if (response.isOk) {
      cookies.delete(GrpcCookiesKeys.AccessToken, {
        path: "/",
      })
      cookies.delete(GrpcCookiesKeys.RefreshToken, {
        path: "/",
      })
    }
  }

  throw redirect(302, Routes.Login)
}
