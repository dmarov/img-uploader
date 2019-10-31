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

        let mut response = reqwest::get(&self.url)
            .unwrap();

        if response.status().is_success() {

            let mut buf = Vec::new();
            response.copy_to(&mut buf).unwrap();

            let bytes = buf.as_slice();
            let img = image::load_from_memory(bytes)
                .unwrap();

            let new_img = img.thumbnail(100, 100);
            let format = image::guess_format(bytes)
                .unwrap();

            let mut new_buf = Vec::new();
            new_img.write_to(&mut new_buf, format)
                .unwrap();
            let new_bytes = new_buf.as_slice(); 

            std::fs::create_dir_all(&self.path)
                .unwrap();

            let extension = match format {
                image::ImageFormat::PNG => Ok("png"),
                image::ImageFormat::JPEG => Ok("jpg"),
                image::ImageFormat::GIF => Ok("gif"),
                image::ImageFormat::WEBP => Ok("webp"),
                image::ImageFormat::BMP => Ok("bmp"),
                image::ImageFormat::ICO => Ok("ico"),
                _ => Err("unsupported image format"),
            };

            let md5 = md5::compute(new_bytes);
            let md5_str = String::from(format!("{:x}", md5));
            let mut file_name = String::new();
            file_name.push_str(&md5_str);
            file_name.push_str("_100x100.");
            file_name.push_str(extension.unwrap());

            let full_name = std::path::Path::new(&self.path)
                .join(file_name);

            let mut file = std::fs::File::create(full_name)
                .unwrap();
            file.write_all(new_bytes).unwrap();
        }

        Ok(Async::Ready(()))
    }
}


pub fn upload_with_thumbnail(url: String, upload_path: String) -> std::result::Result<(), Box<dyn std::error::Error>> {

    tokio::spawn(UploadWithThumbnailFuture{
        url: url,
        path: upload_path
    });

    Ok(())
}
