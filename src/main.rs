use axum::{Router, extract::Path, http::StatusCode, routing::get};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or("info,tower_http=debug".into()),
        )
        .init();

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/add/{lhs}/{rhs}", get(route_add))
        .route("/sub/{lhs}/{rhs}", get(route_sub))
        .route("/mul/{lhs}/{rhs}", get(route_mul))
        .route("/div/{lhs}/{rhs}", get(route_div))
        .route("/sum/{*path}", get(route_sum_seq))
        .route("/prod/{*path}", get(route_prod_seq))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
/// Computes the sum of two numbers
async fn route_add(Path((lhs, rhs)): Path<(f64, f64)>) -> String {
    (lhs + rhs).to_string()
}

/// Computes the difference of two numbers
async fn route_sub(Path((lhs, rhs)): Path<(f64, f64)>) -> String {
    (lhs - rhs).to_string()
}

/// Computes the product of two numbers
async fn route_mul(Path((lhs, rhs)): Path<(f64, f64)>) -> String {
    (lhs * rhs).to_string()
}

/// Computes the ratio of two numbers
async fn route_div(Path((lhs, rhs)): Path<(f64, f64)>) -> (StatusCode, String) {
    if rhs == 0.0 {
        (
            StatusCode::BAD_REQUEST,
            "Attempted to divide by zero".to_owned(),
        )
    } else {
        (StatusCode::OK, (lhs / rhs).to_string())
    }
}

/// Computes the sum of a given sequence of numbers
async fn route_sum_seq(Path(path): Path<String>) -> (StatusCode, String) {
    let segments = path
        .split("/")
        .map(|s| s.parse::<f64>().ok())
        .collect::<Vec<Option<f64>>>();

    match segments
        .into_iter()
        .try_fold(0.0, |acc, opt| opt.map(|n| acc + n))
    {
        Some(sum) => (StatusCode::OK, sum.to_string().to_owned()),
        None => (
            StatusCode::BAD_REQUEST,
            "Sequence contains numbers that were not parsed".to_owned(),
        ),
    }
}

/// Computes the product of a given sequence of numbers
async fn route_prod_seq(Path(path): Path<String>) -> (StatusCode, String) {
    let segments = path
        .split("/")
        .map(|s| s.parse::<f64>().ok())
        .collect::<Vec<Option<f64>>>();
    match segments
        .into_iter()
        .try_fold(1.0, |acc, opt| opt.map(|n| acc * n))
    {
        Some(prod) => (StatusCode::OK, prod.to_string().to_owned()),
        None => (
            StatusCode::BAD_REQUEST,
            "Sequence contains numbers that were not parsed".to_owned(),
        ),
    }
}
