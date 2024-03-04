use makoto_grpc::pkg::integrations::GetBasicUserResponse;
use crate::provider::ProviderRequestError;

mod url {
    pub const GOOGLE_PROFILE_URL: &str = "https://www.googleapis.com/oauth2/v1/userinfo";
}

mod payload {
    #[derive(serde::Deserialize, Debug)]
    pub struct GoogleUserResponse {
        pub id: String,
        pub email: Option<String>,
        pub name: String, //  username
        pub picture: Option<String>
    }
}


pub struct Google;

#[tonic::async_trait]
impl crate::provider::ProviderTrait for Google {
    async fn get_basic_user(&self, access_token: String) -> Result<GetBasicUserResponse, ProviderRequestError> {
        let http_client = reqwest::Client::new();

        let response= http_client.get(url::GOOGLE_PROFILE_URL)
            .query(&[("alt", "json"), ("access_token", &access_token)])
            // .header("Accept", "application/json") // json response
            .send().await.map_err(|err| ProviderRequestError::RequestError(err.to_string()))?;

        let response = response
            .json::<payload::GoogleUserResponse>() // response into struct
            .await.map_err(|err| ProviderRequestError::CannotDeserialize(err.to_string()))?;

        // As `picture` would have `size` in the response, we should strip it
        // @example https://lh3.googleusercontent.com/a/ACg8ocLE4oqn1c6KC1jgzJB3vL3hhJBDEKxINbHfQmG34Ubrozk=s96-c
        // @output https://lh3.googleusercontent.com/a/ACg8ocLE4oqn1c6KC1jgzJB3vL3hhJBDEKxINbHfQmG34Ubrozk

        let picture = response.picture.map(|v| v.split("=s96-c").collect::<Vec<_>>().join(""));

        Ok(GetBasicUserResponse {
            username: response.name,
            email: response.email,
            provider_id: response.id,
            picture,
        })
    }
}

