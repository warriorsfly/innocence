use actix_web::web::{block, Data, Json};
use innocence_database::{ Database, dao, entity::{ NewUser, NewUserInput, User, UserLoginInput, UserLoginOutput}};
use innocence_utils::{validate, hash, Error};
use crate::helpers::respond_json;

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
    let res = block(move || dao::login(&pool, &entity.name, &entity.password)).await??;
    respond_json(res)
}
