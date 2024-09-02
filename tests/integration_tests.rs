use warp::Filter;
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};

#[tokio::main]
async fn main() {
    let request_count = Arc::new(Mutex::new((0, Instant::now())));

    let rate_limit = {
        let request_count = Arc::clone(&request_count);
        warp::any().map(move || {
            let mut count = request_count.lock().unwrap();
            let elapsed = count.1.elapsed();
            if elapsed > Duration::from_secs(1) {
                *count = (1, Instant::now());
            } else {
                count.0 += 1;
                if count.0 > 5 {
                    return warp::reply::with_status("Too Many Requests", warp::http::StatusCode::TOO_MANY_REQUESTS);
                }
            }
            warp::reply::with_status("Request handled", warp::http::StatusCode::OK)
        })
    };

    let route = warp::path("example")
        .and(warp::get())
        .and(rate_limit);

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
