import { TwirpFetchTransport } from "@protobuf-ts/twirp-transport"
import { ApiRpcClient } from "../dist/api.client"

const gateway_twirp_host = "http://localhost:3102/twirp"

const twirp_transport = new TwirpFetchTransport({
  baseUrl: gateway_twirp_host
})

export const TwirpClient = new ApiRpcClient(twirp_transport)