#!/bin/bash
# Create a floppy disk that can boot the kernel

mkdir floppy

dd if=/dev/zero of=floppy.img bs=1024 count=1440

# ext2 filesystem
mke2fs -F floppy.img

sudo mount -oloop floppy.img floppy
mkdir floppy/boot
mkdir floppy/boot/grub
cp grub/stage1 floppy/boot/grub
cp grub/stage2 floppy/boot/grub
cp grub/menu.lst floppy/boot/grub

grub --batch --device-map=/dev/null <<EOT
device (fd0) floppy.img
root (fd0)
setup (fd0)
quit
EOT

sudo umount floppy

rmdir floppy
