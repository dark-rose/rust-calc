use super::core::str::as_bytes;
use super::core::slice::iter;
use super::core::iter::Iterator;
use super::core::option::{Some, None};
use extra::range;
use kernel::outw;
use extra;

pub enum Color {
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Pink = 5,
	Brown = 6,
	LightGray = 7,
	DarkGray = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	LightPink = 13,
	Yellow = 14,
	White = 15,
}

static VGA_ADDRESS: uint = 0xb8000;
static VGA_WIDTH  : uint = 80;
static VGA_HEIGHT : uint = 24;

#[packed]
struct Screen	{
	x : u8,
	y : u8,
	color : u8,
	tab_sz : u8,
	stop_x : u8,
	stop_y : u8,
	start_x : u8,
	start_y : u8
}

// 40 screens is the maximum
#[no_mangle] static mut screen : [Screen, ..40] = [Screen {
	x: 0, y: 0, color: 0, tab_sz: 4, stop_x : VGA_WIDTH as u8,
	stop_y : VGA_HEIGHT as u8, start_x : 0, start_y : 0} , ..40];

// Numbers of screens used
static mut screen_used : u8 = 0;

pub unsafe fn add_screen(startx: u8, starty: u8, stopx: u8, stopy: u8, fg: u8, bg: u8)	{
	screen[screen_used].color = (fg | (bg << 4)) as u8;
	screen[screen_used].start_x = startx;
	screen[screen_used].start_y = starty;
	screen[screen_used].stop_x = stopx;
	screen[screen_used].stop_y = stopy;
	screen[screen_used].x = 0;
	screen[screen_used].y = 0;
	clear_screen(screen_used);
	screen_used += 1;
}

pub unsafe fn initialize_monitor(fg: u8, bg: u8)	{
	let color : u8 = fg | (bg << 4);
	let blank : u16 = (color as u16 << 8) | (0x20 as u16);
    range(0, VGA_WIDTH*VGA_HEIGHT, |i| {
        *((VGA_ADDRESS + i * 2) as *mut u16) = blank;
    });

	// Hide the cursor
	outw(0x3D4,0x200A);
    outw(0x3D4,0xB);
}

pub unsafe fn clear_screen(ind : u8)	{
	let blank : u16 = (screen[ind].color as u16 << 8) | (0x20 as u16);
	let x : u8 = screen[ind].start_x;
	let y : u8 = screen[ind].start_y;
	range(x as uint, screen[ind].stop_x as uint, |i|	{
		range(y as uint, screen[ind].stop_y as uint, |j|	{
	        *((VGA_ADDRESS + (j*80+i)* 2) as *mut u16) = blank;
		});
	});
	screen[ind].x = 0;
	screen[ind].y = 0;
}

// Put one character on the given screen
pub unsafe fn putc(c: u8, ind : u8)	{
	let att_byte : u16 = (screen[ind].color as u16 << 8) | (c as u16);
	if c == 0x08 && screen[ind].x > 0	{
		screen[ind].x -= 1;
		*((VGA_ADDRESS + ((screen[ind].y+screen[ind].start_y) as uint *
		80+(screen[ind].x+screen[ind].start_x) as uint )*2) as *mut u16) =
		0x20 as u16 | (att_byte & 0xff00);
	}
	// Tab
	else if c == 0x09	{
		screen[ind].x = (screen[ind].x+screen[ind].tab_sz) & !(screen[ind].tab_sz-1);
	}
	else if c as char == '\r'	{
		screen[ind].x = screen[ind].start_x;
	}
	else if c as char == '\n'	{
		screen[ind].x = screen[ind].start_x;
		screen[ind].y += 1;
	}
	else if c as char >= ' ' && c as char <= '~'	{
		*((VGA_ADDRESS + ((screen[ind].y+screen[ind].start_y) as uint *
		80+(screen[ind].x+screen[ind].start_x) as uint )*2) as *mut u16) = 
		c as u16| att_byte;
		screen[ind].x += 1;
	}
	
	if screen[ind].x >= screen[ind].stop_x	{
		screen[ind].x = 0;
		screen[ind].y += 1;
	}

	// Scrolling is not done, because it should not be needed
}

// Write a string to a given screen
pub unsafe fn write_string(s: &'static str, ind : u8)	{
	let bytes : &[u8] = as_bytes(s);
	for b in super::core::slice::iter(bytes)	{
		putc(*b, ind);
	}
}

// Write a floating point number to the screen
pub unsafe fn write_float(num : f32, ind : u8)	{
	let m = extra::float_to_str_bytes(num);
	let mut l : int = 0;
	while m[l] != 0x00	{
		putc(m[l] as u8, ind);
		l += 1;
	}

}

// Write an integer to the screen
pub unsafe fn write_int(num : int, base : int, ind : u8)	{
	let m = extra::to_str_bytes(num, base);
	let mut l = 0;
	while m[l] != 0x00	{
		putc(m[l], ind);
		l += 1;
	}
}

