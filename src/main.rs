use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use thiserror::Error;
mod types;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/address/:id", get(get_address));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world() -> impl IntoResponse {
    "Hello World!"
}

#[derive(Error, Debug)]
enum GetAddressError {
    #[error(transparent)]
    UsersRequestFailed(#[from] reqwest::Error),
}

impl IntoResponse for GetAddressError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("internal server error: {self}"),
        )
            .into_response()
    }
}

#[derive(Serialize, Debug)]
struct AppUser {
    id: usize,
    address: String,
}

impl From<types::ForeingUser> for AppUser {
    fn from(f: types::ForeingUser) -> Self {
        AppUser {
            id: f.id,
            address: format!(
                "{} {} ({}, {})",
                f.address.city, f.address.zipcode, f.address.geo.lat, f.address.geo.lng
            ),
        }
    }
}

async fn get_address(Path(addr_id): Path<i32>) -> Result<Json<AppUser>, GetAddressError> {
    let response = reqwest::get(format!(
        "https://jsonplaceholder.typicode.com/users/{addr_id}"
    ))
    .await?
    .error_for_status()?
    .json::<types::ForeingUser>()
    .await?;

    Ok(axum::Json(Into::<AppUser>::into(response)))
}
