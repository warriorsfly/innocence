use actix_web::web::{Data, Json};
use diesel::prelude::*;
use validator::Validate;

use crate::{
    claims::{create_jwt, hash, Claims},
    datasource::{self, Database, NewUser, NewUserInput, User, UserLoginInput, UserLoginOutput},
    errors::Error,
};

pub(crate) async fn signup(
    database: Data<Database>,
    entity: Json<NewUserInput>,
) -> Result<User, Error> {
    // entity.validate()?;
    let ref mut conn = database.get()?;
    let psw = hash(&entity.password);
    let ur = NewUser {
        username: &entity.username,
        email: &entity.email,
        password: &psw,
        bio: "",
        avatar: "",
    };
    datasource::signup(conn, &ur)
}
pub(crate) fn login(
    database: &Data<Database>,
    entity: Json<UserLoginInput>,
) -> Result<UserLoginOutput, Error> {
    entity.validate()?;
    let ref mut conn = database.get()?;
    datasource::login(conn, &entity.name, &entity.password)
}
