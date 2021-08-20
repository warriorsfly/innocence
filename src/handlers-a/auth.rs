use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct LoginData {
    #[validate(email(message = "email must be a valid email"))]
    pub email: String,

    #[validate(length(
        min = 6,
        message = "password is required and must be at least 6 characters"
    ))]
    pub password: String,
}
