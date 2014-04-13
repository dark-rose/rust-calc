// Some helper functions

pub fn range(lo: uint, hi: uint, it: |uint| -> ()) {
    let mut iter = lo;
    while iter < hi {
        it(iter);
        iter += 1;
    }
}

// Convert an integer to a string
#[no_mangle] pub fn to_str_bytes(num : int, base : int) -> ~[u8]	{
	let neg = num < 0;
	let digits = "0123456789ABCDEF";
	let mut place = 0;
	let mut stored = num;
	let mut stor2 = num;

	let mut ret : ~[u8] = ~[0, ..65];
	if neg	{
		ret[place] = '-' as u8;
		place += 1;
		stored = -stored;
		stor2 = -stor2;
	}
	if num == 0	{
		ret[place] = '0' as u8;
		return ret;
	}
	while stored != 0	{
		place += 1;
		stored /= base as int;
	}
	stored = stor2;
	while stored != 0	{
		place -= 1;
		ret[place] = digits[stored % base as int];
		stored /= base as int;
	}
	return ret;
}

// Dumb and simple way to convert float to string
// Is not very accurate and almost guaranteed to produce a wrong result
#[no_mangle] pub fn float_to_str_bytes(num : f32) -> ~[u8]	{
	let neg = num < 0.0;
	let digits = "0123456789";
	let mut place = 0;
	let mut m_num = num;
	let mut whole : int;
	let mut stored : int;
	let mut stored_place : int;
	let mut rem : f32;

	let mut ret : ~[u8] = ~[0, ..65];

	if neg	{
		ret[place] = '-' as u8;
		place += 1;
		m_num = -num;
	}
	whole = m_num as int;

	if whole == 0	{
		ret[place] = '0' as u8;
		place += 1;
		ret[place] = '.' as u8;
		place += 1;
	}
	else	{
		stored = whole;
		while stored != 0	{
			place += 1;
			stored /= 10;
		}
		stored = whole;
		stored_place = place;
		while stored != 0	{
			stored_place -= 1;
			ret[stored_place] = digits[stored % 10];
			stored /= 10;
		}
		ret[place] = '.' as u8;
		place += 1;
		m_num -= whole as f32;
		
	}
	
	rem = m_num*10.0;
	range(0, 10, |_i|	{
		ret[place] = digits[rem as u8];
		place += 1;
		rem -= rem as u32 as f32;
		rem *= 10.0;
	});

	place -= 1;
	while ret[place] == '0' as u8	{
		ret[place] = 0x00;
		place -= 1;
	}
	if ret[place] == '.' as u8	{
		ret[place] = 0x00;
	}
	return ret;
}

