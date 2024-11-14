use actix_web::{dev::ServiceRequest, Error, HttpServer, App, web, HttpResponse, middleware};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::env;
use actix_ratelimit::{RateLimiter, MemoryStore, MemoryStoreActor};
use std::time::Duration;

async fn index() -> &'static str {
    "Hello, welcome to our server"
}

async fn validate_token(auth: BearerAuth) -> Result<(), Error> {
    if auth.token() == env::var("SECRET_TOKEN").expect("SECRET_TOKEN must be set") {
        Ok(())
    } else {
        Err(Error::from(HttpResponse::Unauthorized().finish()))
    }
}

async fn validator(req: ServiceRequest) -> Result<ServiceRequest, Error> {
    if let Some(auth) = req.headers().get("Authorization") {
        let auth_str = auth.to_str().unwrap_or("");
        if auth_str.starts_with("Bearer") {
            let auth = BearerAuth::new(auth_str);
            validate_token(auth).await?;
            Ok(req)
        } else {
            Err(Error::from(HttpResponse::Unauthorized().finish()))
        }
    } else {
        Err(Error::from(HttpResponse::Unauthorized().finish()))
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