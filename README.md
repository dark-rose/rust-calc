rust-calc
=========

Calculator in Rust written as a 32 bit OS kernel.

A very simple operating system written in Rust and a bit of Assembly. It can
only be used as a basic calculator and was written just for fun.

Prerequisites:
- Rust 0.10 - Path must be placed in variable $RUSTC010
- Clang 3.5 (Not sure on the exact version number, but must be newer than 3.0) -
  Path must be placed in $CLANG35
- ld and nasm
- Bochs (must be placed in $BOCHS) or Qemu
- Grub2 (grub-mkrescue in $GRUB\_RESCUE)

Building:

    cd rust-calc
    git submodule init
    git submodule update
    make

Running:

    make run-bochs
    OR
    make run-qemu
