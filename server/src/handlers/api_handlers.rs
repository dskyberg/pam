use actix_web::{route, web, HttpResponse};

/// Service initialization endpoint
/// This is called once at service start to prepopulate the database
#[route("/auth", method = "GET")]
pub async fn auth() -> HttpResponse {
    HttpResponse::Ok().body(r#"{"auth":true}"#)
}

pub fn register(config: &mut web::ServiceConfig) {
    config.service(web::scope("api").service(auth));
}
