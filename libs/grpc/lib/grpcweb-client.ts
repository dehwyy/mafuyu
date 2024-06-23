import { GrpcWebFetchTransport } from '@protobuf-ts/grpcweb-transport'
import {
  FinishedUnaryCall,
  MethodInfo,
  NextUnaryFn,
  RpcError,
  RpcInterceptor,
  RpcMetadata,
  RpcOptions,
  UnaryCall
} from '@protobuf-ts/runtime-rpc'

import { ApiRpcClient } from '../dist/api.client'
import { GrpcCookiesKeys, MetadataKeys } from './const'

export const ParseCookie = (cookie: string): Record<string, string> => {
  return cookie
    .split(';')
    .map((v) => v.split('='))
    .reduce((acc: Record<string, string>, v) => {
      acc[decodeURIComponent(v[0].trim())] = decodeURIComponent(v[1].trim())
      return acc
    }, {})
}

const WithTokens = (): RpcInterceptor => {
  return {
    interceptUnary(
      next: NextUnaryFn,
      method: MethodInfo,
      input: object,
      options: RpcOptions
    ): UnaryCall {
      try {
        const cookie = ParseCookie(document.cookie)
        const access_token = cookie[GrpcCookiesKeys.AccessToken]
        if (access_token) {
          options.meta = options.meta || {}
          options.meta[MetadataKeys.AccessToken] = access_token
        }
      } catch (_) {}

      const r = next(method, input, options)

      r.headers.then((headers) => {
        const access_token = headers[MetadataKeys.AccessToken]
        if (access_token && typeof access_token === 'string') {
          document.cookie = `${GrpcCookiesKeys.AccessToken}=${access_token};path=/;maxAge=604800`
        }

        const refresh_token = headers[MetadataKeys.RefreshToken]
        if (refresh_token && typeof refresh_token === 'string') {
          document.cookie = `${GrpcCookiesKeys.RefreshToken}=${refresh_token};path=/;maxAge=18144000`
        }
      })

      return r
    }
  }
}

const grpc_web_transport = new GrpcWebFetchTransport({
  baseUrl: 'http://localhost:3100',
  fetchInit: {
    credentials: 'include'
  },
  interceptors: [WithTokens()]
})

export const GrpcWebClient = new ApiRpcClient(grpc_web_transport)
export {
  RpcMetadata,
  FinishedUnaryCall,
  RpcError,
  RpcInterceptor,
  ApiRpcClient
}
