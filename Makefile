TARGET_ARM64=aarch64-unknown-linux-gnu
BUILD_FLAG?=

.PHONY: setup
setup:
	cd ./docker; ./build-docker-image.sh

.PHONY: build
build:
	BUILD_FLAG=--release $(MAKE) build-debug

.PHONY: build-debug
build-debug:
	SYSROOT=/usr/aarch64-linux-gnu cross build --target=${TARGET_ARM64} ${BUILD_FLAG}