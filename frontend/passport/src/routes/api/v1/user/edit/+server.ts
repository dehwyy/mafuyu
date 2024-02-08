import { type RequestHandler } from "@sveltejs/kit"
import { GrpcClient } from "@makoto/grpc"
import { Routes } from "$lib/utils/typed-fetch"

export const POST: RequestHandler = async ({ request, cookies }) => {
  const [req, create_response] = await Routes["user/edit"].get_request_with_response_creator(request)

  const editUserPromise = GrpcClient.editUser({
    languages: req.languages,
    userId: req.userId,
    pseudonym: req.pseudonym,
    picture: req.picture,
    bio: req.bio,
    birthday: req.birthday,
    location: req.location,
  })

  req.username &&
    (await GrpcClient.updateUsername({
      userId: req.userId,
      username: req.username,
    }))

  await editUserPromise

  return create_response({}, { status: 200 })
}
