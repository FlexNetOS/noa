use crate::vault::Vault;
use anyhow::Result;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use tracing::{info, warn};

#[derive(Clone)]
pub struct OAuthProxy {
    pub listen: SocketAddr,
    pub app: String,
    pub vault: Vault,
}

impl OAuthProxy {
    /// Start a tiny HTTP server that captures OAuth codes and stores them in the vault.
    pub async fn start(self) -> Result<()> {
        let proxy = self.clone();
        let make_svc = make_service_fn(move |_| {
            let proxy = proxy.clone();
            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let proxy = proxy.clone();
                    async move { proxy.handle(req).await }
                }))
            }
        });

        info!(addr = ?self.listen, app = %self.app, "OAuth proxy listening");
        Server::bind(&self.listen).serve(make_svc).await?;
        Ok(())
    }

    async fn handle(&self, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        if req.method() != Method::GET || req.uri().path() != "/callback" {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("not found"))
                .unwrap());
        }

        let query = req.uri().query().unwrap_or_default();
        let params: Vec<_> = url::form_urlencoded::parse(query.as_bytes()).collect();
        let code = params.iter().find(|(k, _)| k == "code").map(|(_, v)| v.to_string());
        let state = params.iter().find(|(k, _)| k == "state").map(|(_, v)| v.to_string());

        if let Some(code) = code {
            let _ = self
                .vault
                .store(&self.app, "oauth", &code, None, None)
                .map_err(|e| warn!("failed to write token: {e}"));

            let body = format!(
                "<html><body><h3>{app} authorization captured</h3><p>state={state:?}</p></body></html>",
                app = self.app,
                state = state
            );
            return Ok(Response::builder()
                .status(StatusCode::OK)
                .body(Body::from(body))
                .unwrap());
        }

        Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("missing code"))
            .unwrap())
    }
}
