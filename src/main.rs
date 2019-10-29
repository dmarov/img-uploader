extern crate clap;
use clap::{Arg, App as ClapApp};
use actix_web::{web, App as ActixApp, HttpServer};
use listenfd::ListenFd;
mod controller;

fn main() {

    let args = [

        Arg::with_name("listen")
            .short("l")
            .long("listen")
            .value_name("LISTEN ADDRESS")
            .takes_value(true)
            .required(true),

        Arg::with_name("fs-storage-path")
            .long("fs-storage-path")
            .value_name("FILE SYSTEM STORAGE PATH")
            .help("sets directory to store previews")
            .takes_value(true)
            .required(true),

        Arg::with_name("hres")
            .long("hres")
            .value_name("HORIZONTAL PREVIEW RESOLUTION")
            .takes_value(true)
            .required(true),

        Arg::with_name("vres")
            .long("vres")
            .value_name("VERTICAL PREVIEW RESOLUTION")
            .takes_value(true)
            .required(true),
    ];

    let mut app = ClapApp::new("Image preview generation server")
        .version("1.0")
        .author("Dmitry Marov <d.marov94@gmail.com>")
        .about("Server to generate image preview");

    for arg in args.iter() {

        app = app.arg(arg);
    }

    let matches = app.get_matches();
    let addr = matches.value_of("listen").unwrap();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        ActixApp::new().route("/img-uploader", web::post().to_async(controller::img_uploader::index))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(addr).unwrap()
    };

    server.run().unwrap();
}
