FROM rust:1.51-slim-buster as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/server
COPY ./latex_gen ./latex_gen
COPY ./server ./server
COPY ./hir ./hir
COPY ./onnx ./onnx

RUN cd server && cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/server /usr/local/bin/server

EXPOSE 8080
CMD ["server"]