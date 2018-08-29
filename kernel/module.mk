KERNEL := target/$(ARCH)-unknown-none/$(BUILD_TYPE)/kernel-$(ARCH)

TARGET_FILES := $(KERNEL)

$(KERNEL): $(shell find kernel/src -name "*.rs") kernel/Cargo.toml
	$(RUST_COMPILER) build --target=$(KERNEL_BUILD_TARGET) $(RUST_COMPILER_FLAGS) --bin=kernel-$(ARCH)