FROM ekidd/rust-musl-builder:nightly

COPY . .
RUN sudo chown -R rust:rust .
RUN cargo install
EXPOSE 8000
ENTRYPOINT ["./target/release/aliyun-image-bot"]
