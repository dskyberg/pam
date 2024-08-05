use actix_files::{Files, NamedFile};
use actix_web::{get, web};

#[get("/")]
async fn index() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open_async("./static/index.html").await?)
}

pub fn register(config: &mut web::ServiceConfig) {
    config
        .service(Files::new("/", "./static").index_file("index.html"))
        .service(index);
}
