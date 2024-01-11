pub mod general;
pub mod api_auth;
pub mod api;

/// Convinient way to create `general::BoolStatus`
impl general::BoolStatus {
  pub fn truthy() -> Self {
    Self {
      status: true
    }
  }
  pub fn falsy() -> Self {
    Self {
      status: false
    }
  }
}

const METADATA_ACCESS_TOKEN_KEY: &str = "x-access-token";
const METADATA_REFRESH_TOKEN_KEY: &str = "x-refresh-token";

/// Converts Response from GrpcService with `(RefreshToken, AccessToken)` for GrpcGateway (attaches tokens as `headers`)
impl Into<tonic::Response<api_auth::AuthenticationResponse>> for api_auth::AuthenicationServiceResponseWithRefreshToken {
  fn into(self) -> tonic::Response<api_auth::AuthenticationResponse> {
    let (access_token, refresh_token) = (self.token, self.refresh_token);

    let mut response = tonic::Response::new(api_auth::AuthenticationResponse {
      username: self.username,
      user_id: self.user_id
    });

    response.metadata_mut().insert(METADATA_ACCESS_TOKEN_KEY, access_token.parse().unwrap());
    response.metadata_mut().insert(METADATA_REFRESH_TOKEN_KEY, refresh_token.parse().unwrap());

    response
  }
}

/// Same as `Above`, but stores only `AccessToken` in `Metadata`
impl Into<tonic::Response<api_auth::AuthenticationResponse>> for api_auth::AuthenticationServiceResponse {
  fn into(self) -> tonic::Response<api_auth::AuthenticationResponse> {

    let access_token = self.token;

    let mut response = tonic::Response::new(api_auth::AuthenticationResponse {
      username: self.username,
      user_id: self.user_id
    });

    response.metadata_mut().insert(METADATA_ACCESS_TOKEN_KEY, access_token.parse().unwrap());

    response
  }
}
