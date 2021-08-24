use diesel::{prelude::*, QueryResult};
use validator::Validate;

use crate::{
    claims::{create_jwt, hash, Claims},
    model::{NewUser, User},
};

use super::Connection;


    // #[graphql(description = "user's book history")]
    // pub fn favorites(&self, ctx: &UserContext) -> FieldResult<Vec<Book>> {
    //     use crate::schema::{
    //         books::{self, dsl::*},
    //         favorite_books::{self, dsl::*},
    //     };
    //     let ref mut conn = ctx.database.get()?;
    //     let list = favorite_books
    //         .filter(favorite_books::user_id.eq(self.id))
    //         .inner_join(books.on(favorite_books::book_id.eq(books::id)))
    //         .select(books::all_columns)
    //         .get_results(conn)?;
    //     Ok(list)
    // }


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

async fn signup(ctx: &UserContext, entity: NewUserInput) -> FieldResult<User> {
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
        signup(conn, ur).map_err(|err| FieldError::from(err.to_string()))
    }
    fn login(ctx: &UserContext, entity: UserLoginInput) -> FieldResult<UserLoginOutput> {
        entity.validate()?;
        let hashed = hash(&entity.password);
        let ref mut conn = ctx.database.get()?;
        let user = login(conn, &entity.email, &hashed)?;
        let claims = Claims::new(user.id);
        let user_login_output = UserLoginOutput {
            token: create_jwt(claims)?,
            account: user,
        };
        Ok(user_login_output)
    }

