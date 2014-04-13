// Simple bitmap implementation of the kernel heap
use super::core::mem::size_of;
use super::core::vec::Vec;

use extra;

/*
TODO:
- This implementation is not tested at all.
- It seems to work ok when using it
- Only one block is allocated, if we need more memory than that one block, it
  will fail. This can be handled in "alloc"
- realloc is not implemented, which limits functionality a bit
- used field in Header is not updated when allocating or freeing data
*/

// One block of memory that can be allocated
#[packed] struct Header	{
	next : *mut Header,	// Next block of data
	size : uint,		// Size of this block (in bytes)
	used : uint,		// Number of bytes available in this block
	bsize : uint		// Minimum number of bytes that is allocated each time,
						// this is also the alignment
}

// The start for the list of blocks
struct List	{
	head : *mut Header
}


// Not very Rust-like, but we use NULL for empty pointer
static mut kheap : List = List{head : 0 as *mut Header};

// Allocate a block of memory that can be used to be further subdivided when
// malloc-ing data
// addr: physical address of the block
// size: Number of bytes in this block
// bsize: Size (in bytes) of each block in this block, this becomes the minimum
// number of bytes that can be allocated and is also the alignment
pub unsafe fn kheap_add_block(addr : uint, size : uint, bsize : uint)	{
	// The header with all the other metadata is placed at the beginning
	let b : *mut Header = addr as *mut Header;
	(*b).size = size - size_of::<Header>();
	(*b).bsize = bsize;

	// Place this at the beginning of the list
	(*b).next = kheap.head;
	kheap.head = b;

	// Number of blocks in this block
	let bcnt : uint = (size / bsize) as uint;

	// Number of bytes we must allocate in the beginning for our bitmap
	// Each bit represent 1 block, hence we have to divide by 8
	//let bytes_map : uint = bcnt / 8;
	let bytes_map : uint = (bcnt / (8*bsize))+1;

	// Get a pointer to the first available byte after our header
	let bm : *mut u8 = ((b as uint) + size_of::<Header>()) as *mut u8;

	// Mark all memory in this block as available
	let mut x : uint = 0;
	while x < bytes_map	{
		(*((bm as uint + x) as *mut u8)) = 0 as u8;
		x += 1;
	}

	// Mark the memory for the header and this bitmap as memory that is taken
	// It might mark more than what is strictly used.
	x = 0;
	//while x < (bytes_map/8+1) as uint	{
	while x < (bytes_map/8+1) as uint	{
		(*((bm as uint + x) as *mut u8)) = 0xff as u8;
		x += 1;
	}

	// Mark the number of bytes used
	(*b).used = bytes_map;
}

unsafe fn kheap_find_block(size : uint, header : *Header, mark : bool) -> uint	{
	// Number of concurrent bits we need to be set to 0
	let need : uint = size / (*header).bsize +
		if size % (*header).bsize == 0	{ 0 }
		else { 1 };
	
	// Start of our bitmap
	let bm : *mut u8 = ((header as uint) + size_of::<Header>()) as *mut u8;
	// Number of bitmap blocks
	let blocks : uint = ((*header).size / (*header).bsize / 8) as uint;
	let mut free_bits = 0;
	let mut x : uint = 0;
	while x < blocks	{
		let y : u8 = (*((bm as uint + x) as *mut u8)) as u8;
		let mut a : uint = 0;
		while a < 8	{
			if (1 << a) & y == 0	{
				free_bits += 1;
			}
			else	{
				free_bits = 0;
			}
			if free_bits >= need	{
				// If we should mark this memory as taken
				if mark == true	{
					let mut rem = free_bits;
					let mut aa = a;
					let mut xx = x;
					while rem > 0	{
						(*((bm as uint + xx) as *mut u8)) |= 1 << aa;
						if aa == 0	{
							aa = 7;
							xx -= 1;
						}
						else	{
							aa -= 1;
						}
						rem -= 1;
					}
				}
				// Return the first block index
				return x*8 + (a - free_bits);
			}
			a += 1;
		}
		x += 1;
	}
	0	// Nothing is found, first block is always bitmap, so an ok return value
}

// We have not stored how much is allocated
pub unsafe fn kheap_malloc(size : uint) -> *mut u8	{
	let mut b : *mut Header = kheap.head;

	while b != 0 as *mut Header	{
		// There is (theoretically) enough room in this block
		if (*b).size - (*b).used >= size	{
			let x = kheap_find_block(size, b as *Header, true);
			if x != 0	{
				// Has already marked it as used, only need to return it
				let ret = (b as uint + ((*b).bsize * x) + size_of::<Header>())
					as *mut u8;
				// Mark how many blocks are used
				*(ret as *mut u32) = x as u32;
				return (ret as uint + 4) as *mut u8;

			}
		}
		b = (*b).next;
	}

	return 0 as *mut u8;
}

pub unsafe fn kheap_free(ptr : *mut u8)	{
	if ptr as uint == 0	{
		return;
	}
		
	let mut b : *mut Header = kheap.head;
	let mem : uint = ptr as uint - 4;

	while b != 0 as *mut Header	{
		if mem > b as uint && mem < (b as uint + (*b).size)	{
			// Number of bytes inside the current block
			let base : uint = mem - b as uint - size_of::<Header>();
			// Number of blocks that should be freed
			let blocks : u32 = *(mem as *mut u32);

			// Offset (number of bytes) before forst bitmap we should free
			let bytes = base / (*b).bsize / 8;
			let bits_rem = (base / (*b).bsize) % 8;
			let mut x = bytes;
			let mut y = bits_rem;
			let mut a = 0;
			while a < blocks	{
				(*((b as uint + size_of::<Header>() + x as uint) as *mut u8)) &=
					0xff & !(1 << y);
				if y == 7	{
					y = 0;
					x += 1;
				}
				else	{
					y += 1;
				}
				a += 1;
			}
		}
		else	{
			return;

		}
		b = (*b).next;
	}

}

// Should allocate more if there is no more memory left
#[no_mangle] pub extern "C" fn malloc(len : uint) -> *mut u8	{
	unsafe	{
		return kheap_malloc(len);
	}
}

#[no_mangle] pub extern "C" fn free(ptr: *mut u8)	{
	unsafe	{
		kheap_free(ptr);
	}
}


// Simple heap test to check that it's not completely broken. Does not check
// addresses that the addresses are correct, but does check that we are able to
// allocate memory and reallocate free-ed memory
// input value of 512 allocates:
// - 512*4 = 2KB and up to 1023*4 ~= 4KB 511 times
// Which should be possible when the available memory to allocate is at least
// 4KB
pub fn heap_test(len : uint) -> bool {
    let mut res : int = 0;
	let mut ret : bool = true;
    extra::range(0, len, |i| {
		res = 0;
 		let mut tmp : Vec<int> = Vec::with_capacity(len+i);
		extra::range(0, len+i, |j|	{
			tmp.push(j as int);
		});
		extra::range(0, len+i, |_j|	{
			res += tmp.pop().get();
		});
		// N*(N-1)/2
		let expected : int = ((len+i)*(len+i-1)/2) as int;
		if res != expected	{
			ret = false;
		}
    });
	return ret;
}
