use super::environment::EnvironmentVariables;

pub struct JWT {}
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    http::StatusCode,
    Error, HttpMessage,
};
use chrono::{Duration, Utc};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::future::{ready, Ready};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

impl JWT {
    pub fn jwt_encode(user_id: String) -> Result<std::string::String, jsonwebtoken::errors::Error> {
        let environment_variables = EnvironmentVariables::initialize();
        let jwt_secret = environment_variables.jwt_secret.as_str();
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::minutes(60)).timestamp() as usize;
        let claims: Claims = Claims {
            sub: user_id,
            exp,
            iat,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_ref()),
        );

        return token;
    }

    pub fn jwt_decode(token: String) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        let environment_variables = EnvironmentVariables::initialize();
        let jwt_secret = environment_variables.jwt_secret.as_str();
        return decode::<Claims>(
            &token,
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::default(),
        );
    }
}
pub struct JWTAuthentication;
pub struct JWTAuthenticationMiddleware<S> {
    pub service: S,
}

impl<S, B> Transform<S, ServiceRequest> for JWTAuthentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JWTAuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JWTAuthenticationMiddleware { service }))
    }
}

impl<S, B> Service<ServiceRequest> for JWTAuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let token = request
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok()?.strip_prefix("Bearer "));
        if token.is_none() {
            let json_error = json!({
                "message":"missing or invalid authorization header",
                "statusCode": StatusCode::UNAUTHORIZED.as_u16(),
            });
            return Box::pin(ready(Err(ErrorUnauthorized(json_error))));
        }
        let decode: Result<TokenData<Claims>, jsonwebtoken::errors::Error> =
            JWT::jwt_decode(token.unwrap().to_string());

        match decode {
            Ok(TokenData { header: _, claims }) => {
                let user_id = claims.sub;
                request.extensions_mut().insert(user_id);
            }
            Err(_) => {
                let json_error = json!({
                    "message":"invalid JWT token",
                    "statusCode": StatusCode::UNAUTHORIZED.as_u16(),
                });
                return Box::pin(ready(Err(ErrorUnauthorized(json_error))));
            }
        }
        return Box::pin(self.service.call(request));
    }
}
