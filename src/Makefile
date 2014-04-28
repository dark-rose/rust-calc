
LD=ld
NASM=nasm

# Need rust 0.10
RUSTC=$(shell echo $(RUSTC010))/rustc

# Need a newer clang than the one in the Ubuntu repositories
CLANG=$(shell echo $(CLANG35))/clang


ASFLAGS=-felf

# Flags when compiling rust core
RUSTCF=--cfg libc --out-dir . -O -Z no-landing-pads

# All the object files we create
OBJS=boot.o interrupt.o
MAIN_OBJ=main.o

# All Rust source files
SOURCES=timer.rs keyboard_driver.rs isr.rs screen.rs desc_tables.rs kernel.rs kheap.rs calc.rs extra.rs
MAIN=main.rs

# Output name for rust-core
RUSTCORE=libcore-caef0f5f-0.0.rlib


all: $(RUSTCORE) $(OBJS) $(MAIN_OBJ) $(MAIN) kernel

clean:
	rm -f *.o kernel *.bc $(RUSTCORE)

kernel: $(OBJS) $(MAIN_OBJ) $(RUSTCORE)
	$(LD)  -melf_i386 -o kernel -T link.ld "-(" $(RUSTCORE) "-)" $(OBJS) $(MAIN_OBJ)

.s.o:
	$(NASM) $(ASFLAGS) -o $@ $<

$(RUSTCORE):
	$(RUSTC) $(RUSTCF) rust-core/core/lib.rs

main.bc: $(RUSTCORE) $(SOURCES) $(MAIN)
	$(RUSTC) --target i386-intel-linux -Z no-landing-pads --crate-type=lib --emit=bc -L . -o main.bc main.rs

main.o: main.bc
	$(CLANG) -c -O2 -o main.o main.bc

.PHONY: clean all
