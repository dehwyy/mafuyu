export enum MetadataKeys {
  AccessToken = 'x-access-token',
  RefreshToken = 'x-refresh-token'
}

// Cookies used by Grpc(Interceptor)
export enum GrpcCookiesKeys {
  AccessToken = 'access_token',
  RefreshToken = 'refresh_token'
}

export enum GrpcErrors {
  UNAUTHENTICATED = 'UNAUTHENTICATED'
}
