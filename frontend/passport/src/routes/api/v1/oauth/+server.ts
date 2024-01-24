import { redirect, type RequestHandler } from "@sveltejs/kit"
import { GrpcClient } from "@makoto/grpc"

export const POST: RequestHandler = async ({ request, cookies }) => {
  const req = await request.json()

  const { response } = await GrpcClient.createOAuth2RedirectUrl({
    provider: req.provider,
  })

  return new Response(
    JSON.stringify({
      redirect_url: response.url,
    }),
    {
      status: 302,
    },
  )
}
