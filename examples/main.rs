use std::time::Duration;
use tokio::time::sleep;
use tower::limit::RateLimitLayer;
use tower::ServiceBuilder;
use warp::Filter;

#[tokio::main]
async fn main() {
    // Rate limiting: 5 requests per second
    let rate_limit = ServiceBuilder::new()
        .layer(RateLimitLayer::new(5, Duration::from_secs(1)))
        .service(warp::service::service_fn(|req| async move {
            // Handle request
            Ok::<_, warp::Rejection>(warp::reply::with_status(
                "Request handled",
                warp::http::StatusCode::OK,
            ))
        }));

    // Define CORS settings
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allow_headers(vec!["Authorization", "Content-Type"]);

    // Example route that uses both CORS and rate limiting
    let route = warp::path("example")
        .and(warp::get())
        .and_then(move || {
            // Simulate some processing
            async move {
                sleep(Duration::from_millis(500)).await;
                Ok::<_, warp::Rejection>("CORS and Rate Limiting applied")
            }
        })
        .with(cors)
        .with(rate_limit);

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
