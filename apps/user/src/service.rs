use std::borrow::BorrowMut;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use makoto_db::repo::user::GetUserRecordBy;
use makoto_grpc::pkg::user as rpc;
use makoto_grpc::pkg::cdn;
use makoto_lib::errors::{HandleError, SafeUnwrapWithMessage};
use makoto_logger::info;
use crate::repo::user::EditPrimitiveUserPayload;

pub struct UserRpcServiceImplementation<T = tonic::transport::Channel> {
    cdn_client: cdn::cdn_rpc_client::CdnRpcClient<T>,
    user_repo: crate::repo::user::UserRepo,
    languages_repo: crate::repo::languages::LanguagesRepo,
}

impl UserRpcServiceImplementation {
    pub async fn new(
        user_repo: crate::repo::user::UserRepo,
        languages_repo: crate::repo::languages::LanguagesRepo
    ) -> Self {

        let clients = makoto_grpc::RpcClients::get_all_client().await;

        Self {
            cdn_client: clients.cdn_client.unwrap(),
            user_repo,
            languages_repo
        }
    }
}

#[tonic::async_trait]
impl rpc::user_rpc_server::UserRpc for UserRpcServiceImplementation {
    async fn create_user(&self, req: Request<rpc::CreateUserRequest>) -> Result<Response<()>, Status> {
        let req = req.into_inner();

        let user_id = Uuid::try_parse(&req.user_id).map_err(|err| Status::invalid_argument(err.to_string()))?;

        self.user_repo.create_basic_user(user_id, req.picture).await.handle()?;

        Ok(Response::new(()))
    }

    async fn edit_user(&self, req: Request<rpc::EditUserRequest>) -> Result<Response<()>, Status> {
        let req = req.into_inner();

        let user_id = Uuid::try_parse(&req.user_id).map_err(|err| Status::invalid_argument(err.to_string()))?;

        let update_languages_fut = self.languages_repo.set_languages(&user_id, req.languages);

        let mut picture: Option<String> = None;

        if let Some(image) = req.picture {
            if !image.starts_with("http") {
                if let Some(image) = image.split_once(",") {
                    let response = self.cdn_client.clone().borrow_mut().upload_new_image(Request::new(cdn::UploadNewImageRequest {
                        image_base64: image.1.to_string(),
                        keyword: req.user_id.clone()
                    })).await?.into_inner();

                    picture = Some(response.full_url)
                }
            }
        }


        let edit_user_fut = self.user_repo.edit_primitive_user(EditPrimitiveUserPayload {
            user_id,
            location: req.location,
            birthday: req.birthday,
            pseudonym: req.pseudonym,
            bio: req.bio,
            picture,
        });

        let (update_languages, edit_user) = tokio::join!(update_languages_fut, edit_user_fut);
        update_languages.handle()?;
        edit_user.handle()?;

        Ok(Response::new(()))
    }

    async fn get_user(&self, req: Request<rpc::GetUserRequest>) -> Result<Response<rpc::GetUserResponse>, Status> {
        let req = req.into_inner();

        let user = self.user_repo.get_user(GetUserRecordBy::Username(req.username.clone())).await.handle()?;

        Ok(Response::new(rpc::GetUserResponse {
            user_id: user.user_id.to_string(),
            username: req.username,
            location: user.location,
            birthday: None,
            pseudonym: user.pseudonym,
            bio: user.bio,
            picture: user.picture,
            languages: vec!(),
            followers: vec!(),
            friends: vec!()
        }))

    }
}