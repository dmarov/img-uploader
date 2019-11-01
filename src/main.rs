mod controller;
mod core;

pub struct AppState {
    upload_dir: String,
}

fn main() {

    let app = clap::App::new("Image preview generation server")
        .version("1.0")
        .author("Dmitry Marov <d.marov94@gmail.com>")
        .about("Server to generate image preview")
        .arg(
            clap::Arg::with_name("fs-upload-dir")
                .long("fs-upload-dir")
                .value_name("FILE SYSTEM UPLOAD DIRECTORY")
                .help("sets directory to store previews")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("listen")
                .short("l")
                .long("listen")
                .value_name("LISTEN ADDRESS")
                .takes_value(true)
                .required(true),
        );

    let matches = app.get_matches();

    let addr = matches.value_of("listen")
        .unwrap();

    let upload_dir = matches.value_of("fs-upload-dir")
        .unwrap()
        .to_string();

    let data = actix_web::web::Data::new(AppState {
        upload_dir,
    });

    let server = actix_web::HttpServer::new(move || {

        actix_web::App::new()
            .register_data(data.clone())
            .route("/img-uploader", actix_web::web::post().to_async(controller::img_uploader::upload_images))
    });

    server.bind(addr)
        .unwrap()
        .run()
        .unwrap();
}
