use actix_web::{dev::ServiceRequest, Error, HttpServer, App, web, HttpResponse, middleware, http::StatusCode};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::env;
use actix_ratelimit::{RateLimiter, MemoryStore, MemoryStoreActor};
use std::time::Duration;

async fn index() -> &'static str {
    "Hello, welcome to our server"
}

async fn validate_token(auth: BearerAuth) -> Result<(), Error> {
    match env::var("SECRET_TOKEN") {
        Ok(token) => {
            if auth.token() == token {
                Ok(())
            } else {
                Err(Error::from(HttpResponse::Unauthorized().finish()))
            }
        },
        Err(_) => Err(Error::from(HttpResponse::InternalServerError().reason("Internal server error: SECRET_TOKEN missing").finish())),
    }
}

async fn validator(req: ServiceRequest) -> Result<ServiceRequest, Error> {
    let auth_header = req.headers().get("Authorization");
    match auth_header {
        Some(auth) => {
            match auth.to_str() {
                Ok(auth_str) => {
                    if auth_str.starts_with("Bearer ") {
                        let auth = BearerAuth::new(auth_str.trim_start_matches("Bearer "));
                        validate_token(auth).await?;
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
    let store = MemoryStore::new();
    
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                RateLimiter::new(
                    MemoryStoreActor::from(store.clone()).start_rate_limiter()
                )
                .with_interval(Duration::from_secs(60))
                .with_max_requests(1)
            )
            .service(
                web::resource("/")
                    .wrap(middleware::Condition::new(true, middleware::fn_service(validator)))
                    .route(web::get().to(index))
            )            
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}