use isr;
use kernel::{outb,inb};
use screen;
use isr::Registers;

// http://wiki.osdev.org/Programmable_Interval_Timer and
// http://wiki.osdev.org/CMOS

// Current number of ticks since startup
static mut tick : u32 = 0;

// The index to the screen that contains the clock
static mut scr_ind : u8 = 0;

// The number of ticks generated each second, will be set when timer is
// initialized
static mut ticks_per_sec : u32 = 1;

// All the fields we use in the clock
struct Clock	{
	secs : u8,
	mins : u8,
	hour : u8,
	day : u8,
	month : u8,
	year : u8
}

#[allow(non_camel_case_types)]
enum PIT_ports	{
	PIT_CHANNEL0 = 0x40,
	PIT_CHANNEL1 = 0x41,
	PIT_CHANNEL2 = 0x42,
	PIT_COMMAND = 0x43,
}

#[allow(non_camel_case_types)]
enum CMOS_ports	{
	CMOS_ADDRESS = 0x70,
	CMOS_DATA = 0x71,
}

#[allow(non_camel_case_types)]
enum CMOS_data	{
	CMOS_SECONDS = 0x00,
	CMOS_MINUTES = 0x02,
	CMOS_HOURS = 0x04,
	CMOS_WEEKDAY = 0x06,
	CMOS_DAY_OF_MONTH = 0x07,
	CMOS_MONTH = 0x08,
	CMOS_YEAR = 0x09,
	CMOS_STATUS_REG_A = 0x0A,
	CMOS_STATUS_REG_B = 0x0B,
}

// Current time
static mut current_time : Clock = Clock{
	secs : 0, mins : 0, hour : 0, day : 0, month : 0, year : 0
};

// Number of days in each month
// TODO: Does not handle leap year
static Months : [u8, ..12] = [31,28,31,30,31,30,31,31,30,31,30,31];

#[no_mangle] unsafe fn handle_timer_interrupt(_regs : &Registers)	{
	tick += 1;

	// If another second has passed, we increment and write new clock
	if tick % ticks_per_sec == 0	{
		inc_clock();
		write_clock(current_time, scr_ind);
	}
}

pub fn init(freq : u32)	{
	isr::register_interrupt_handler(32, handle_timer_interrupt);

	let div :u32 = 1193180 / freq;

	unsafe {
		ticks_per_sec = freq;
		
		// Use channel 1
		outb(PIT_COMMAND as u16, 0x34);

		// Lower byte then upper byte
		outb(PIT_CHANNEL0 as u16, (div & 0xff) as u8);
		outb(PIT_CHANNEL0 as u16, ((div>>8) & 0xff) as u8);
	}
}


// Increment the current time by one second
unsafe fn inc_clock()	{
	current_time.secs += 1;
	if current_time.secs >= 60	{
		current_time.secs = 0;
		current_time.mins += 1;
		if current_time.mins >= 60	{
			current_time.mins = 0;
			current_time.hour += 1;
			if current_time.hour >= 24	{
				current_time.hour = 0;
				current_time.day += 1;
				if current_time.day > Months[current_time.month-1]	{
					current_time.day = 1;
					current_time.month += 1;
					if current_time.month > 12	{
						current_time.month = 1;
						current_time.year += 1;
					}
				}
			}
		}
	}
}

unsafe fn read_rtc_update() -> bool	{
	outb(CMOS_ADDRESS as u16, CMOS_STATUS_REG_A as u8);
	return (inb(CMOS_DATA as u16) & 0x80) != 0;
}

unsafe fn get_rtc_reg(reg : u8)	-> u8	{
	outb(CMOS_ADDRESS as u16, reg);
	return inb(CMOS_DATA as u16);
}

unsafe fn read_rtc_clock_values() -> Clock	{
	let mut cl : Clock = Clock{
		secs : 0, mins : 0, hour : 0, day : 0, month : 0, year : 0
	};
	cl.secs = get_rtc_reg(CMOS_SECONDS as u8);
	cl.mins = get_rtc_reg(CMOS_MINUTES as u8);
	cl.hour = get_rtc_reg(CMOS_HOURS as u8);
	cl.day = get_rtc_reg(CMOS_DAY_OF_MONTH as u8);
	cl.month = get_rtc_reg(CMOS_MONTH as u8);
	cl.year = get_rtc_reg(CMOS_YEAR as u8);
	return cl;
}

// Turn the clock into something that is easier to understand
unsafe fn normalize_clock(cl : Clock) -> Clock	{
	let mut c = cl;
	let reg_b = get_rtc_reg(CMOS_STATUS_REG_B as u8);
	if reg_b & 0x04 == 0	{
		c.secs = (cl.secs & 0x0F) + ((cl.secs / 16) * 10);
		c.mins = (cl.mins & 0x0F) + ((cl.mins / 16) * 10);
		c.hour = (cl.hour & 0x0F) + (((cl.hour & 0x70) / 16) * 10);
		c.day = (cl.day & 0x0F) + ((cl.day / 16) * 10);
		c.month = (cl.month & 0x0F) + ((cl.month / 16) * 10);
		c.year = (cl.year & 0x0F) + ((cl.year / 16) * 10);
	}
	return c;
}

unsafe fn write_digits(n : u8, ind : u8)	{
	screen::putc( (n/10) + '0' as u8, ind);
	screen::putc( (n-((n/10)*10)) + '0' as u8, ind);
}

unsafe fn write_clock(cl : Clock, scr : u8)	{
	screen::clear_screen(scr);
	write_digits(cl.day, scr); screen::putc('/' as u8, scr);
	write_digits(cl.month, scr); screen::putc('-' as u8, scr);
	write_digits(cl.year, scr); screen::putc(' ' as u8, scr);
	write_digits(cl.hour, scr); screen::putc(':' as u8, scr);
	write_digits(cl.mins, scr); screen::putc(':' as u8, scr);
	write_digits(cl.secs, scr);
}

pub fn read_rtc_clock(scr : u8)	{
	unsafe	{
		// Make sure we are not reading while it's updating
		while read_rtc_update()	{ }

		current_time = read_rtc_clock_values();
		current_time = normalize_clock(current_time);

		scr_ind = scr;	// Set the global variable

		write_clock(current_time, scr);
	}
}
