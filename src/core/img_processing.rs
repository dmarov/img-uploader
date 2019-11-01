use std::io::Write;
extern crate tokio;
use futures::{Async, future::Future};

pub struct UploadWithThumbnailFuture {
    pub url: String,
    pub path: String,
}

impl Future for UploadWithThumbnailFuture {

    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {

        let mut response = match reqwest::get(&self.url) {

            Ok(response) => response,
            Err(error) => {
                eprintln!("{}", error);
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
                eprintln!("{}", error);
                return Err(())
            },
            Ok(res) => res,
        };

        let bytes = buf.as_slice();

        let img = match image::load_from_memory(bytes) {
            Ok(img) => img,
            Err(error) => {
                eprintln!("{}", error);
                return Err(())
            },
        };

        let new_img = img.thumbnail(100, 100);
        let format = match image::guess_format(bytes) {
            Ok(format) => format,
            Err(error) => {
                eprintln!("{}", error);
                return Err(())
            },
        };


        let mut new_buf = Vec::new();
        match new_img.write_to(&mut new_buf, format) {
            Ok(res) => res,
            Err(error) => {
                eprintln!("{}", error);
                return Err(())
            },
        };

        let new_bytes = new_buf.as_slice(); 

        match std::fs::create_dir_all(&self.path) {
            Ok(res) => res,
            Err(error) => {
                eprintln!("{}", error);
                return Err(())
            },
        };

        let extension: String = match format {
            image::ImageFormat::PNG => String::from("png"),
            image::ImageFormat::JPEG => String::from("jpg"),
            image::ImageFormat::GIF => String::from("gif"),
            image::ImageFormat::WEBP => String::from("webp"),
            image::ImageFormat::BMP => String::from("bmp"),
            image::ImageFormat::ICO => String::from("ico"),
            _ => {
                eprintln!("unsupported image format");
                return Err(())
            },
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

        Ok(Async::Ready(()))
    }
}
