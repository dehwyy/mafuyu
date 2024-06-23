import { ApiRpcClient } from '@makoto/grpc/web'
import type { RpcInterceptor } from '@makoto/grpc/web'

export { default as GrpcWeb } from './client'

export interface GrpcClient {
  client: ApiRpcClient
  interceptors: RpcInterceptor[]
  staleTime: number
}
