use crate::api_error::ApiError;
use crate::image::Image;
use actix_web::{get, web, HttpResponse};
// use serde_json::json;
// use uuid::Uuid;


#[get("/images")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let images = Image::find_all()?;
    Ok(HttpResponse::Ok().json(images))
}

// #[post("/images")]
// async fn create(image: ) -> Result<HttpResponse, ApiError> {
//     let image = Image::create(image)?;
//     Ok(HttpResponse::Ok().json(image))
// }

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    // cfg.service(create);
}
