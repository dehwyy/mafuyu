use tonic::{Request, Response, Status};
use tonic::codegen::Body;
use makoto_grpc::pkg::cdn::cdn_rpc_server::CdnRpc;
use makoto_grpc::pkg::cdn::{UploadNewImageRequest, UploadNewImageResponse};
use makoto_lib::errors::HandleError;

use mafuyu_nats::payload::cdn as nats_cdn;

pub struct CdnRpcServiceImplementation {
    url_base: String,
    nats_client: async_nats::Client,
    repo: crate::repo::Repo
}

impl CdnRpcServiceImplementation {
    pub fn new(nats_client: async_nats::Client, repo: crate::repo::Repo) -> Self {
        Self {
            url_base: "https://127.0.0.1/v1/static".to_string(),
            nats_client,
            repo
        }
    }
}

#[tonic::async_trait]
impl CdnRpc for CdnRpcServiceImplementation {
    async fn upload_new_image(&self, req: Request<UploadNewImageRequest>) -> Result<Response<UploadNewImageResponse>, Status> {
        let req = req.into_inner();

        let mut i = 0;
        let filename = match loop {
            if i > 5 { break None }

            let filename = format!("{}.{}", req.keyword, uuid::Uuid::new_v4().to_string());
            if self.repo.has_value_by_key(&filename).await.map_err(|err| Status::internal(err.to_string()))? {
                break Some(filename);
            };

            i+=1
        } {
            Some(filename) => {
                self.repo.reserve_key(&filename).await.map_err(|err| Status::internal(err.to_string()))?;

                filename
            },
            None => return Err(Status::aborted("cannot generate new image filename 6x times"))
        };

        let image_ext = "png".to_string();

        let payload = serde_json::to_vec(&nats_cdn::PublishImageRequest {
            filename: filename.clone(),
            base64_image: req.image_base64,
            image_ext: image_ext.clone()
        }).map_err(|err| Status::internal(err.to_string()))?;

        self.nats_client.publish(nats_cdn::subject::PUBLISH_IMAGE, payload.into()).await.handle()?;

        Ok(Response::new(UploadNewImageResponse {
            full_url: format!("{base}/{file}.{ext}", base=self.url_base, file=filename, ext=image_ext),
            filename,
        }))
    }
}