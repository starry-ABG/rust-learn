#!/bin/bash
set -e

# 配置：镜像名称、平台、和 Rust target
IMAGE_X86="rust-build-x86:latest"
IMAGE_ARM="rust-build-arm:latest"
PLATFORM_X86="linux/amd64"
PLATFORM_ARM="linux/arm64/v8"
TARGET_X86="x86_64-unknown-linux-musl"
TARGET_ARM="aarch64-unknown-linux-musl"

# 1. 询问要构建的架构（默认 x86）
echo "Which architecture do you want to build? [x86/arm] (default: x86)"
read -r ARCH
ARCH="${ARCH:-x86}"

# 2. 询问构建命令（默认 cargo build --release）
echo "Input the cargo build command (without --target), e.g. 'cargo build --release'"
echo "(default: 'cargo build --release')"
read -r BUILDCMD
BUILDCMD="${BUILDCMD:-cargo build --release}"

# 根据架构，设置镜像名、platform、Rust target
if [ "$ARCH" = "arm" ]; then
  DOCKER_IMAGE="$IMAGE_ARM"
  DOCKER_PLATFORM="$PLATFORM_ARM"
  RUST_TARGET="$TARGET_ARM"
else
  DOCKER_IMAGE="$IMAGE_X86"
  DOCKER_PLATFORM="$PLATFORM_X86"
  RUST_TARGET="$TARGET_X86"
fi

echo "Selected ARCH: $ARCH"
echo "Docker image will be: $DOCKER_IMAGE"
echo "Docker platform will be: $DOCKER_PLATFORM"
echo "Rust target will be: $RUST_TARGET"
echo "Cargo command: $BUILDCMD --target $RUST_TARGET"

# 3. 检查本地是否已有对应镜像，没有的话先构建
HAS_IMAGE=$(docker images --format '{{.Repository}}:{{.Tag}}' | grep -c "$DOCKER_IMAGE" || true)
if [ "$HAS_IMAGE" -eq 0 ]; then
  echo "Image '$DOCKER_IMAGE' not found locally. Building now..."

  # 将 Dockerfile 内容写入临时文件
  cat << 'EOF' > Dockerfile.buildenv
# 使用最新 Alpine 版官方 Rust 镜像
FROM rust:alpine

# 安装编译依赖:
#   - build-base: 提供 musl-gcc、g++, make 等基础构建工具
#   - clang: 在部分情况下 ring 需要
#   - openssl-dev: 用于编译依赖 openssl 的 crate
#   - pkgconfig: 定位 openssl 等库
#   - openssl-libs-static: 提供静态链接所需的 .a
RUN apk add --no-cache \
    build-base \
    clang \
    pkgconfig \
    openssl-dev \
    openssl-libs-static

# 如果你想要强制让 openssl 走静态链接
ENV OPENSSL_STATIC=1 \
    PKG_CONFIG_ALL_STATIC=1

RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add aarch64-unknown-linux-musl

WORKDIR /work
CMD [ "sh" ]
EOF

  # 用刚刚生成的 Dockerfile 进行构建
  docker build \
    --platform="$DOCKER_PLATFORM" \
    -t "$DOCKER_IMAGE" \
    -f Dockerfile.buildenv \
    .

  # (可选) 构建完成后删除临时 Dockerfile
  rm -f Dockerfile.buildenv
else
  echo "Image '$DOCKER_IMAGE' already exists, skipping build."
fi

# 4. 使用容器执行 Cargo 编译
docker run --rm \
  --platform="$DOCKER_PLATFORM" \
  -v "$(pwd)":/work \
  -w /work \
  "$DOCKER_IMAGE" \
  $BUILDCMD --target "$RUST_TARGET"