[BITS 32]

[GLOBAL gdt_flush]

; Writes the GDT pointer
; 32-bit address is passed as a parameter on the stack to the function
gdt_flush:
	mov eax, [esp+4]	; Get address
	lgdt [eax]			; Load address in register

	; Data segment is at offset 0x10
	mov ax, 0x10
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
	mov ss, ax

	; Change CS to 0x08
	jmp 0x08:.flush
.flush:
	ret
