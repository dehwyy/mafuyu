use makoto_grpc::pkg::integrations::GetBasicUserResponse;
use crate::provider::ProviderRequestError;

mod url {
    pub const GITHUB_PROFILE_URL: &str = "https://api.github.com/user";
}

mod payload {
    #[derive(serde::Deserialize, Debug)]
    pub struct GithubUserResponse {
        pub id: i32,
        pub email: Option<String>,
        pub login: String, // actually username
        pub avatar_url: Option<String>
    }
}


pub struct Github;

#[tonic::async_trait]
impl crate::provider::ProviderTrait for Github {
    async fn get_basic_user(&self, access_token: String) -> Result<GetBasicUserResponse, ProviderRequestError> {
        let http_client = reqwest::Client::new();

        let response= http_client.get(url::GITHUB_PROFILE_URL)
            .header("Authorization", format!("Bearer {token}", token=access_token))
            .header("Accept", "application/json") // json response
            .header("User-Agent", "Mafuyu-App")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send().await.map_err(|err| ProviderRequestError::RequestError(err.to_string()))?;


        let response = response
            .json::<payload::GithubUserResponse>() // response into struct
            .await.map_err(|err| ProviderRequestError::CannotDeserialize(err.to_string()))?;

        Ok(GetBasicUserResponse {
            username: response.login,
            email: response.email,
            picture: response.avatar_url,
            provider_id: response.id.to_string()
        })
    }
}

