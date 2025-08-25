#!/bin/bash

set -e

# 切换到脚本所在目录
cd $(dirname $0)
echo "当前执行目录: $(pwd)"

# 编译程序
cargo build --release --manifest-path s3-tool/Cargo.toml --target x86_64-unknown-linux-musl

app_name=rcxid/s3-tool
date_time=$(date +%y%m%d_%H%M)
image_with_version=${app_name}:${date_time}

echo "====================================="
echo "应用名称: ${app_name}"
echo "构建时间: ${date_time}"
echo "镜像版本: ${image_with_version}"
echo "====================================="

# 编译镜像
docker build -t ${image_with_version} .

# 上传镜像
docker push ${image_with_version}
