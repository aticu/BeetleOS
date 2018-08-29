BUILD_DIR := target

include config.mk

TARGET_FILES :=

ifeq ($(BUILD_TYPE),release)
	RUST_COMPILER_FLAGS += --release
endif

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