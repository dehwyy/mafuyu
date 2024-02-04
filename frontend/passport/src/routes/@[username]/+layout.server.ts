import type { LayoutServerLoad } from "./$types"
import { GrpcClient, Interceptors } from "@makoto/grpc"

export const load: LayoutServerLoad = async ({ url, cookies }) => {
  const username = url.pathname.split("/")[1].slice(1)

  if (!username) return {}

  try {
    const { response } = await GrpcClient.getUser(
      {
        username,
      },
      {
        interceptors: [Interceptors.WithTokens(Interceptors.WithTokensPayload.CreateForSvelteKit(cookies))],
        timeout: 5000, // secs
      },
    )

    return {
      userId: response.userId,
      username: response.username,
      location: response.location,
      birthday: response.birthday,
      pseudonym: response.pseudonym,
      bio: response.bio,
      picture: response.picture,
      languages: response.languages,
      friends: response.friends,
      followers: response.followers,
    }
  } catch (e) {
    console.log(e)
  }
}
