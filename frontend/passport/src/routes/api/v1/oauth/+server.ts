import { redirect, type RequestHandler } from "@sveltejs/kit"
import { GrpcClient } from "@makoto/grpc"
import { Routes } from "$lib/utils/typed-fetch"

export const POST: RequestHandler = async ({ request, cookies }) => {
  const [req, create_response] = await Routes["oauth"].get_request_with_response_creator(request)

  const { response, headers, trailers } = await GrpcClient.createOAuth2RedirectUrl({
    provider: req.provider,
  })

  return create_response(
    {
      redirect_url: response.redirectUrl,
    },
    {
      status: 302,
    },
  )
}
