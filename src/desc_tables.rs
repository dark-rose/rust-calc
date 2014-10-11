
use super::core::mem;
use kernel::outb;

// All our external interrupt routines
extern {
	fn isr0();
	fn isr1();
	fn isr2();
	fn isr3();
	fn isr4();
	fn isr5();
	fn isr6();
	fn isr7();
	fn isr8();
	fn isr9();
	fn isr10();
	fn isr11();
	fn isr12();
	fn isr13();
	fn isr14();
	fn isr15();
	fn isr16();
	fn isr17();
	fn isr18();
	fn isr19();
	fn isr20();
	fn isr21();
	fn isr22();
	fn isr23();
	fn isr24();
	fn isr25();
	fn isr26();
	fn isr27();
	fn isr28();
	fn isr29();
	fn isr30();
	fn isr31();
	
	fn isr128();

	fn irq0();
	fn irq1();
	fn irq2();
	fn irq3();
	fn irq4();
	fn irq5();
	fn irq6();
	fn irq7();
	fn irq8();
	fn irq9();
	fn irq10();
	fn irq11();
	fn irq12();
	fn irq13();
	fn irq14();
	fn irq15();
}

extern	{
	fn gdt_flush(ptr : u32);
}

#[allow(non_camel_case_types)]
#[packed]
struct IDT_entry	{
	base_low : u16,		// Lower part of address to function
	sel : u16,			// Kernel segment
	zero : u8,
	flags : u8,
	base_high : u16		// Higher part of address to function
}

#[allow(non_camel_case_types)]
#[packed]
struct IDT_pointer	{
	limit : u16,
	base : u32
}

#[no_mangle] static mut idt_pointer : IDT_pointer = IDT_pointer {limit : 0, base : 0};

#[no_mangle]
static mut idt_entry : [IDT_entry, ..256] = [IDT_entry {base_low : 0, sel :
0, zero : 0, flags : 0, base_high : 0}, ..256];

#[allow(non_camel_case_types)]
#[packed]
struct GDT_entry	{
	limit_low : u16,
	base_low : u16,
	base_mid : u8,
	access : u8,
	gran : u8,
	base_hi : u8
}

#[allow(non_camel_case_types)]
#[packed]
struct GDT_pointer	{
	limit : u16,
	base : u32
}

#[no_mangle] static mut gdt_pointer : GDT_pointer = GDT_pointer{limit : 0, base : 0};

#[no_mangle]
static mut gdt_entry : [GDT_entry, ..3] = [GDT_entry {
	limit_low : 0,
	base_low : 0,
	base_mid : 0,
	access : 0,
	gran : 0,
	base_hi : 0
}, ..3];



// Set one IDT gate
unsafe fn idt_set_gate(num : u8, func : extern "C" unsafe fn(), sel : u16, flags : u8)	{
	let base = func as u32;
	idt_entry[num].zero = 0;
	idt_entry[num].sel = sel;
	idt_entry[num].flags = flags;
	idt_entry[num].base_high = (base >> 16) as u16;
	idt_entry[num].base_low = (base & ((1 << 16) -1)) as u16;
}

unsafe fn gdt_set_gate(num : u8, base : u32, limit : u32, acc : u8, gran : u8)	{
	gdt_entry[num].limit_low = (limit & 0xFFFF) as u16;

	gdt_entry[num].base_low = (base & 0xFFFF) as u16;
	gdt_entry[num].base_mid = ((base >> 16) & 0xFF) as u8;
	gdt_entry[num].base_hi = ((base >> 24) & 0xFF) as u8;

	gdt_entry[num].access = acc;
	gdt_entry[num].gran = ((limit >> 16) & 0x0F) as u8;
	gdt_entry[num].gran |= gran & 0xF0;
}

// Remap the IRQ table
unsafe fn remap_irq()	{
	outb(0x20, 0x11);
	outb(0xA0, 0x11);
	outb(0x21, 0x20);
	outb(0xA1, 0x28);
	outb(0x21, 0x04);
	outb(0xA1, 0x02);
	outb(0x21, 0x01);
	outb(0xA1, 0x01);
	outb(0x21, 0x00);
	outb(0xA1, 0x00);
}


pub unsafe fn initialize_gdt()	{
	gdt_pointer.limit = ((mem::size_of::<GDT_entry>() * 3) - 1) as u16;
	gdt_pointer.base = &gdt_entry as *[GDT_entry, ..3] as u32;

	// Mandatory 0-entry
	gdt_set_gate(0, 0, 0, 0, 0);

	// Kernel space code, offset 0x08
	gdt_set_gate(1, 0, 0xFFFFFFFF, 0x9A, 0xCF);

	// Kernel space data, offset 0x10
	gdt_set_gate(2, 0, 0xFFFFFFFF, 0x92, 0xCF);

	gdt_flush(&gdt_pointer as *GDT_pointer as u32)
}

// Initialize the IDT table
pub unsafe fn initialize_idt()	{
	idt_pointer.limit = (mem::size_of::<IDT_pointer>() * 256 - 1) as u16;
	idt_pointer.base = &idt_entry as *[IDT_entry, ..256] as u32;

	remap_irq();

	idt_set_gate(0, isr0, 0x08, 0x8E);
	idt_set_gate(1, isr1, 0x08, 0x8E);
	idt_set_gate(2, isr2, 0x08, 0x8E);
	idt_set_gate(3, isr3, 0x08, 0x8E);
	idt_set_gate(4, isr4, 0x08, 0x8E);
	idt_set_gate(5, isr5, 0x08, 0x8E);
	idt_set_gate(6, isr6, 0x08, 0x8E);
	idt_set_gate(7, isr7, 0x08, 0x8E);
	idt_set_gate(8, isr8, 0x08, 0x8E);
	idt_set_gate(9, isr9, 0x08, 0x8E);
	idt_set_gate(10, isr10, 0x08, 0x8E);
	idt_set_gate(11, isr11, 0x08, 0x8E);
	idt_set_gate(12, isr12, 0x08, 0x8E);
	idt_set_gate(13, isr13, 0x08, 0x8E);
	idt_set_gate(14, isr14, 0x08, 0x8E);
	idt_set_gate(15, isr15, 0x08, 0x8E);
	idt_set_gate(16, isr16, 0x08, 0x8E);
	idt_set_gate(17, isr17, 0x08, 0x8E);
	idt_set_gate(18, isr18, 0x08, 0x8E);
	idt_set_gate(19, isr19, 0x08, 0x8E);
	idt_set_gate(20, isr20, 0x08, 0x8E);
	idt_set_gate(21, isr21, 0x08, 0x8E);
	idt_set_gate(22, isr22, 0x08, 0x8E);
	idt_set_gate(23, isr23, 0x08, 0x8E);
	idt_set_gate(24, isr24, 0x08, 0x8E);
	idt_set_gate(25, isr25, 0x08, 0x8E);
	idt_set_gate(26, isr26, 0x08, 0x8E);
	idt_set_gate(27, isr27, 0x08, 0x8E);
	idt_set_gate(28, isr28, 0x08, 0x8E);
	idt_set_gate(29, isr29, 0x08, 0x8E);
	idt_set_gate(30, isr30, 0x08, 0x8E);
	idt_set_gate(31, isr31, 0x08, 0x8E);

	idt_set_gate(32, irq0, 0x08, 0x8E);
	idt_set_gate(33, irq1, 0x08, 0x8E);
	idt_set_gate(34, irq2, 0x08, 0x8E);
	idt_set_gate(35, irq3, 0x08, 0x8E);
	idt_set_gate(36, irq4, 0x08, 0x8E);
	idt_set_gate(37, irq5, 0x08, 0x8E);
	idt_set_gate(38, irq6, 0x08, 0x8E);
	idt_set_gate(39, irq7, 0x08, 0x8E);
	idt_set_gate(40, irq8, 0x08, 0x8E);
	idt_set_gate(41, irq9, 0x08, 0x8E);
	idt_set_gate(42, irq10, 0x08, 0x8E);
	idt_set_gate(43, irq11, 0x08, 0x8E);
	idt_set_gate(44, irq12, 0x08, 0x8E);
	idt_set_gate(45, irq13, 0x08, 0x8E);
	idt_set_gate(46, irq14, 0x08, 0x8E);
	idt_set_gate(47, irq15, 0x08, 0x8E);
	
	idt_set_gate(48, isr128, 0x08, 0x8E);

	// Load IDT
	asm!("lidt ($0)" :: "r" (idt_pointer));
}
