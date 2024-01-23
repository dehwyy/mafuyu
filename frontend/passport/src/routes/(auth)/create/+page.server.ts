import type { Actions } from "../../../../.svelte-kit/types/src/routes"
import { GrpcClient, Interceptors } from "@makoto/grpc"
import { redirect } from "@sveltejs/kit"
import { Routes } from "$lib/const"
import type { PageServerLoad } from "../../../../.svelte-kit/types/src/routes"

export const load: PageServerLoad = async ({ parent }) => {
  const data = await parent()
  if (data.username) {
    throw redirect(307, Routes.Account + `/@${data.username}`)
  }
}

export const actions = {
  create: async ({ request, cookies }) => {
    const data = await request.formData()

    const username = data.get("username") as string
    const email = data.get("email") as string
    const password = data.get("password") as string

    let r: unknown

    try {
      const { response } = await GrpcClient.signUp(
        {
          email,
          password,
          username,
        },
        {
          interceptors: [Interceptors.WithTokens(Interceptors.WithTokensPayload.CreateForSvelteKit(cookies))],
        },
      )

      r = response
    } catch (e) {
      console.log(e)
    }

    r && redirect(302, "/")
  },
} satisfies Actions
