# The architecture to build for
ARCH ?= x86_64
# The type of build (debug or release)
BUILD_TYPE ?= debug
# The build target triple of the kernel
KERNEL_BUILD_TARGET := $(ARCH)-unknown-none

# The modules to include
MODULES := kernel

# The directory where the target file structure will be built
TARGET_DIR := target_root

# The name of the iso file that will be generated in the end
ISO := $(BUILD_DIR)/image.iso

# The timeout for integration tests in seconds
INTEGRATION_TEST_TIMEOUT := 10

# The rust compiler to use
RUST_COMPILER := xargo
# The flags used for the rust compiler
RUST_COMPILER_FLAGS :=

# The path to the OVMF image
OVMF ?= /usr/share/ovmf/OVMF.fd
# The flags to use for qemu
QEMU_FLAGS := --no-reboot \
			  -smp cores=4 \
			  -s \
			  -serial stdio \
			  -net none \
			  -display none \
			  -bios $(OVMF)
