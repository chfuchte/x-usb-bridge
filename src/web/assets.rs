use axum::{
    body::Body,
    http::{Response, StatusCode, Uri, header},
};

#[derive(rust_embed::RustEmbed)]
#[folder = "frontend/dist/"]
struct Assets;

pub(crate) async fn static_handler(uri: Uri) -> Response<Body> {
    let path = uri.path().trim_start_matches('/');

    if let Some(content) = Assets::get(path) {
        return response_for_asset(path, content.data.into_owned());
    }

    if let Some(index) = Assets::get("index.html") {
        return response_for_asset("index.html", index.data.into_owned());
    }

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Not Found"))
        .expect("Failed to build response")
}

fn response_for_asset(path: &str, data: Vec<u8>) -> Response<Body> {
    let content_type = mime_guess::from_path(path)
        .first_or_octet_stream()
        .to_string();

    Response::builder()
        .header(header::CONTENT_TYPE, content_type)
        .body(Body::from(data))
        .expect("Failed to build response")
}
