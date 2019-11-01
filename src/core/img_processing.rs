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

    fn process_buffer(&mut self, buf: Vec<u8>) -> std::result::Result<(), ProcessingError> {

        let bytes = buf.as_slice();

        let img = match image::load_from_memory(bytes) {
            Ok(img) => img,
            Err(error) => return Err(ProcessingError{msg: error.to_string()}),
        };

        let (width, height) = img.dimensions();
        let min = std::cmp::min(width, height);
        let x = (width - min) / 2;
        let y = (height - min) / 2;
        let square_img = img.clone().crop(x, y, min, min);
        let new_img = square_img.thumbnail(100, 100);

        let format = match image::guess_format(bytes) {
            Ok(format) => format,
            Err(error) => return Err(ProcessingError{msg: error.to_string()}),
        };

        let mut new_buf = Vec::new();
        match new_img.write_to(&mut new_buf, format) {
            Ok(res) => res,
            Err(error) => return Err(ProcessingError{msg: error.to_string()}),
        };

        let new_bytes = new_buf.as_slice(); 

        match std::fs::create_dir_all(&self.path) {
            Ok(res) => res,
            Err(error) => return Err(ProcessingError{msg: error.to_string()}),
        };

        let extension: String = match format {
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
        let md5_str = String::from(format!("{:x}", md5));

        let mut file_name = String::new();
        let mut thumbnail_file_name = String::new();

        file_name.push_str(&md5_str);
        file_name.push_str(".");
        file_name.push_str(extension.as_str());

        thumbnail_file_name.push_str(&md5_str);
        thumbnail_file_name.push_str("_100x100");
        thumbnail_file_name.push_str(".");
        thumbnail_file_name.push_str(extension.as_str());

        let full_name = std::path::Path::new(&self.path)
            .join(file_name);

        let thumbnail_full_name = std::path::Path::new(&self.path)
            .join(thumbnail_file_name);

        let mut file = std::fs::File::create(full_name)
            .unwrap();
        file.write_all(bytes).unwrap();

        let mut thumbnail_file = std::fs::File::create(thumbnail_full_name)
            .unwrap();
        thumbnail_file.write_all(new_bytes).unwrap();

        Ok(())
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

        match self.process_buffer(buf) {
            Ok(_) => Ok(Async::Ready(())),
            Err(error) => {
                println!("{:?}", error);
                Err(())
            }
        }
    }
}
