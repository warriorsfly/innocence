use actix_web::web::{Data, Json};

use crate::{
    database::{self, Database, NewUser, NewUserInput, User, UserLoginInput, UserLoginOutput},
    errors::Error,
    helpers::respond_json,
    plugins::hash,
    validate::validate,
};

pub async fn signup(
    database: Data<Database>,
    entity: Json<NewUserInput>,
) -> Result<Json<User>, Error> {
    validate(&entity)?;
    let ref mut conn = database.get()?;
    let psw = hash(&entity.password);
    let ur = NewUser {
        username: &entity.username,
        email: &entity.email,
        password: &psw,
        bio: "",
        avatar: "",
    };
    let us = database::signup(conn, &ur)?;
    respond_json(us)
}
pub async fn login(
    database: Data<Database>,
    entity: Json<UserLoginInput>,
) -> Result<Json<UserLoginOutput>, Error> {
    validate(&entity)?;
    let ref mut conn = database.get()?;
    let res = database::login(conn, &entity.name, &entity.password)?;
    respond_json(res)
}
