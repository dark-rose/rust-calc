
all: floppy.img kernel
	./update_image.sh

floppy.img:
	./create_image.sh

kernel:
	make -C ./src

run-bochs: all
	bochs -f .bochsrc.txt

run-qemu: all
	qemu-system-i386 -fda floppy.img

clean:
	make -C ./src clean

fclean: clean
	rm -f floppy.img

.PHONY: fclean clean run-qemu run-bochs kernel all
