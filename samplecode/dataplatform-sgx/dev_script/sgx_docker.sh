#!/bin/bash

set -e

IMAGE_NAME=baiduxlab/sgx-rust
RUST_SGX=/home/developCode/RustProject/github.com/green-element-chain/rust-sgx-sdk
# RUST_SGX=/home/developCode/OpenSource/github.com/baidu/rust-sgx-sdk

docker run \
    -v $RUST_SGX:/root/sgx \
    -v $HOME/.cargo:/root/.cargo \
    -v $HOME/.rustup:/root/.rustup \
    -ti --device /dev/isgx \
    $IMAGE_NAME
