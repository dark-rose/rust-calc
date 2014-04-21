

// This should be implemented as a library and not a standalone executable
#![crate_type = "lib"]

// We are allowed to use C types
#![allow(ctypes)]

// We can't use any of the standard libraries
#![no_std]

// Needed for inline assembly
#![feature(asm)]

#![feature(macro_rules)]

extern crate core;

pub mod extra;
pub mod screen;
pub mod desc_tables;
pub mod kernel;
pub mod kheap;
pub mod calc;
pub mod isr;
pub mod keyboard_driver;


unsafe fn print_functions()	{
	screen::write_string(" * Multiply\n", 1);
	screen::write_string(" / Division\n", 2);
	screen::write_string(" + Addition\n", 3);
	screen::write_string(" - Subtract\n", 4);
	
	screen::write_string(" % Modulus\n", 1);
	screen::write_string(" ^ Exponentiation\n", 2);
}

// Stop rustc from mangling the function names
// no_split_stack is used to stop the linker from complaining that __morestack
// is missing, thanks to
// http://blog.theincredibleholk.org/blog/2013/11/18/booting-to-rust/
// The function is unsafe because we need to manipulate the VGA memory address
// directly
#[no_mangle] #[no_split_stack]
pub unsafe fn kmain() {
	screen::initialize_monitor(screen::White as u8, screen::Blue as u8);
//	screen::add_screen(0, 0, 80, 25, screen::Black as u8, screen::LightGray as u8);


	desc_tables::initialize_idt();
	// Input and result screen
	screen::add_screen(1, 1, 79, 4, screen::Black as u8, screen::LightGray as u8);
	
	// 1MB of memory
	kheap::kheap_add_block(0x01000000, 0x100000, 16);
//	kheap::kheap_add_block(0x01100000, 0x400, 16);

	asm!("sti");	// Enable interrupts


	/*
	// Basic sanity check on the memory, if we get to the calculator, it has
	// passed
	if kheap::heap_test(64) == false	{
		screen::write_string("Heap test failed", 0);
		return;
	}

	if kheap::realloc_test(64) == false	{
		screen::write_string("realloc test failed", 0);
		return;
	}
*/
	
	// Screens for the allowable operators
	screen::add_screen(1, 5, 20, 9, screen::Black as u8, screen::Cyan as u8);
	screen::add_screen(21, 5, 40, 9, screen::Black as u8, screen::Cyan as u8);
	screen::add_screen(41, 5, 60, 9, screen::Black as u8, screen::Cyan as u8);
	screen::add_screen(61, 5, 79, 9, screen::Black as u8, screen::Cyan as u8);
	
	// Screens that holds variables
	extra::range(0, 10, |i|	{
		screen::add_screen(1, (i+10) as u8, 26, (i+11) as u8, screen::Black as u8, screen::Cyan as u8);
	});
	extra::range(0, 10, |i|	{
		screen::add_screen(27, (i+10) as u8, 52, (i+11) as u8, screen::Black as u8, screen::Cyan as u8);
	});
	extra::range(0, 5, |i|	{
		screen::add_screen(53, (i+10) as u8, 79, (i+11) as u8, screen::Black as u8, screen::Cyan as u8);
	});
	screen::add_screen(53, 15, 79, 20, screen::Black as u8, screen::Cyan as u8);

	// Larger answer screen at the bottom
	screen::add_screen(1, 22, 79, 23, screen::Black as u8, screen::Cyan as u8);

	// All the variables are initialized to 0, just need to print it
	extra::range(0, 26, |i| {
		let scr_ind = (5 + i) as u8;
		screen::putc(i as u8 + 'A' as u8, scr_ind);
		screen::write_string(" = 0", scr_ind);
	});

	print_functions();

	// Initialize input from keyboard
	keyboard_driver::init_keyboard();
}
