#!/bin/bash

mkdir floppy
chmod 0777 floppy
kernel="src/kernel"

sudo mount -oloop floppy.img floppy
sudo cp ${kernel} floppy/kernel
sudo umount floppy

rmdir floppy
