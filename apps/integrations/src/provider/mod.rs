use makoto_grpc::pkg::integrations::GetBasicUserResponse;

mod github;
mod google;

pub enum ProviderRequestError {
    RequestError(String),
    CannotDeserialize(String)
}
impl ProviderRequestError {
    pub fn as_tonic_status(&self) -> tonic::Status {
        match self {
            Self::CannotDeserialize(s) => tonic::Status::internal(s),
            Self::RequestError(s) => tonic::Status::internal(s)
        }
    }
}

pub enum Provider {
    Google,
    Github
}
impl Provider {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "google" => Some(Self::Google),
            "github" => Some(Self::Github),
            _ => None
        }
    }

    pub fn as_provider(&self) -> Box<dyn ProviderTrait> {
        match self {
            Self::Github => Box::new(github::Github),
            Self::Google => Box::new(google::Google)
        }
    }
}

#[tonic::async_trait]
pub trait ProviderTrait: Send {
    async fn get_basic_user(&self, access_token: String) -> Result<GetBasicUserResponse, ProviderRequestError>;
}