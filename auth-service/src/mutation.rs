use ::http::header::HeaderName;
use async_graphql::*;
use bcrypt::{DEFAULT_COST, hash, verify};
use common::{
    jwt::{
        external::encode_external_jwt, internal::authenticate_internal_request, jwt_utils::Claims,
    },
    schemas::AuthRole,
    utils::send_graphql_request,
};
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId},
};
use serde_json::json;
use std::{thread, time::Duration};
use validator::Validate;

use crate::schema::{
    Auth, CreateUserInternalResponse, GetUserByEmailOrPseudoInternalResponse, LoginRequest,
    LoginResponse, RegisterRequest, RegisterResponse,
};

// =============================================================================================================================

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn register(&self, ctx: &Context<'_>, user: RegisterRequest) -> Result<RegisterResponse> {
        user.validate()?;

        let query = r#"
            mutation CreateUser($user: CreateUserRequest!) {
                createUser(user: $user) {
                    id
                    firstName
                    lastName
                    pseudo
                    email
                }
            }
        "#;

        let variables = json!({
            "user": {
                "firstName": user.first_name,
                "lastName": user.last_name,
                "pseudo": user.pseudo,
                "email": user.email,
            }
        });

        let payload = json!({
            "query": query,
            "variables": variables,
        });

        let created_user: CreateUserInternalResponse = send_graphql_request(
            "http://users-service:8080/api/users/graphql",
            &payload,
            "createUser",
        )
        .await?;

        let id = ObjectId::parse_str(&created_user.id)?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Auth>("auth");

        let hashed_password = hash(&user.password, DEFAULT_COST)?;

        let mut credential = Auth {
            _id: None,
            password: hashed_password,
            role: AuthRole::User,
            user_id: id,
        };

        let result = collection.insert_one(&credential).await?;

        credential._id = result.inserted_id.as_object_id();

        let token = encode_external_jwt(credential.user_id.to_hex(), credential.role)?;

        let cookie = actix_web::cookie::Cookie::build("session_token", token.clone())
            .path("/")
            .http_only(true)
            .finish();

        let set_cookie = HeaderName::from_static("set-cookie");
        ctx.append_http_header(set_cookie, cookie.to_string());

        Ok(RegisterResponse {
            user_id: credential.user_id.to_hex(),
            email: user.email,
            token,
        })
    }

    // =============================================================================================================================

    async fn login(&self, ctx: &Context<'_>, login_request: LoginRequest) -> Result<LoginResponse> {
        login_request.validate()?;

        let query = r#"
            query GetUserByEmailOrPseudo($credential: String!) {
                getUserByEmailOrPseudo(credential: $credential) {
                    id
                    email
                }
            }
        "#;

        let variables = json!({
            "credential": login_request.credential,
        });

        let payload = json!({
            "query": query,
            "variables": variables,
        });

        let user: GetUserByEmailOrPseudoInternalResponse = send_graphql_request(
            "http://users-service:8080/api/users/graphql",
            &payload,
            "getUserByEmailOrPseudo",
        )
        .await?;

        let user_id = ObjectId::parse_str(&user.id)?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Auth>("auth");

        let credentials = match collection.find_one(doc! { "user_id": user_id  }).await? {
            Some(credentials) => credentials,
            None => return Err("No user with this id exist".into()),
        };

        if let Err(_) | Ok(false) = verify(&login_request.password, &credentials.password) {
            thread::sleep(Duration::from_millis(300));
            return Err("Invalid crendential or password".into());
        }

        let token = encode_external_jwt(credentials.user_id.to_hex(), credentials.role)?;

        let cookie = actix_web::cookie::Cookie::build("session_token", token.clone())
            .path("/")
            .http_only(true)
            .finish();

        let set_cookie = HeaderName::from_static("set-cookie");
        ctx.append_http_header(set_cookie, cookie.to_string());

        Ok(LoginResponse {
            user_id: credentials.user_id.to_hex(),
            email: user.email,
            token,
        })
    }

    // =============================================================================================================================

    async fn logout(&self, ctx: &Context<'_>) -> Result<serde_json::Value> {
        let mut cookie = actix_web::cookie::Cookie::build("session_token", "unused")
            .path("/")
            .http_only(true)
            .finish();
        cookie.make_removal();

        let set_cookie = HeaderName::from_static("set-cookie");
        ctx.append_http_header(set_cookie, cookie.to_string());

        Ok(json!({
            "message": "Logged out successfully"
        }))
    }

    // =============================================================================================================================

    async fn delete_auth_by_user_id(&self, ctx: &Context<'_>, user_id: String) -> Result<Auth> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Forbidden: token invalid or missing")?;
        authenticate_internal_request(token)?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Auth>("auth");

        let user_id = ObjectId::parse_str(&user_id)?;
        let deleted = collection
            .find_one_and_delete(doc! { "user_id": user_id })
            .await?
            .ok_or_else(|| Error::new("No auth credentials found for the given user_id"))?;

        Ok(deleted)
    }

    // =============================================================================================================================
}

// =============================================================================================================================
