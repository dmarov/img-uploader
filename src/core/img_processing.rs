use std::io::Write;
extern crate tokio;
use futures::{Async, future::Future};
use std::fmt;
use image::GenericImageView;

pub struct UploadWithThumbnailFuture {
    pub url: String,
    pub path: String,
}

#[derive(Debug)]
pub struct ProcessingError {
    msg: String,
}

impl fmt::Display for ProcessingError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl UploadWithThumbnailFuture {

    fn process_buffer(&self, bytes: &[u8]) -> std::result::Result<(), ProcessingError> {

        let img = match image::load_from_memory(bytes) {
            Ok(img) => img,
            Err(error) => return Err(ProcessingError{msg: error.to_string()}),
        };

        let thumbnail_img = self.generate_thumbnail(&img);

        let format = match image::guess_format(bytes) {
            Ok(format) => format,
            Err(error) => return Err(ProcessingError{msg: error.to_string()}),
        };

        let mut thumbnail_buf = Vec::new();
        match thumbnail_img.write_to(&mut thumbnail_buf, format) {
            Ok(res) => res,
            Err(error) => return Err(ProcessingError{msg: error.to_string()}),
        };

        let thumbnail_bytes = thumbnail_buf.as_slice();

        let ext: String = match format {
            image::ImageFormat::PNG => String::from("png"),
            image::ImageFormat::JPEG => String::from("jpg"),
            image::ImageFormat::GIF => String::from("gif"),
            image::ImageFormat::WEBP => String::from("webp"),
            image::ImageFormat::BMP => String::from("bmp"),
            image::ImageFormat::ICO => String::from("ico"),
            _ => return Err(ProcessingError{
                msg: "unsupported image format".to_string()
            }),
        };

        let md5 = md5::compute(bytes);

        match std::fs::create_dir_all(&self.path) {
            Ok(res) => res,
            Err(error) => return Err(ProcessingError{msg: error.to_string()}),
        };

        let file_name = format!("{:x}.{}", md5, ext);
        match self.write_file(file_name, bytes) {
            Ok(res) => res,
            Err(error) => return Err(error),
        };

        let thumbnail_file_name = format!("{:x}_100x100.{}", md5, ext);
        match self.write_file(thumbnail_file_name, thumbnail_bytes) {
            Ok(res) => res,
            Err(error) => return Err(error),
        };

        Ok(())
    }

    fn write_file(&self, name: String, bytes: &[u8]) -> std::result::Result<(), ProcessingError>{

        let full_name = std::path::Path::new(&self.path)
            .join(name);

        let mut file = match std::fs::File::create(full_name) {
            Ok(file) => file,
            Err(error) => return Err(ProcessingError{msg: error.to_string()}),
        };

        match file.write_all(bytes) {
            Ok(res) => res,
            Err(error) => return Err(ProcessingError{msg: error.to_string()}),
        };

        Ok(())
    }

    fn generate_thumbnail(&self, img: &image::DynamicImage) -> image::DynamicImage {

        let (width, height) = img.dimensions();
        let min = std::cmp::min(width, height);
        let x = (width - min) / 2;
        let y = (height - min) / 2;
        let square_img = img.clone().crop(x, y, min, min);
        square_img.thumbnail(100, 100)
    }
}

impl Future for UploadWithThumbnailFuture {

    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {

        let mut response = match reqwest::get(&self.url) {

            Ok(response) => response,
            Err(error) => {
                eprintln!("{:?}", error);
                return Err(())
            },
        };

        if !response.status().is_success() {

            eprintln!("couldn't get successful response from given url");
            return Err(())
        }

        let mut buf = Vec::new();
        match response.copy_to(&mut buf) {
            Err(error) => {
                eprintln!("{:?}", error);
                return Err(())
            },
            Ok(res) => res,
        };

        match self.process_buffer(buf.as_slice()) {
            Ok(_) => Ok(Async::Ready(())),
            Err(error) => {
                println!("{:?}", error);
                Err(())
            }
        }
    }
}
