import { type GrpcClient } from "./index"
import { GrpcClient as GrpcClientServer, Interceptors } from "@makoto/grpc"
import type { Cookies } from "@sveltejs/kit"

class GrpcServerClient implements GrpcClient {
  constructor(
    public staleTime: number,
    private cookies?: Cookies,
  ) {}
  get client() {
    return GrpcClientServer
  }

  get interceptors() {
    return this.cookies ? [Interceptors.WithTokens(Interceptors.WithTokensPayload.CreateForSvelteKit(this.cookies))] : []
  }
}

export default (staleNumber: number, cookies?: Cookies) => new GrpcServerClient(staleNumber, cookies)
