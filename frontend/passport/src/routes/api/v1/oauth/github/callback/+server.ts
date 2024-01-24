import { redirect, type RequestHandler } from "@sveltejs/kit"
import { GrpcClient, Interceptors } from "@makoto/grpc"

export const GET: RequestHandler = async ({ url, cookies }) => {
  const code = url.searchParams.get("code")
  // const state = url.searchParams.get("state")

  if (code === null)
    new Response(null, {
      status: 400,
    })

  const { response } = await GrpcClient.signInOauth(
    {
      code: code!,
      provider: "github",
    },
    {
      interceptors: [Interceptors.WithTokens(Interceptors.WithTokensPayload.CreateForSvelteKit(cookies))],
    },
  )

  console.log(response)

  redirect(302, `/@${response.username}`)
}
