
use screen;

pub unsafe fn outb(port: u16, value: u8)	{
	asm!("outb %al, %dx"
	:
	: "{dx}" (port), "{al}" (value));
}

pub unsafe fn outw(port: u16, value: u16)	{
	asm!("outw %ax, %dx"
	:
	: "{dx}" (port), "{ax}" (value));
}

pub unsafe fn inb(port: u16) -> u8	{
	let ret : u8;
	asm!("inb %dx, %al"
	: "={al}" (ret)
	: "{dx}" (port));
	return ret;
}

pub unsafe fn inw(port: u16) -> u16	{
	let ret : u16;
	asm!("inw %dx, %ax"
	: "={ax}" (ret)
	: "{dx}" (port));
	return ret;
}

#[no_mangle] pub extern "C" fn abort()	{
	unsafe {
		screen::reset_monitor();
		screen::initialize_monitor(screen::Red as u8, screen::Black as u8);
		screen::add_screen(28, 12, 40, 14, screen::Red as u8, screen::Black as u8);
		screen::write_string("ABORT", 0);
	}

	loop { }	// Endless loop
}

// Reboot the computer
pub unsafe fn reboot()	{
	let mut tmp : u8;

	// Disable interrupts
	asm!("cli");

	loop	{
		tmp = inb(0x64);
		if tmp & 1 != 0	{
			inb(0x60);	// Clear data
		}
		if (tmp & 2) == 0	{
			break;
		}
	}

	outb(0x64, 0xFE);
	loop	{
		asm!("hlt");
	}
}
