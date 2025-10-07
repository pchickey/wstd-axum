use axum::extract::Request;
use axum::response::Response;
use std::convert::Infallible;
use tower_service::Service;

pub use wstd_axum_macro::attr_macro_http_server as http_server;

pub async fn serve<S>(
    request: wstd::http::Request<wstd::http::Incoming>,
    mut service: S,
) -> wstd::http::error::Result<wstd::http::Response<wstd::http::Body>>
where
    S: Service<Request, Response = Response, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send,
{
    let resp = service
        .call(
            request.map(|incoming: wstd::http::Incoming| -> axum::body::Body {
                axum::body::Body::new(incoming.into_http_body())
            }),
        )
        .await
        .unwrap_or_else(|err| match err {});
    Ok(resp.map(|body: axum::body::Body| -> wstd::http::Body { body.into() }))
}
