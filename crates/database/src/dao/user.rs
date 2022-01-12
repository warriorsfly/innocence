use diesel::prelude::*;
use innocence_utils::{create_jwt, hash, Claims,Error};
use crate::{
    entity::{NewUser, User, UserLoginOutput},
    Database,
};

pub(crate) fn signup<'a>(pool: &'a Database, entity: &'a NewUser) -> Result<User, Error> {
    use crate::schema::users::dsl::*;
    let ref mut conn = pool.get()?;
    diesel::insert_into(users)
        .values(entity)
        .get_result::<User>(conn)
        .map_err(|err| Error::DataBaseError(err.to_string()))
}

pub(crate) fn login<'a>(
    pool: &'a Database,
    name: &'a str,
    psw: &'a str,
) -> Result<UserLoginOutput, Error> {
    use crate::schema::users::dsl::*;
    let hashed = hash(psw);
    let ref mut conn = pool.get()?;
    let user: User = users
        .filter(username.eq(name))
        .filter(password.eq(&hashed))
        .get_result(conn)?;
    let claims = Claims::new(user.id);
    let user_login_output = UserLoginOutput {
        token: create_jwt(claims)?,
        account: user,
    };
    Ok(user_login_output)
}
