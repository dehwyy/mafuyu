use std::borrow::BorrowMut;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use makoto_db::repo::user::GetUserRecordBy;
use makoto_grpc::pkg::user as rpc;
use makoto_grpc::pkg::cdn;
use makoto_lib::errors::{HandleError, SafeUnwrapWithMessage};
use makoto_logger::info;
use crate::repo::user::EditPrimitiveUserPayload;
use crate::tools::image::{Image, ImageType};

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
        if let Some(Some(image)) = req.picture.map(|image| Image::parse(&image)) {
            picture = Some(match image {
                ImageType::Base64(base64_image) => {
                    let response = self.cdn_client.clone().borrow_mut().upload_new_image(Request::new(cdn::UploadNewImageRequest {
                        image_base64: base64_image,
                        keyword: req.user_id.clone()
                    })).await?.into_inner();

                    response.full_url
                },
                ImageType::Url(url_image) => url_image
            });
        }

        self.user_repo.edit_primitive_user(EditPrimitiveUserPayload {
            user_id,
            location: req.location,
            birthday: req.birthday,
            pseudonym: req.pseudonym,
            bio: req.bio,
            picture,
        }).await.handle()?;

        let (update_languages, ) = tokio::join!(update_languages_fut);
        update_languages.handle()?;

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