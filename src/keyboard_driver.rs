
use isr;
use isr::Registers;
use kernel::{outb, inb};
use calc;
use core::option::{Option, Some, None};

static mut shift_pressed : bool = false;

// Not sure how accurate this mapping is
pub static SCAN_CODE: &'static [u8] = bytes!("\x00\x1B1234567890-=\x08\tqwertyuiop[]\n\x00asdfghjkl;'`\x00\\zxcvbnm,./\x00*\x00 \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00789-456+1230.\x00\x00\x00\x00\x00");

pub static SCAN_CODE_SHIFTED: &'static [u8] = bytes!("\x00\x1B!@#$%^&*()_+\x08\tQWERTYUIOP{}\n\x00ASDFGHJKL:\"~\x00|ZXCVBNM<>\x00\x00*\x00 \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00789-456+1230.\x00\x00\x00\x00\x00");


// Register the callback function for each keypress
pub fn init_keyboard()	{
	isr::register_interrupt_handler(33, handle_keyboard_interrupt);
}

// The callback function for each keypress
#[no_mangle] unsafe fn handle_keyboard_interrupt(_regs : &Registers)	{
	// Get the actual byte
	let sc = inb(0x60);
	check_shift(sc);
	match get_char(sc) {
        None => {}, 
		Some(c) => calc::add_key(c)
    };
	outb(0x20, 0x20);
}

// Mark shift as pressed or unpressed
pub fn check_shift(sc: u8) {
	// Check right and left shift and if it's up or down
	if sc == 0x2A || sc == 0x36	{
		unsafe {shift_pressed = true; }
	}
	else if sc == 0xAA || sc == 0xB6	{
		unsafe {shift_pressed = false; }
	}
}

// Get the character or None if it's not a printable character
pub fn get_char(scancode : u8) -> Option<u8> {
	// Make sure we don't go out of bounds
    if scancode >= 88 || scancode < 2 {
   	    return None;
	}
	unsafe {
		// Check if the value is 0x00
	    if (shift_pressed == false && SCAN_CODE[scancode] == 0 as u8) ||
		(shift_pressed == true && SCAN_CODE_SHIFTED[scancode] == 0 as u8)	{
			return None
	    }
	}
    unsafe {
        if shift_pressed {
            Some(SCAN_CODE_SHIFTED[scancode])
        } else {
            Some(SCAN_CODE[scancode])
        }
    }
}
