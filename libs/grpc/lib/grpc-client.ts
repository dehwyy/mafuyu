import { ApiRpcClient } from '../dist/api.client'
import { ChannelCredentials } from '@grpc/grpc-js'
import { GrpcTransport } from '@protobuf-ts/grpc-transport'
import { TwirpFetchTransport } from '@protobuf-ts/twirp-transport'
import {
  Deferred, FinishedUnaryCall,
  MethodInfo,
  NextUnaryFn, RpcError,
  RpcInterceptor,
  RpcMetadata,
  RpcOptions, RpcStatus,
  UnaryCall
} from '@protobuf-ts/runtime-rpc'
import { MetadataKeys, GrpcErrors, GrpcCookiesKeys } from './const'
import { Cookies } from '@sveltejs/kit/src/exports/public';


const gateway_host = process.env?.GATEWAY_HOST ?? "localhost:3100"

const grpc_transport = new GrpcTransport({
  host: gateway_host,
  channelCredentials: ChannelCredentials.createInsecure(),
})

export const GrpcClient = new ApiRpcClient(grpc_transport)


export namespace Interceptors {
  export class WithTokensPayload {

    private constructor(

      public get_access_token: () => Promise<string | undefined>,
      public get_refresh_token: () => Promise<string | undefined>,
      public set_access_token: (token: string) => void,
      public set_refresh_token: (token: string) => void
    ) {}

    public static CreateForSvelteKit(cookies: Cookies): WithTokensPayload {
      return new WithTokensPayload(
        async () => cookies.get(GrpcCookiesKeys.AccessToken),
        async () => cookies.get(GrpcCookiesKeys.RefreshToken),
        (token) => cookies.set(GrpcCookiesKeys.AccessToken, token, {path: '/', httpOnly: true}),
        (token) => cookies.set(GrpcCookiesKeys.RefreshToken, token, {path: '/', httpOnly: true})
      )
    }

  }

  class DeferredRequest {
    headers: Deferred<RpcMetadata>
    response: Deferred<object>
    status: Deferred<RpcStatus>
    trailers: Deferred<RpcMetadata>

    constructor() {
      this.headers = new Deferred<RpcMetadata>()
      this.response = new Deferred<object>()
      this.status = new Deferred<RpcStatus>()
      this.trailers = new Deferred<RpcMetadata>()
    }

    getAll() {
      return {
        headers: this.headers.promise,
        response: this.response.promise,
        status: this.status.promise,
        trailers: this.trailers.promise,
      }
    }

    resolveAll<T extends object = object>(r: FinishedUnaryCall<T, T> | null) {
      this.headers.resolve(r?.headers || {})
      this.response.resolve(r?.response || {})
      this.status.resolve(
        r?.status ||
        ({
          code: 'UNKNOWN',
          detail: 'Unknown error',
        } as RpcStatus)
      )
      this.trailers.resolve(r?.trailers || {})
    }

    rejectAll(r: unknown) {
      this.headers.rejectPending(r)
      this.response.rejectPending(r)
      this.status.rejectPending(r)
      this.trailers.rejectPending(r)
    }
  }
  export const WithTokens = (
    payload: WithTokensPayload
  ): RpcInterceptor  => {

    return {
      interceptUnary(next: NextUnaryFn, method: MethodInfo, input: object, options: RpcOptions): UnaryCall {
        const deferred_req = new DeferredRequest()


        if (!options.meta) {
          options.meta = {}
        }

        void (async () => {
          const access_token =  await payload.get_access_token()

          if (access_token) {
            options.meta![MetadataKeys.AccessToken] = access_token
          }

          try {
            const response = await next(method, input, options)

            const access_token_from_response = response.headers[MetadataKeys.AccessToken] as string
            const refresh_token_from_response = response.headers[MetadataKeys.RefreshToken] as string

            if (access_token_from_response) {
              payload.set_access_token(access_token_from_response)
            }

            if (refresh_token_from_response) {
              payload.set_refresh_token(refresh_token_from_response)
            }
            deferred_req.resolveAll(response)
          } catch (e) {
            if (! (e instanceof RpcError)) {
              deferred_req.resolveAll(null)
              console.log("[NOT RPC_ERROR]: ", e)
            }
            const err = e as RpcError

            if (err.code === GrpcErrors.UNAUTHENTICATED) {
              // try to refresh
              const refresh_token = await payload.get_refresh_token()
              if (!refresh_token)  {
                deferred_req.rejectAll(e)
                return
              }
              let new_access_token = ""

              try {

                const {headers  } = await GrpcClient.refreshTheToken({
                  refreshToken: refresh_token,
                })

                const access_token = headers[MetadataKeys.AccessToken] as string

                payload.set_access_token(access_token)
                new_access_token = access_token

              } catch (e) {
                deferred_req.rejectAll(e)
                return
              }

              options.meta![MetadataKeys.AccessToken] = new_access_token

              try {
                // perform request once again
                const response = await next(method, input, options)
                deferred_req.resolveAll(response)
              } catch (e) {
                deferred_req.rejectAll(e)
                return
              }

            }

            deferred_req.rejectAll(e)
          }
        })()



        return new UnaryCall(
          method, options.meta, input,
          deferred_req.headers.promise, deferred_req.response.promise, deferred_req.status.promise, deferred_req.trailers.promise
        )
      }
    }
  }
}


