import type { Actions } from './$types'
import { GrpcClient, Interceptors } from '@makoto/grpc';
import {GrpcCookiesKeys} from "@makoto/grpc/const"

export const actions = {
  create: async ({request, cookies, }) => {
    const data = await request.formData()

    const username = data.get('username') as string
    const email = data.get("email") as string
    const password = data.get('password') as string

    console.log(username, email, password)

    const {response} = await GrpcClient.signUp({
      email,
      password,
      username
    }, {
      interceptors: [Interceptors.WithTokens(
        async () => cookies.get(GrpcCookiesKeys.AccessToken),
        async () => cookies.get(GrpcCookiesKeys.RefreshToken),
        (key, value) => cookies.set(key, value, {path: '/', httpOnly: true})
      )]
    })

    console.log(response)


    return {
      success: true
    }
  }
} satisfies Actions