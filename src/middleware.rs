use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::{AuthenticationError, bearer::{BearerAuth, Config}};

pub async fn bearer_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    if credentials.token() == "mF_9.B5f-4.1JqM" {
        Ok(req)
    } else {
        let config = req
            .app_data::<Config>()
            .map(|data| data.clone())
            .unwrap_or_else(Default::default)
            .scope("urn:example:channel=HBO&urn:example:rating=G,PG-13");

        Err(AuthenticationError::from(config).into())
    }
}
