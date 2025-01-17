use salvo::{async_trait, writing::Text, Depot, Request, Response, Writer};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("400, Bad Request")]
    BadRequest(String),
    #[error("404, Not Found, {0}")]
    NotFound(String),

    #[error("500, Internal Server Error")]
    InternalServerError(String),
}

pub type ServiceResult<T> = Result<T, ServiceError>;

#[async_trait]
impl Writer for ServiceError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        match self {
            ServiceError::BadRequest(err) => {
                res.status_code(salvo::http::StatusCode::BAD_REQUEST);
                res.render(Text::Json(format!("{{\"error\": \"{}\"}}", err)));
            }
            ServiceError::NotFound(err) => {
                res.status_code(salvo::http::StatusCode::NOT_FOUND);
                res.render(Text::Plain(format!("404, Not Found, {}", err)));
            }
            ServiceError::InternalServerError(err) => {
                res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
                tracing::error!("InternalServerError: {}", err);
                res.render(Text::Plain(format!("500, Internal Server Error, {}", err)));
            }
        }
    }
}

impl From<salvo::http::ParseError> for ServiceError {
    fn from(err: salvo::http::ParseError) -> Self {
        ServiceError::BadRequest(err.to_string())
    }
}
