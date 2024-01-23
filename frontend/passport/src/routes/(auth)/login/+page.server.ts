import { redirect } from "@sveltejs/kit"
import { Routes } from "$lib/const"
import type { Actions, PageServerLoad } from "../../../../.svelte-kit/types/src/routes"
import { GrpcClient, Interceptors } from "@makoto/grpc"

export const load: PageServerLoad = async ({ parent }) => {
  const data = await parent()
  if (data.username) {
    throw redirect(307, Routes.Account + `/@${data.username}`)
  }
}

export const actions: Actions = {
  login: async ({ request, cookies }) => {
    const data = await request.formData()
    const email = data.get("email") as string
    const password = data.get("password") as string

    try {
      const { response } = await GrpcClient.signIn(
        {
          email,
          password,
          username: "",
        },
        {
          interceptors: [Interceptors.WithTokens(Interceptors.WithTokensPayload.CreateForSvelteKit(cookies))],
        },
      )
    } catch (e) {
      console.log(e)
      return
    }

    redirect(302, "/")
  },
}
