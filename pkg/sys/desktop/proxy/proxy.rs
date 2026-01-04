use crate::proxy_configs::{load_rules, Proxyconfigs, ProxyRule};
use anyhow::Result;
use hyper::body::to_bytes;
use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, Uri};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

#[derive(Clone)]
pub(crate) struct SharedState {
    client: Client<HttpConnector>,
    configs: Proxyconfigs,
    rules: Arc<RwLock<Vec<ProxyRule>>>,
}

/// Start a lightweight HTTP proxy for desktop apps.
pub async fn start_proxy(configs: Proxyconfigs) -> Result<()> {
    let addr: SocketAddr = format!("{}:{}", configs.host, configs.port).parse()?;
    let rules_dir = configs.rules_directory.clone();
    let rules = if let Some(dir) = rules_dir {
        load_rules(&dir)?
    } else {
        Vec::new()
    };

    let shared = SharedState {
        client: Client::new(),
        configs: configs.clone(),
        rules: Arc::new(RwLock::new(rules)),
    };

    let make_svc = make_service_fn(move |_| {
        let state = shared.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let state = state.clone();
                async move { handle_request(req, state).await }
            }))
        }
    });

    info!(?addr, "NDCL proxy listening");
    Server::bind(&addr).serve(make_svc).await?;
    Ok(())
}

async fn handle_request(req: Request<Body>, state: SharedState) -> Result<Response<Body>, Infallible> {
    // Simple allow/block check based on host
    let host = req
        .headers()
        .get("host")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
        .to_string();

    if is_blocked(&host, &state).await {
        let body = Body::from("blocked by NDCL proxy");
        let resp = Response::builder()
            .status(451)
            .body(body)
            .unwrap_or_else(|_| Response::new(Body::empty()));
        return Ok(resp);
    }

    // If no upstream configsured, bail out early
    let upstream = match state
        .configs
        .default_upstream
        .as_ref()
        .and_then(|u| u.parse::<Uri>().ok())
    {
        Some(uri) => uri,
        None => {
            let body = Body::from("no upstream configsured for NDCL proxy");
            let resp = Response::builder()
                .status(502)
                .body(body)
                .unwrap_or_else(|_| Response::new(Body::empty()));
            return Ok(resp);
        }
    };

    // Rewrite request to upstream
    let mut new_req = Request::builder()
        .method(req.method())
        .uri(rewrite_uri(req.uri(), &upstream))
        .body(Body::empty())
        .unwrap();

    // Copy headers and body
    *new_req.headers_mut() = req.headers().clone();
    match to_bytes(req.into_body()).await {
        Ok(bytes) => {
            *new_req.body_mut() = Body::from(bytes);
        }
        Err(e) => {
            error!("proxy body read failed: {}", e);
            let resp = Response::builder()
                .status(500)
                .body(Body::from("proxy read error"))
                .unwrap();
            return Ok(resp);
        }
    }

    match state.client.request(new_req).await {
        Ok(resp) => Ok(resp),
        Err(err) => {
            error!("proxy upstream error: {}", err);
            let resp = Response::builder()
                .status(502)
                .body(Body::from("upstream error"))
                .unwrap();
            Ok(resp)
        }
    }
}

async fn is_blocked(host: &str, state: &SharedState) -> bool {
    let rules = state.rules.read().await;
    for rule in rules.iter() {
        if rule.blocklist.iter().any(|p| host.contains(p)) {
            return true;
        }
    }
    false
}

fn rewrite_uri(original: &hyper::Uri, upstream: &Uri) -> Uri {
    let mut parts = original.clone().into_parts();
    parts.scheme = upstream.scheme().cloned();
    parts.authority = upstream.authority().cloned();
    Uri::from_parts(parts).unwrap_or_else(|_| upstream.clone())
}

/// Helper to reload rule files without restarting the proxy
pub(crate) async fn reload_rules(state: &SharedState, rules_dir: PathBuf) -> Result<()> {
    let loaded = load_rules(&rules_dir)?;
    let mut guard = state.rules.write().await;
    *guard = loaded;
    Ok(())
}
