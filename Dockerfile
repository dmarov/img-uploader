FROM archlinux AS img_uploader_server
LABEL description="container to run image uploader"
WORKDIR /root/app
RUN pacman --noconfirm -Sy
RUN pacman --noconfirm -S openssl
RUN pacman --noconfirm -S grep
RUN pacman --noconfirm -S curl
RUN pacman --noconfirm -S gcc
RUN pacman --noconfirm -S pkgconf
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
COPY . .
RUN cargo build --release
EXPOSE 8080
CMD ["target/release/img-uploader", "--listen", "0.0.0.0:8080", "--fs-upload-dir", "/tmp/images"]
