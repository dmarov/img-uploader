use actix_web::{web, HttpResponse, Error};
use futures::future::{ok, Future};
use serde::Deserialize;
use crate::core::img_processing::UploadWithThumbnailFuture;

#[derive(Deserialize)]
pub struct RequestModel {
    urls: Vec<String>,
}

pub fn upload_images(
        request_data: web::Json<RequestModel>,
        data: web::Data<crate::AppState>
    )-> Box<dyn Future<Item = HttpResponse, Error = Error>> {

    for url in request_data.urls.iter() {

        tokio::spawn(UploadWithThumbnailFuture{
            url: url.to_string(),
            path: data.upload_dir.clone(),
        });
    }

    Box::new(ok::<_, Error>(
        HttpResponse::NoContent()
            .finish()
    ))
}
