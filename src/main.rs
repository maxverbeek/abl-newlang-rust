use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
mod types;

// The entire main function is directly from the getting started example of Axum, except for the
// part where the `get_address` method is registered on a route.
#[tokio::main]
async fn main() {
    // build our application with a two routes
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
    // the #[from] macro implements a conversion from a reqwest error to this enum, which allows
    // reqwest errors to be converted automatically when the ? operator is used.
    #[error(transparent)]
    UsersRequestFailed(#[from] reqwest::Error),

    #[error("user is not found")]
    UserNotFound,
}

// implement the IntoResponse trait for our error type, which allows it to be returned from http
// handlers directly. Note that Result<T, E> where T and E implement IntoResponse also implements
// IntoResponse. This implementation allows http handlers to return errors directly.
impl IntoResponse for GetAddressError {
    // This function defines how our error types are converted into HTTP responses. Currently,
    // NotFound is mapped to something with a message (and corresponding status code) while
    // everything else is mapped to a 500 with a printout of the error(s).
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Self::UsersRequestFailed(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("internal server error: {e}"),
            ),
            Self::UserNotFound => (StatusCode::NOT_FOUND, "not found".to_owned()),
        };

        (status, Json(json!({ "message": message }))).into_response()
    }
}

// A response object, intended to be used only as response type for a handler.
#[derive(Serialize, Debug)]
struct AppUser {
    id: usize,
    address: String,
}

// A conversion method
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

async fn get_address(Path(addr_id): Path<usize>) -> Result<Json<AppUser>, GetAddressError> {
    let response = reqwest::get("https://jsonplaceholder.typicode.com/users")
        .await?
        // By default, HTTP errors are considered as a succesful request. this method converts
        // non-200 responses to errors as well.
        .error_for_status()?
        // Assume the response is a JSON array and parse it to a vec of our struct
        .json::<Vec<types::ForeingUser>>()
        .await?;

    // At this point, the type of the `response` variable is Vec<types::ForeingUser> which is
    // searchable.
    let user = response
        .into_iter()
        .find(|u| u.id == addr_id)
        .ok_or(GetAddressError::UserNotFound)?;

    // .into() knows what conversion to make because it's set in the return type of this function
    Ok(axum::Json(user.into()))
}
