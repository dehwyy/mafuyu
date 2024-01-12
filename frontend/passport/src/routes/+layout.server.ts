import type {LayoutServerLoad} from "./$types"
import { GrpcClient, Interceptors } from '@makoto/grpc';
import { GrpcCookiesKeys } from '@makoto/grpc/const';

export const load: LayoutServerLoad = async ({cookies}) => {
	if (
		!cookies.get(GrpcCookiesKeys.AccessToken) &&
		!cookies.get(GrpcCookiesKeys.RefreshToken)
	) return

	try {

		const {response} = await GrpcClient.signInToken({
			token: "" // will be set in interceptor
		}, {
			interceptors: [
				Interceptors.WithTokens(Interceptors.WithTokensPayload.CreateForSvelteKit(cookies))
			],
			timeout: 5000 // secs
		})

		return {
			userId: response.userId,
			username: response.username
		}

	} catch (e) {
		console.log(e)

		return {}
	}
}