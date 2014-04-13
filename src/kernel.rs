
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


