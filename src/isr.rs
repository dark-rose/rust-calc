
use kernel::outb;

#[packed]
pub struct Registers	{
	ds : u32,
	edi : u32,
	esi : u32,
	ebp : u32,
	tampered_esp : u32,
	ebx : u32,
	edx : u32,
	ecx : u32,
	eax : u32,
	int_no : u32,
	err_code : u32,
	eip : u32,
	cs : u32,
	eflags : u32,
	esp : u32,
	ss : u32
}

// Dummy initialazation function for uninitialized interrupts
pub fn int_handler_dummy(_regs : &Registers)	{

}

// All interrupt handlers
pub static mut int_handlers: [unsafe fn(&Registers), ..256] = [int_handler_dummy, ..256];


pub fn register_interrupt_handler(ind : u8, f : unsafe fn(&Registers) )	{
	unsafe	{ int_handlers[ind] = f; }
}


#[no_mangle]
pub extern "C" fn isr_handler(regs : Registers)	{
	let int_no : u8 = (regs.int_no & 0xff) as u8;
	unsafe { int_handlers[int_no](&regs); }
}

#[no_mangle]
pub extern "C" fn irq_handler(regs : Registers)	{
	let int_no : u8 = (regs.int_no & 0xff) as u8;
	if int_no >= 40	{
		// Reset signal to slave
		unsafe { outb(0xA0, 0x20); }
	}
	unsafe {
		// Reset signal to master
		outb(0x20, 0x20);

		// Call function
		int_handlers[int_no](&regs);
	}
}
