use std::env;
use std::collections::HashMap;
use std::sync::Mutex;
use warp::{http::StatusCode, Filter, Reply};
use log::{info, warn};
use futures::future::BoxFuture;
use warp::http::Response;
use once_cell::sync::OnceCell;
use tokio::sync::Mutex as AsyncMutex;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Starting up the server...");

    let health_route = build_health_route();
    let backend_service_route = build_backend_service_route();

    // Initialize our global state for caching
    let cache = Cache::new();
    let cache_filter = warp::any().map(move || cache.clone());

    let routes = health_route.or(backend_service_route.with(cache_filter));

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
        .and(warp::any().map(move || Cache::new())) // Inject the cache into the route
        .and_then(handle_backend_service_call)
}

async fn handle_backend_service_call(service_name: String, cache: Cache) -> Result<impl Reply, warp::Rejection> {
    let cached_response = cache.get(&service_name).await;
    match cached_response {
        Some(response) => {
            Ok(warp::reply::with_status(response, StatusCode::OK))
        },
        None => {
            let response: BoxFuture<'static, Response<String>> = match service_name.as_str() {
                "service1" => {
                    info!("Service 1 called");
                    Box::pin(async {
                        let result = call_service1().await;
                        cache.store("service1".to_string(), result.clone()).await;
                        result.into_response()
                    })
                },
                "service2" => {
                    info!("Service 2 called");
                    Box::pin(async {
                        let result = call_service2().await;
                        cache.store("service2".to_string(), result.clone()).await;
                        result.into_response()
                    })
                },
                _ => {
                    warn!("Service not found: {}", service_name);
                    Box::pin(async { warp::reply::with_status("Service not found".to_string(), StatusCode::NOT_FOUND).into_response() })
                },
            };

            Ok(response.await)
        }
    }
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

#[derive(Clone)]
struct Cache {
    inner: Arc<AsyncMutex<HashMap<String, String>>>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            inner: Arc::new(AsyncMutex::new(HashMap::new())),
        }
    }

    async fn get(&self, key: &str) -> Option<String> {
        let cache = self.inner.lock().await;
        cache.get(key).cloned()
    }

    async fn store(&self, key: String, value: String) {
        let mut cache = self.inner.lock().await;
        cache.insert(key, value);
    }
}