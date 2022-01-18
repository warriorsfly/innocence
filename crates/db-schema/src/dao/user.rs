use crate::{
    entity::{NewUser, User},
    Database,
};
use diesel::prelude::*;
use innocence_utils::{hash, Error};

pub fn signup<'a>(pool: &'a Database, entity: &'a NewUser) -> Result<User, Error> {
    use crate::schema::users::dsl::*;
    let ref mut conn = pool.get()?;
    diesel::insert_into(users)
        .values(entity)
        .get_result::<User>(conn)
        .map_err(|err| Error::DataBaseError(err.to_string()))
}

pub fn login<'a>(pool: &'a Database, name: &'a str, psw: &'a str) -> Result<User, Error> {
    use crate::schema::users::dsl::*;
    let hashed = hash(psw);
    let ref mut conn = pool.get()?;
    let user: User = users
        .filter(username.eq(name))
        .filter(password.eq(&hashed))
        .get_result(conn)?;

    Ok(user)
}
