use actix_web::{web, HttpResponse, Error};
use futures::future::{self, ok, Future, lazy};
use serde::Deserialize;
use crate::core::img_processing;

#[derive(Deserialize)]
pub struct RequestModel {
    urls: Vec<String>,
}

pub fn upload_images(request_data: web::Json<RequestModel>) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {

    for url in request_data.urls.iter() {

        img_processing::upload_with_thumbnail(url.to_string(), "/tmp/images".to_string())
            .unwrap();
    }

    Box::new(ok::<_, Error>(
        HttpResponse::NoContent()
            .finish()
    ))
}
