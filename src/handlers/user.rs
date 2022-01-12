use crate::helpers::respond_json;
use actix_web::web::{block, Data, Json};
use innocence_database::{
    dao,
    entity::{NewUser, User},
    Database,
};
use innocence_utils::{create_jwt, hash, validate, Claims, Error};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct NewUserInput {
    pub username: String,
    #[validate(email(message = "email must be a valid email"))]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserInput {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UserLoginInput {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserLoginOutput {
    pub token: String,
    pub account: User,
}

pub async fn signup(pool: Data<Database>, entity: Json<NewUserInput>) -> Result<Json<User>, Error> {
    validate(&entity)?;
    let psw = hash(&entity.password);

    let us = block(move || {
        let ur = NewUser {
            username: &entity.username,
            email: &entity.email,
            password: &psw,
            bio: "",
            avatar: "",
        };
        dao::signup(&pool, &ur)
    })
    .await??;
    respond_json(us)
}
pub async fn login(
    pool: Data<Database>,
    entity: Json<UserLoginInput>,
) -> Result<Json<UserLoginOutput>, Error> {
    validate(&entity)?;
    let ur = block(move || dao::login(&pool, &entity.name, &entity.password)).await??;
    let claims = Claims::new(ur.id);
    let res = UserLoginOutput {
        token: create_jwt(claims)?,
        account: ur,
    };
    respond_json(res)
}
