use actix_web::{dev::ServiceRequest, Error, HttpServer, App, web, HttpResponse, middleware, http::StatusCode};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::env;
use actix_ratelimit::{RateLimiter, MemoryStore, MemoryStoreActor};
use std::time::Duration;

async fn welcome_message() -> &'static str {
    "Hello, welcome to our server"
}

async fn check_auth_token(bearer_auth: BearerAuth) -> Result<(), Error> {
    match env::var("SECRET_TOKEN") {
        Ok(expected_token) => {
            if bearer_auth.token() == expected_token {
                Ok(())
            } else {
                Err(Error::from(HttpResponse::Unauthorized().finish()))
            }
        },
        Err(_) => Err(Error::from(HttpResponse::InternalServerError().reason("Internal server error: SECRET_TOKEN missing").finish())),
    }
}

async fn auth_validator_middleware(req: ServiceRequest) -> Result<ServiceRequest, Error> {
    let authorization_header = req.headers().get("Authorization");
    match authorization_header {
        Some(auth_value) => {
            match auth_value.to_str() {
                Ok(auth_str) => {
                    if auth_str.starts_with("Bearer ") {
                        let auth_extracted = BearerAuth::new(auth_str.trim_start_matches("Bearer "));
                        check_auth_token(auth_extracted).await?;
                        Ok(req)
                    } else {
                        Err(Error::from(HttpResponse::Unauthorized().finish()))
                    }
                },
                Err(_) => Err(Error::from(HttpResponse::BadRequest().reason("Bad Authorization header").finish()))
            }
        },
        None => Err(Error::from(HttpResponse::Unauthorized().reason("Authorization header missing").finish())),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let rate_limiter_store = MemoryStore::new();
    
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                RateLimiter::new(
                    MemoryStoreActor::from(rate_limiter_store.clone()).start_rate_limiter()
                )
                .with_interval(Duration::from_secs(60))
                .with_max_requests(1)
            )
            .service(
                web::resource("/")
                    .wrap(middleware::Condition::new(true, middleware::fn_service(auth_validator_middleware)))
                    .route(web::get().to(welcome_message))
            )            
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}