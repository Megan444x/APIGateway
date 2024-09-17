use std::env;
use warp::{http::StatusCode, Filter};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok(); // Load .env file if present

    // Setup routes
    let health_route = warp::path!("health")
        .map(|| warp::reply::with_status("OK", StatusCode::OK));

    let backend_service_route = warp::path!("service" / String)
        .and(warp::get())
        .map(|service_name: String| {
            let response = match service_name.as_str() {
                "service1" => call_service1(),
                "service2" => call_service2(),
                _ => "Service not found".to_string(),
            };
            warp::reply::with_status(response, StatusCode::OK)
        });

    let routes = health_route.or(backend_service_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn call_service1() -> String {
    // Logic to integrate with backend service 1
    // For example, fetching an environment variable
    let service1_url = env::var("SERVICE1_URL").unwrap_or_else(|_| "http://default-service1-url".into());
    
    format!("Calling Service 1 at URL: {}", service1_url)
}

fn call_service2() -> String {
    // Logic to integrate with backend service 2
    // For example, fetching another environment variable
    let service2_url = env::var("SERVICE2_URL").unwrap_or_else(|_| "http://default-service2-url".into());
    
    format!("Calling Service 2 at URL: {}", service2_url)
}