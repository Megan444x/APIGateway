use std::env;
use warp::{http::StatusCode, Filter};
// Import log macros
use log::{info, warn};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok(); // Load .env file if present
    // Initialize the logger
    env_logger::init();

    // Log that the server is starting up
    info!("Starting up the server...");

    // Setup routes
    let health_route = warp::path!("health")
        .map(|| {
            info!("Health check route called");
            warp::reply::with_status("OK", StatusCode::OK)
        });

    let backend_service_route = warp::path!("service" / String)
        .and(warp::get())
        .map(|service_name: String| {
            let response = match service_name.as_str() {
                "service1" => {
                    info!("Service 1 called");
                    call_service1()
                },
                "service2" => {
                    info!("Service 2 called");
                    call_service2()
                },
                _ => {
                    warn!("Service not found: {}", service_name);
                    "Service not found".to_string()
                },
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
    
    // Log calling service 1
    info!("Calling Service 1 at URL: {}", service1_url);
    format!("Calling Service 1 at URL: {}", service1_url)
}

fn call_service2() -> String {
    // Logic to integrate with backend service 2
    // For example, fetching another environment variable
    let service2_url = env::var("SERVICE2_URL").unwrap_or_else(|_| "http://default-service2-url".into());
    
    // Log calling service 2
    info!("Calling Service 2 at URL: {}", service2_url);
    format!("Calling Service 2 at URL: {}", service2_url)
}