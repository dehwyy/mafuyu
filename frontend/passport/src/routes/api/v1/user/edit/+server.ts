import { type RequestHandler } from "@sveltejs/kit"
import { GrpcClient } from "@makoto/grpc"

export const POST: RequestHandler = async ({ request, cookies }) => {
  const req = await request.json()

  console.log(req)

  await GrpcClient.editUser({
    languages: [],
    userId: req.userId,
    pseudonym: req.username,
    picture: req.image,
  })

  return new Response(JSON.stringify({}), {
    status: 200,
  })
}
