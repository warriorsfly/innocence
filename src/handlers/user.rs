use actix_web::web::Data;
use diesel::{prelude::*, QueryResult};
use validator::Validate;

use crate::{
    claims::{create_jwt, hash, Claims},
    errors::Error,
    model::{NewUser, User},
};

use super::{Connection, Database};


async fn signup(database: Data<Database>, entity: NewUserInput) -> Result<User, Error> {
    // entity.validate()?;
    let ref mut conn = database.get()?;
    let psw = hash(&entity.password);
    let ur = NewUser {
        username: &entity.username,
        email: &entity.email,
        password: &psw,
        bio: &entity.bio.unwrap_or("".into()),
        avatar: &entity.avatar.unwrap_or("".into()),
    };
    signup(conn, ur).map_err(|err| FieldError::from(err.to_string()))
}
fn login(database: &Data<Database>, entity: UserLoginInput) -> Result<UserLoginOutput, Error> {
    entity.validate()?;
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
