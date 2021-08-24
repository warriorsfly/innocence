use actix_web::guard::Connect;
use diesel::{prelude::*, QueryResult};
use validator::Validate;

use crate::{
    claims::{create_jwt, hash, Claims},
    errors::Error,
    model::{NewUser, User},
};

use super::{Connection, Database};

#[derive(Debug, Validate)]
pub struct NewUserInput {
    pub username: String,
    #[validate(email(message = "email must be a valid email"))]
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}
pub struct UpdateUserInput {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

pub struct UserLoginInput {
    pub email: String,
    pub password: String,
}

pub struct UserLoginOutput {
    pub token: String,
    pub account: User,
}

async fn signup<'a>(conn: &'a mut Connection, entity: NewUserInput) -> Result<User, Error> {
    let psw = hash(&entity.password);
    let ur = NewUser {
        username: &entity.username,
        email: &entity.email,
        password: &psw,
        bio: &entity.bio.unwrap_or("".into()),
        avatar: &entity.avatar.unwrap_or("".into()),
    };
}
fn login(database: &Database, entity: UserLoginInput) -> Result<UserLoginOutput, Error> {

    let hashed = hash(&entity.password);
    let ref mut conn = database.get()?;
    let user = login(conn, &entity.email, &hashed)?;
    let claims = Claims::new(user.id);
    let user_login_output = UserLoginOutput {
        token: create_jwt(claims)?,
        account: user,
    };
    Ok(user_login_output)
}
