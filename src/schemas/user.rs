use diesel::{prelude::*, QueryResult};
use juniper::{graphql_object, FieldResult, GraphQLInputObject};
use validator::Validate;

use crate::entity::{Book, NewUser, User};

use super::{Connection, DataContext};

#[graphql_object(Context = DataContext)]
impl User {
    pub fn id(&self) -> &i32 {
        &self.id
    }
    #[graphql(description = "user's name")]
    pub fn username(&self) -> &str {
        &self.username
    }

    #[graphql(description = "user's email")]
    pub fn email(&self) -> &str {
        &self.email
    }
    #[graphql(description = "user's bio")]
    pub fn bio(&self) -> &str {
        &self.bio
    }
    #[graphql(description = "user's avatar")]
    pub fn avatar(&self) -> &str {
        &self.avatar
    }

    #[graphql(description = "user's book history")]
    pub fn favorites(&self, ctx: &DataContext) -> FieldResult<Vec<Book>> {
        use crate::schema::{
            books::{self, dsl::*},
            favorite_books::{self, dsl::*},
        };
        let ref mut conn = ctx.database.get()?;
        let list = favorite_books
            .filter(favorite_books::user_id.eq(self.id))
            .inner_join(books.on(favorite_books::book_id.eq(books::id)))
            .select(books::all_columns)
            .get_results(conn)?;
        Ok(list)
    }
}

#[derive(Debug, GraphQLInputObject, Validate)]
pub struct NewUserInput {
    #[graphql(description = "user's name")]
    pub username: String,
    #[graphql(description = "user's email")]
    #[validate(email(message = "email must be a valid email"))]
    pub email: String,
    #[graphql(description = "user's password,at least 6 chapters")]
    pub password: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, GraphQLInputObject)]
pub struct UpdateUserInput {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, juniper::GraphQLInputObject, Validate)]
pub struct UserLoginInput {
    #[graphql(description = "user's email")]
    #[validate(email(message = "email must be a valid email"))]
    pub email: String,
    #[graphql(description = "user's password,at least 6 chapters")]
    pub password: String,
}

pub struct UserLoginOutput {
    pub token: String,
    pub account: User,
}

#[graphql_object(Context = DataContext)]
impl UserLoginOutput {
    pub async fn token(&self) -> &str {
        &self.token
    }

    pub async fn account(&self) -> &User {
        &self.account
    }
}

pub fn regist_by_email<'a>(conn: &'a mut Connection, entity: NewUser<'a>) -> QueryResult<User> {
    use crate::schema::users::dsl::*;
    diesel::insert_into(users)
        .values(entity)
        .get_result::<User>(conn)
}

pub fn find_by_email<'a>(conn: &'a mut Connection, em: &'a str, pa: &'a str) -> QueryResult<User> {
    use crate::schema::users::dsl::*;
    users
        .filter(email.eq(em))
        .filter(password.eq(pa))
        .limit(1)
        .get_result::<User>(conn)
}
