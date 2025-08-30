use axum::{body::Body, extract::Request, response::Response};

pub fn is_built_version() -> bool {
    std::env::var("CARGO_MANIFEST_DIR").is_err()
}

pub async fn proxy_to_frontend(req: Request<Body>) -> Response<Body> {
    let uri = req.uri();
    let frontend_url = format!(
        "http://localhost:5173{}",
        uri.path_and_query().map(|pq| pq.as_str()).unwrap_or("/")
    );

    let client = reqwest::Client::new();
    match client.get(&frontend_url).send().await {
        Ok(resp) => {
            let status = resp.status();
            let headers = resp.headers().clone();
            let body = resp.bytes().await.unwrap_or_default();

            let mut response = Response::builder().status(status);
            for (key, value) in headers.iter() {
                response = response.header(key, value);
            }
            response.body(Body::from(body)).unwrap()
        }
        Err(_) => Response::builder()
            .status(502)
            .body(Body::from("Frontend dev server not available"))
            .unwrap(),
    }
}
