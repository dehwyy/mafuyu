use tonic::{Request, Response, Status};
use makoto_grpc::pkg::cdn::cdn_rpc_server::CdnRpc;
use makoto_grpc::pkg::cdn::{UploadNewImageRequest, UploadNewImageResponse};
use makoto_grpc::errors::GrpcHandleError;
use mafuyu_lib::errors::prelude::*;

use mafuyu_nats::payload::cdn as nats_cdn;

pub struct CdnRpcServiceImplementation {
    url_base: String,
    nats_client: async_nats::Client,
    repo: crate::repo::Repo
}

impl CdnRpcServiceImplementation {
    pub fn new(nats_client: async_nats::Client, repo: crate::repo::Repo) -> Self {
        Self {
            url_base: "https://localhost:6996/v1/static".to_string(),
            nats_client,
            repo
        }
    }
}

#[tonic::async_trait]
impl CdnRpc for CdnRpcServiceImplementation {
    async fn upload_new_image(&self, req: Request<UploadNewImageRequest>) -> Result<Response<UploadNewImageResponse>, Status> {
        let req = req.into_inner();

        // Iterates at most 5 times to generate a unique filename
        let mut i = 0;
        let filename = match loop {
            // strict `==` is acceptable, but why not `>=`? :3
            if i >= 5 { break None }

            let filename = format!("{}.{}", req.keyword, uuid::Uuid::new_v4().to_string());
            // if `generated filename` is unique (not exists in db) -> break the loop and return `new filename`
            if !self.repo.has_value_by_key(&filename).await.internal_error()? {
                break Some(filename);
            }

            i+=1
        } {
            Some(filename) => {
                self.repo.reserve_key(&filename).await.internal_error()?;

                filename
            },
            None => return Err(Status::aborted("cannot generate new image filename 5x times"))
        };

        let payload = serde_json::to_vec(&nats_cdn::PublishImageRequest {
            filename: filename.clone(),
            base64_image: req.image_base64,
            image_ext: req.ext.clone(),
        }).internal_error()?;

        self.nats_client.publish(nats_cdn::subject::PUBLISH_IMAGE, payload.into()).await.handle()?;

        Ok(Response::new(UploadNewImageResponse {
            full_url: format!("{base}/{filename}.{ext}", base=self.url_base, ext=req.ext),
            filename,
        }))
    }
}