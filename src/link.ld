OUTPUT_FORMAT("elf32-i386")
ENTRY(start)

SECTIONS
{
	. = 0x100000;	/* 1 MB */

	.text : AT(ADDR(.text))
	{
		*(.text)
	}

	.data ALIGN(4096) : AT(ADDR(.data))
	{
		*(.data)
		*(.rodata*)
	}

	.bss ALIGN(4096) : AT(ADDR(.bss))
	{
		*(.bss*)
	}
}
