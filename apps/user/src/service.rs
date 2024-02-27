use std::borrow::BorrowMut;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use logger::{info, warn};
use makoto_db::repo::user::GetUserRecordBy;
use makoto_grpc::errors::GrpcHandleError;
use makoto_grpc::pkg::user as rpc;
use makoto_grpc::pkg::cdn;
use makoto_grpc::pkg::user::*;
use makoto_lib::errors::prelude::*;

use crate::repo::user::{GetUsersPayload, EditPrimitiveUserPayload};
use crate::tools::image::{Image, ImageType};
use crate::tools::request::RequestTools;

pub struct UserRpcServiceImplementation<T = tonic::transport::Channel> {
    cdn_client: cdn::cdn_rpc_client::CdnRpcClient<T>,
    user_repo: crate::repo::user::UserRepo,
    languages_repo: crate::repo::languages::LanguagesRepo,
    followers_friends_repo: crate::repo::followers_friends::FollowersFriendsRepo,
    blocked_user_repo: crate::repo::blocked_users::BlockedUsersRepo
}

impl UserRpcServiceImplementation {
    pub async fn new(
        user_repo: crate::repo::user::UserRepo,
        languages_repo: crate::repo::languages::LanguagesRepo,
        followers_friends_repo: crate::repo::followers_friends::FollowersFriendsRepo,
        blocked_user_repo: crate::repo::blocked_users::BlockedUsersRepo
    ) -> Self {

        let clients = makoto_grpc::RpcClients::get_all_client().await;

        Self {
            cdn_client: clients.cdn_client.unwrap(),
            user_repo,
            languages_repo,
            followers_friends_repo,
            blocked_user_repo
        }
    }
}

#[tonic::async_trait]
impl rpc::user_rpc_server::UserRpc for UserRpcServiceImplementation {
    async fn create_user(&self, req: Request<rpc::CreateUserRequest>) -> Result<Response<()>, Status> {
        let req = req.into_inner();

        let user_id = Uuid::try_parse(&req.user_id).invalid_argument_error()?;

        self.user_repo.create_basic_user(user_id, req.picture).await.handle()?;

        Ok(Response::new(()))
    }

    async fn edit_user(&self, req: Request<rpc::EditUserRequest>) -> Result<Response<()>, Status> {
        let req = req.into_inner();

        let user_id = Uuid::try_parse(&req.user_id).invalid_argument_error()?;

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

        // according prost to docs, `oneof` returns `Option`
        let get_by = match req.login.unwrap_or_internal("weirdo error")? {
            rpc::get_user_request::Login::Username(username) => GetUserRecordBy::Username(username),
            rpc::get_user_request::Login::UserId(user_id) => GetUserRecordBy::UserId(Uuid::try_parse(&user_id).invalid_argument_error()?),
        };

        let (user , username) = self.user_repo.get_user(get_by).await.handle()?;
        let languages_future = self.languages_repo.get_languages(&user.user_id);


        let (languages,) = tokio::join!(languages_future);
        let languages = languages.handle()?;


        Ok(Response::new(rpc::GetUserResponse {
            user_id: user.user_id.to_string(),
            username,
            location: user.location,
            birthday: None,
            pseudonym: user.pseudonym,
            bio: user.bio,
            picture: user.picture,
            languages,
        }))

    }

    async fn get_basic_user(&self, req: Request<GetUserRequest>) -> Result<Response<GetBasicUserResponse>, Status> {
            let req = req.into_inner();

            let get_by = match req.login.unwrap_or_internal("weirdo error")? {
                rpc::get_user_request::Login::Username(username) => GetUserRecordBy::Username(username),
                rpc::get_user_request::Login::UserId(user_id) => GetUserRecordBy::UserId(Uuid::try_parse(&user_id).invalid_argument_error()?),
            };

            let (user , username) = self.user_repo.get_user(get_by).await.handle()?;

            Ok(Response::new(GetBasicUserResponse {
                user_id: user.user_id.to_string(),
                username,
                picture: user.picture
            }))
    }

    async fn get_users(&self, req: Request<GetUsersRequest>) -> Result<Response<GetUsersResponse>, Status> {
        let req = req.into_inner();

        let users = self.user_repo.get_users(GetUsersPayload {
            pattern: req.pattern,
            limit: req.limit,
            offset: req.offset,
        }).await.handle()?;

        let users = users.iter().filter_map(|(user, user_cred)| {
            user_cred.clone().map(|user_cred| {
                GetUserResponse {
                    user_id: user.user_id.to_string(),
                    username: user_cred.username.clone(),
                    location: user.location.clone(),
                    birthday: None,
                    pseudonym: user.pseudonym.clone(),
                    bio: user.bio.clone(),
                    picture: user.picture.clone(),
                    languages: vec!()
                }
            })
        }).collect::<Vec<_>>();

        Ok(Response::new(GetUsersResponse { users }))

    }

    async fn get_users_i_ds(&self, req: Request<GetUsersRequest>) -> Result<Response<GetUsersIDsResponse>, Status> {
        let req = req.into_inner();

        let users = self.user_repo.get_users(GetUsersPayload {
            pattern: req.pattern,
            limit: req.limit,
            offset: req.offset,
        }).await.handle()?;

        Ok(Response::new(GetUsersIDsResponse {
            user_ids: users.iter().map(|(user, _)| user.user_id.to_string()).collect()
        }))
    }

    async fn get_user_friends(&self, req: Request<GetUserFriendsRequest>) -> Result<Response<GetUserFriendsResponse>, Status> {
        let req = req.into_inner();

        let user_id = Uuid::try_parse(&req.user_id).invalid_argument_error()?;

        let friends = self.followers_friends_repo.get_friends(&user_id, req.limit).await.handle()?;
        let friends = friends.iter().map(|v| v.friend_user_id.to_string()).collect::<Vec<_>>();

        Ok(Response::new(GetUserFriendsResponse { friends }))
    }

    async fn get_user_followers(&self, req: Request<GetUserFollowersRequest>) -> Result<Response<GetUserFollowersResponse>, Status> {
        let req = req.into_inner();

        let user_id = Uuid::try_parse(&req.user_id).invalid_argument_error()?;

        let followers = self.followers_friends_repo.get_followers(&user_id, req.limit).await.handle()?;
        let followers = followers.iter().map(|v| v.user_id.to_string()).collect::<Vec<_>>();

        Ok(Response::new(GetUserFollowersResponse { followers }))
    }

    async fn get_blocked_users(&self, req: Request<GetBlockedUsersRequest>) -> Result<Response<GetBlockedUsersResponse>, Status> {
        let req = req.into_inner();

        let user_id = Uuid::try_parse(&req.user_id).invalid_argument_error()?;

        let blocked_users = self.blocked_user_repo.get_blocked_users(&user_id).await.handle()?;
        let blocked_users = blocked_users.iter().map(|v| v.blocked_user_id.to_string()).collect::<Vec<_>>();

        Ok(Response::new(GetBlockedUsersResponse { blocked_users }))
    }

    async fn follow_user(&self, req: Request<UserId>) -> Result<Response<()>, Status> {
        let (user_id, user_to_add_id, _) = RequestTools::parse_user_id_request(req).await?;

        let r = self.followers_friends_repo.follow(&user_id, &user_to_add_id).await.handle()?;

        Ok(Response::new(r))
    }

    async fn unfollow_user(&self, req: Request<UserId>) -> Result<Response<()>, Status> {
        let (user_id, user_to_add_id, _) = RequestTools::parse_user_id_request(req).await?;

        let r = self.followers_friends_repo.unfollow(&user_id, &user_to_add_id).await.handle()?;

        Ok(Response::new(r))
    }

    async fn block_user(&self, req: Request<UserId>) -> Result<Response<()>, Status> {
        let (user_id, user_to_add_id, _) = RequestTools::parse_user_id_request(req).await?;

        let r = self.blocked_user_repo.block(&user_id, &user_to_add_id).await.handle()?;

        Ok(Response::new(r))
    }

    async fn unblock_user(&self, req: Request<UserId>) -> Result<Response<()>, Status> {
        let (user_id, user_to_add_id, _) = RequestTools::parse_user_id_request(req).await?;

        let r = self.blocked_user_repo.unblock(&user_id, &user_to_add_id).await.handle()?;

        Ok(Response::new(r))
    }


}


//
