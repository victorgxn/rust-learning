use redis::{AsyncCommands, Client};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

#[derive(Deserialize)]
struct AlertPriceRequest {
    email: String,
    code: String,
}

#[tokio::main]
async fn main() {
    let redis_client = Client::open("redis://127.0.0.1/").expect("Invalid Redis URL");
    let redis_client = Arc::new(Mutex::new(redis_client));

    // Define the alert-price route
    let alert_price_route = warp::post()
        .and(warp::path("alert-price"))
        .and(warp::body::json())
        .and(with_redis(redis_client.clone()))
        .and_then(handle_alert_price);

    // Start the warp server
    warp::serve(alert_price_route)
        .run(([127, 0, 0, 1], 3000))
        .await;
}

fn with_redis(
    redis_client: Arc<Mutex<Client>>,
) -> impl Filter<Extract = (Arc<Mutex<Client>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || redis_client.clone())
}

async fn handle_alert_price(
    payload: AlertPriceRequest,
    redis_client: Arc<Mutex<Client>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut con = redis_client
        .lock()
        .await
        .get_tokio_connection()
        .await
        .expect("Failed to connect to Redis");
    let _: () = con
        .set(&payload.email, &payload.code)
        .await
        .expect("Failed to set key in Redis");

    Ok(format!(
        "CORRECT ALERT CREATED for {} with id {}",
        payload.email, payload.code
    ))
}
