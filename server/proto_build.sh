#!/bin/bash
protoc --proto_path=../onnx/protos/onnx --python_out=. ../onnx/protos/onnx/onnx.proto3