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
sudo umount floppy


grub --batch <<EOT
device (fd0) floppy.img
install (fd0)/boot/grub/stage1 (fd0) (fd0)/boot/grub/stage2
(fd0)/boot/grub/menu.lst
quit
EOT

rmdir floppy
