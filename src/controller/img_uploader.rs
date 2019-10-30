use actix_web::{web, HttpResponse, Error, client::Client};
use futures::future::{ok, Future, lazy};
use serde::Deserialize;
use actix_rt::System;

#[derive(Deserialize)]
pub struct RequestModel {
    images: Vec<String>,
    hres: u8,
    vres: u8,
}

pub fn index(request_data: web::Json<RequestModel>) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {

    for url in request_data.images.iter() {

        // System::new("test").block_on(lazy(|| {
        tokio::spawn(lazy(move || {
            Client::new()
                .get(url)
                .send()
                .map_err(|_| ())
                .and_then(|response| {
                    println!("Response: {:?}", response);
                    Ok(())
                });
        }));
        // })).unwrap();
    }

    Box::new(ok::<_, Error>(
        HttpResponse::Ok()
            .json("ok")
    ))
}
