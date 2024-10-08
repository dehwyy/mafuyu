import { GrpcClient, Interceptors } from '@makoto/grpc'
import { redirect } from '@sveltejs/kit'
import type { RequestHandler } from '@sveltejs/kit'





export const GET: RequestHandler = async ({ url, cookies, setHeaders }) => {
  const code = url.searchParams.get('code')
  const csrfToken = url.searchParams.get('state')

  if (code === null || csrfToken === null) {
    redirect(307, `/login?error=access_denied&error_desc=Access%20denied`)
  }

  console.log(code, csrfToken)
  const { response, headers } = await GrpcClient.signInOAuth2(
    {
      csrfToken: csrfToken!,
      code: code!,
      provider: 'google'
    },
    {
      interceptors: [
        Interceptors.WithTokens(
          Interceptors.WithTokensPayload.CreateForSvelteKit(cookies)
        )
      ]
    }
  )

  redirect(302, `/@${response.username}`)
}
