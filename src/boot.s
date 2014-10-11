
[BITS 32]

MULTIBOOT_PAGE_ALIGN    equ 1<<0
MULTIBOOT_MEMORY_INFO   equ 1<<1
MULTIBOOT_HEADER_MAGIC  equ 0x1BADB002
MULTIBOOT_HEADER_FLAGS  equ MULTIBOOT_PAGE_ALIGN | MULTIBOOT_MEMORY_INFO
MULTIBOOT_CHECKSUM  equ -(MULTIBOOT_HEADER_MAGIC + MULTIBOOT_HEADER_FLAGS)

ALIGN 4
multiboot_header:
	dd MULTIBOOT_HEADER_MAGIC
	dd MULTIBOOT_HEADER_FLAGS
	dd MULTIBOOT_CHECKSUM


[GLOBAL start]
[EXTERN kmain]

# Different functions required by rust-core, some only lead to endless loop
[GLOBAL abort]
[GLOBAL malloc]
[GLOBAL free]
[GLOBAL realloc]

; Actually loading the kernel
start:
	mov esp, sys_stack	; Set up the kernel stack
	push ebx		; Push multiboot information:

	; enable SSE, needed for handling floating point values
	mov ecx, cr0
	btr ecx, 2	; clear CR0.EM bit
	bts ecx, 1	; set CR0.MP bit
	mov cr0, ecx

	mov ecx, cr4
	bts ecx, 9	; set CR4.OSFXSR bit
	bts ecx, 10	; set CR4.OSXMMEXCPT bit
	mov cr4, ecx

	call kmain		; Entry point in the kernel
	jmp $			; Infinite loop

[section .bss]
	resb 0x1000
	sys_stack:
