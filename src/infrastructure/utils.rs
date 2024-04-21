use axum::{body::Body, http::Response, response::IntoResponse};

use crate::commons::exception::api_exception::ApiException;

impl IntoResponse for ApiException {

    fn into_response(self) -> Response<Body> {
        Response::builder()
        .status(self.status())
        .body(Body::from(self.message()))
        .unwrap()
    }

}