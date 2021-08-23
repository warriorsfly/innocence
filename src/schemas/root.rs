use diesel::prelude::*;
use juniper::{graphql_object, EmptySubscription, FieldError, FieldResult, RootNode};
use validator::Validate;

use crate::{
    claims::{create_jwt, hash, Claims},
    entity::{NewUser, User},
    schema::users,
};

use super::{
    find_by_email, regist_by_email, DataContext, NewUserInput, UserLoginInput, UserLoginOutput,
};
pub struct Query;

#[graphql_object(context = DataContext)]
impl Query {
    #[graphql(arguments(id(description = "id of the user")))]
    async fn users(ctx: &DataContext, id: i32) -> FieldResult<User> {
        let ref mut conn = ctx.database.get()?;
        let ur = users::table.find(id).get_result(conn)?;
        Ok(ur)
    }
    // fn search(
    //     context: &DataContext,
    //     #[graphql(description = "id of the user")] id: i32,
    // ) -> Vec<Book> {
    //     // context.get_user(&id)
    // }
}

pub struct Mutation;

#[graphql_object(Context = DataContext)]
impl Mutation {
    #[graphql(description = "sign up a new user by email")]
    async fn signup(ctx: &DataContext, entity: NewUserInput) -> FieldResult<User> {
        entity.validate()?;
        let ref mut conn = ctx.database.get()?;
        let psw = hash(&entity.password);
        let ur = NewUser {
            username: &entity.username,
            email: &entity.email,
            password: &psw,
            bio: &entity.bio.unwrap_or("".into()),
            avatar: &entity.avatar.unwrap_or("".into()),
        };
        regist_by_email(conn, ur).map_err(|err| FieldError::from(err.to_string()))
    }

    #[graphql(description = "login into inno")]
    fn login(ctx: &DataContext, entity: UserLoginInput) -> FieldResult<UserLoginOutput> {
        entity.validate()?;
        let hashed = hash(&entity.password);
        let ref mut conn = ctx.database.get()?;
        let user = find_by_email(conn, &entity.email, &hashed)?;
        let claims = Claims::new(user.id);
        let user_login_output = UserLoginOutput {
            token: create_jwt(claims)?,
            account: user,
        };
        Ok(user_login_output)
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<DataContext>>;
pub(crate) fn init_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::<DataContext>::new())
}
