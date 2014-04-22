
; Various multiboot values
MBOOT_PAGE_ALIGN    equ 1<<0
MBOOT_MEM_INFO      equ 1<<1
MBOOT_HEADER_MAGIC  equ 0x1BADB002	; Multiboot Magic value, says that we are
									; multiboot compatible

MBOOT_HEADER_FLAGS  equ MBOOT_PAGE_ALIGN | MBOOT_MEM_INFO

; When MBOOT_HEADER_MAGIC, MBOOT_HEADER_FLAGS and this is added together, the
; result should be zero
MBOOT_CHECKSUM      equ -(MBOOT_HEADER_MAGIC + MBOOT_HEADER_FLAGS)


[BITS 32]

[GLOBAL mboot]
[EXTERN code]
[EXTERN bss]
[EXTERN end]

; This is all to make our kernel multiboot compatible
mboot:
    dd MBOOT_HEADER_MAGIC
    dd MBOOT_HEADER_FLAGS
    dd MBOOT_CHECKSUM
    
    dd mboot
    dd code
    dd bss
    dd end
    dd start

[GLOBAL start]
[EXTERN kmain]

# Different functions required by rust-core, some only lead to endless loop
[GLOBAL abort]
[GLOBAL malloc]
[GLOBAL free]
[GLOBAL realloc]

; Actually loading the kernel
start:
    push ebx		; Load multiboot information:

    cli

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


