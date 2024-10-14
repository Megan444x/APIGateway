use std::env;
use warp::{http::StatusCode, Filter, Reply};
use log::{info, warn};
use futures::future::BoxFuture;
use warp::http::Response;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Starting up the server...");

    let health_route = build_health_route();
    let backend_service_route = build_backend_service_route();

    let routes = health_route.or(backend_service_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn build_health_route() -> impl Filter<Extract=impl Reply, Error=warp::Rejection> + Clone {
    warp::path!("health")
        .map(|| {
            info!("Health check route called");
            warp::reply::with_status("OK", StatusCode::OK)
        })
}

fn build_backend_service_route() -> impl Filter<Extract=impl Reply, Error=warp::Rejection> + Clone {
    warp::path!("service" / String)
        .and(warp::get())
        .and_then(handle_backend_service_call)
}

async fn handle_backend_service_call(service_name: String) -> Result<impl Reply, warp::Rejection> {
    let response: BoxFuture<'static, Response<String>> = match service_name.as_str() {
        "service1" => {
            info!("Service 1 called");
            Box::pin(async { call_service1().await.into_response() })
        },
        "service2" => {
            info!("Service 2 called");
            Box::pin(async { call_service2().await.into_response() })
        },
        _ => {
            warn!("Service not found: {}", service_name);
            Box::pin(async { warp::reply::with_status("Service not found".to_string(), StatusCode::NOT_FOUND).into_response() })
        },
    };

    Ok(response.await)
}

async fn call_service1() -> String {
    let service1_url = env::var("SERVICE1_URL").unwrap_or_else(|_| "http://default-service1-url".to_string());
    
    info!("Calling Service 1 at URL: {}", service1_url);
    format!("Result from Service 1 at URL: {}", service1_url)
}

async fn call_service2() -> String {
    let service2_url = env::var("SERVICE2_URL").unwrap_or_else(|_| "http://default-service2-url".to_string());
    
    info!("Calling Service 2 at URL: {}", service2_url);
    format!("Result from Service 2 at URL: {}", service2_url)
}