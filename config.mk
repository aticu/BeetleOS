ARCH ?= x86_64
BUILD_TYPE ?= debug
KERNEL_BUILD_TARGET := $(ARCH)-unknown-none

MODULES := kernel

TARGET_DIR := target_root

ISO := $(BUILD_DIR)/image.iso

RUST_COMPILER_FLAGS :=
RUST_COMPILER := xargo

OVMF ?= /usr/share/ovmf/x64/OVMF_CODE.fd
QEMU_FLAGS := --no-reboot \
			  -smp cores=4 \
			  -s \
			  -serial stdio \
			  -bios $(OVMF)