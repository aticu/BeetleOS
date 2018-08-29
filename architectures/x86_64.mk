ESP_IMAGE := target/esp.img

$(ESP_IMAGE): $(TARGET_FILES)
	mkdir -p $(TARGET_DIR)/boot/EFI/BOOT
	cp $(KERNEL) $(TARGET_DIR)/boot/EFI/BOOT/BOOTX64.EFI
	dd if=/dev/zero of=$@ bs=1M count=64
	mkfs.vfat -F 32 $@ -n EFISys
	mcopy -i $@ -s $(TARGET_DIR)/boot/EFI ::

$(ISO): $(ESP_IMAGE)
	mkdir -p target/iso
	cp $(ESP_IMAGE) target/iso
	xorriso -as mkisofs -o $@ -e $(notdir $(ESP_IMAGE)) -no-emul-boot target/iso