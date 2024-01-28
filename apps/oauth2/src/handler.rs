use actix_web::{Responder, web, get};
use actix_web::dev::HttpServiceFactory;
use crate::oauth2::OAuth2;

pub struct State {
    pub oauth2: OAuth2
}

pub struct Service;

impl Service {
    pub fn new() ->  impl HttpServiceFactory {
        web::scope("/v1")
            .app_data(State { oauth2: OAuth2::new() })
            .service(get_redirect_url)
            .service(get_user_by_token)
            .service(exchange_code_to_token)
            .service(refresh_token)
    }
}

mod routes {
    use actix_web::{error::ErrorNotFound, get, Result as HttpResult, post, put, Responder, web};
    use actix_web::error::{ErrorBadRequest, ErrorUnauthorized};
    use super::State;
    use crate::oauth2::{OAuth2Provider, RefreshError};

    mod tools {
        use actix_web::error::ErrorBadGateway;
        use super::HttpResult;
        use crate::oauth2::{OAuth2, OAuth2Provider, OAuth2ProviderName};

        pub fn get_provider_by_str<'a>(oauth2: &'a OAuth2, provider: &str) -> HttpResult<&'a impl OAuth2Provider> {
            let provider_name = match OAuth2ProviderName::from_str(provider) {
                Some(provider_name) => Ok(provider_name),
                None => Err(ErrorBadGateway("provider name not found"))
            }?;

            Ok(oauth2.get_provider(provider_name))
        }
    }

    mod query {
        use serde::Deserialize;

        #[derive(Deserialize)]
        pub struct GetRedirectUrl {
            pub provider: String
        }

        #[derive(Deserialize)]
        pub struct GetUserByToken {
            pub provider: String,
            pub access_token: String
        }
    }

    mod request {
        use serde::Deserialize;

        #[derive(Deserialize)]
        pub struct ExchangeCodeToToken {
            pub provider: String,
            pub code: String
        }

        #[derive(Deserialize)]
        pub struct RefreshToken {
            pub provider: String,
            pub refresh_token: Option<String>
        }
    }

    mod response {
        use serde::Serialize;

        #[derive(Serialize)]
        pub struct GetRedirectUrl {
            pub redirect_url: String,
            pub csrf_token: String
        }
    }

    #[get("/redirect_url")]
    pub async fn get_redirect_url(state: web::Data<State>, query: web::Query<query::GetRedirectUrl>) -> HttpResult<impl Responder> {
        let provider = tools::get_provider_by_str(&state.oauth2, &query.provider)?;

        let (redirect_url , csrf_token) = provider.create_redirect_url();

        Ok(web::Json(response::GetRedirectUrl {
            redirect_url,
            csrf_token: csrf_token.secret().to_string()
        }))
    }

    #[get("/user_by_token")]
    async fn get_user_by_token(state: web::Data<State>, query: web::Query<query::GetUserByToken>) -> HttpResult<impl Responder> {
        let provider = tools::get_provider_by_str(&state.oauth2, &query.provider)?;

        let provider_response = provider.get_user_by_token(query.access_token.clone())
            .await.map_err(|err| ErrorNotFound(err))?;

        Ok(web::Json(provider_response))
    }

    #[post("/exchange_code_to_token")]
    async fn exchange_code_to_token(state: web::Data<State>, req: web::Json<request::ExchangeCodeToToken>) -> HttpResult<impl Responder> {
        let provider = tools::get_provider_by_str(&state.oauth2, &req.provider)?;

        let token = provider.exchange_code_to_token(req.code.clone()).await.map_err(|err| {
            ErrorBadRequest(err)
        })?;

        Ok(web::Json(token))

    }

    #[put("/refresh_token")]
    async fn refresh_token(state: web::Data<State>, req: web::Json<request::RefreshToken>) -> HttpResult<impl Responder> {
        let provider = tools::get_provider_by_str(&state.oauth2, &req.provider)?;

        let token = provider.refresh().await.map_err(|err| {
            match err {
                RefreshError::NotSupported => ErrorBadRequest("provider doesn't support token refreshment"),
                RefreshError::Internal => ErrorUnauthorized("cannot refresh oauth2 token")
            }
        })?;

        Ok(web::Json(token))
    }

}

use routes::*;