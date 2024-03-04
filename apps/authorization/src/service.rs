use tonic::{Request, Response, Status};

use makoto_grpc::metadata::{UserRole, Metadata};
use makoto_grpc::pkg::authorization::{authorization_rpc_server::AuthorizationRpc, *};

pub struct AuthorizationRpcServiceImplementation {
  redis_client: redis::Client
}

impl AuthorizationRpcServiceImplementation {
  pub async fn new(redis_client: redis::Client) -> Self {
    // let clients = makoto_grpc::RpcClients::get_all_client().await;

    Self {
      redis_client
    }
  }
}

#[tonic::async_trait]
impl AuthorizationRpc for AuthorizationRpcServiceImplementation {
  async fn get_user_profile_scopes(&self, req: Request<GetUserProfileScopesRequest>) -> Result<Response<GetUserProfileScopesResponse>, Status> {
    let (metadata, _, r) = req.into_parts();
    let res = match Metadata::get_user_role_and_id_from_metadata(&metadata) {
      Some((user_role, user_id)) => {
          GetUserProfileScopesResponse {
            view_info: true,
            view_statistics: user_role > UserRole::Unauthorized || user_id == r.user_id,
            edit: user_role > UserRole::User || user_id == r.user_id
          }
      },
      None => GetUserProfileScopesResponse { view_info: true, view_statistics: false, edit: false }
    };


    Ok(Response::new(res))
  }
}