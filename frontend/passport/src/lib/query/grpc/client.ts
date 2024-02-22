import { GrpcWebClient as GrpcClientWeb } from "@makoto/grpc/web"
import { type GrpcClient } from "./index"
class GrpcWebClient implements GrpcClient {
  constructor(public staleTime: number) {}
  get client() {
    return GrpcClientWeb
  }

  get interceptors() {
    return []
  }
}

export default (staleTime: number) => new GrpcWebClient(staleTime)
