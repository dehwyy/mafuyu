use uuid::Uuid;
use makoto_grpc::errors::GrpcHandleError;
use makoto_grpc::metadata::{UserRole, Metadata as MetadataTools};
use makoto_grpc::pkg::user;
use makoto_lib::errors::prelude::ResultedOption;

pub struct RequestTools;

impl RequestTools {
    pub async fn parse_user_id_request(req: tonic::Request<user::UserId>) -> Result<(Uuid, Uuid, UserRole), tonic::Status> {
        let (metadata, _, req) = req.into_parts();
        let (user_role, user_id) = MetadataTools::get_user_role_and_id_from_metadata(&metadata)
            .unwrap_or_unauthorized("user-id not found on metadata")?;

        let user_id = Uuid::try_parse(&user_id).invalid_argument_error()?;
        let requested_user_id = Uuid::try_parse(&req.user_id).invalid_argument_error()?;

        Ok((user_id, requested_user_id, user_role))
    }
}