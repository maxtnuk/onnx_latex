FROM rust:1.51-slim-buster
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/server
COPY ./core ./core
COPY ./data ./data
COPY ./hir ./hir
COPY ./linalg ./linalg 
COPY ./nnef ./nnef
COPY ./onnx ./onnx
COPY ./onnx-opl ./onnx-opl
COPY ./server ./server
RUN ls -al ./onnx

RUN cd server && cargo install --path .
EXPOSE 8080
CMD ["server"]