

all: kernel
	cp src/kernel iso/boot
	$(GRUB_RESCUE) -o image.iso iso/

kernel:
	make -C ./src

run-bochs: all
	$(BOCHS) -q -f .bochsrc.txt

run-bochs-debug: all
	$(BOCHSD) -q -f .bochsrc.txt

run-qemu: all
	qemu-system-i386 -cdrom image.iso

clean:
	make -C ./src clean

fclean: clean
	rm -f bochsout.txt iso/boot/kernel image.iso

.PHONY: fclean clean run-qemu run-bochs kernel all
