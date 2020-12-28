TARGET_ARM64=aarch64-unknown-linux-gnu


.PHONY: setup
setup:
	cd ./docker; ./build-docker-image.sh

.PHONY: build
build:
	SYSROOT=/usr/aarch64-linux-gnu cross build --target=${TARGET_ARM64} -v