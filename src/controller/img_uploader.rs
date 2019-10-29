use actix_web::{HttpResponse, Error};
use futures::future::{ok, Future};

pub fn index() -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    Box::new(ok::<_, Error>(
        HttpResponse::Ok().content_type("text/html").body("Hello!"),
    ))
}
