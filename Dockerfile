FROM alpine:3.22.1

# 设置工作目录
WORKDIR /app

# 程序相关文件
COPY target/x86_64-unknown-linux-musl/release/s3-tool /app
