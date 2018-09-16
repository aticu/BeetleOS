BUILD_DIR := target

include config.mk

# The files that need to build to construct the system
TARGET_FILES :=

RUST_RELEASE :=
ifeq ($(BUILD_TYPE),release)
	RUST_RELEASE += --release
endif
RUST_COMPILER_FLAGS += $(RUST_RELEASE)

export RUST_TARGET_PATH=$(abspath targets)

.PHONY: all
all: target_files

include $(patsubst %,%/module.mk,$(MODULES))
include architectures/$(ARCH).mk

.PHONY: target_files
target_files: $(TARGET_FILES)

.PHONY: clean
clean:
	rm $(BUILD_DIR) $(TARGET_DIR) -rf

.PHONY: run
run: $(ISO)
	qemu-system-$(ARCH) $(QEMU_FLAGS) -cdrom $<

.PHONY: debug
debug: $(ISO)
	qemu-system-$(ARCH) $(QEMU_FLAGS) -cdrom $< -S -d guest_errors

.PHONY: doc
doc:
	cargo doc --all-features --lib

.PHONY: test
test: $(BUILD_DIR)/release/test_runner
	$< $(ARCH) --target-triple=$(KERNEL_BUILD_TARGET) $(RUST_RELEASE) \
		--rust-target-path=$(RUST_TARGET_PATH) --rust-compiler=$(RUST_COMPILER) \
		--run-on=qemu --bios=$(OVMF) --timeout=$(INTEGRATION_TEST_TIMEOUT)

$(BUILD_DIR)/release/test_runner: $(shell find test_runner/src -name "*.rs") test_runner/Cargo.toml
	cd test_runner && cargo build --release
