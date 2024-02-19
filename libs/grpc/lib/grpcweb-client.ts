import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport"
import { ApiRpcClient } from "../dist/api.client"

const grpc_web_transport = new GrpcWebFetchTransport({
  baseUrl: "http://localhost:3100",
  fetchInit: {
    credentials: "include",
  }
})

export const GrpcWebClient = new ApiRpcClient(grpc_web_transport)