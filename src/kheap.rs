// Simple bitmap implementation of the kernel heap
use super::core::mem::size_of;
use super::core::vec::Vec;

use extra;

/*
TODO:
- This implementation is not tested very well
- It seems to work ok when using it
*/

// One block of memory that can be allocated
#[packed] struct Header	{
	next : *mut Header,	// Next block of data
	size : uint,		// Size of this block (in bytes)
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
	let bytes_map : uint = bcnt / 8;

	// Get a pointer to the first available byte after our header
	let bm : *mut u8 = ((b as uint) + size_of::<Header>()) as *mut u8;

	// Mark all memory in this block as available
	let mut x : uint = 0;
	while x < bytes_map+1	{
		(*((bm as uint + x) as *mut u8)) = 0 as u8;
		x += 1;
	}

	// Mark the memory for the header and this bitmap as memory that is taken
	// It might mark more than what is strictly used.
	x = 0;
	while x < (bytes_map/8) as uint	{
		(*((bm as uint + x) as *mut u8)) = 0xff as u8;
		x += 1;
	}
	let mut i = 0;
	while i < (bytes_map%8)	{
		(*((bm as uint + x) as *mut u8)) |= 1 << i;
		i += 1;
	}
}

unsafe fn kheap_find_block(size : uint, header : *Header, mark : bool) -> (uint,uint)	{
	// Number of concurrent bits we need to be set to 0
	let need : uint = size / (*header).bsize +
		if size % (*header).bsize == 0	{ 0 }
		else { 1 };

	// Start of our bitmap
	let bm : *mut u8 = ((header as uint) + size_of::<Header>()) as *mut u8;

	// Number of whole blocks
	let blocks : uint = ((*header).size / (*header).bsize) as uint;
	let mut free_bits = 0;
	let mut x : uint = 0;

	while x < blocks	{
		let y : u8 = (*((bm as uint + (x/8)) as *mut u8)) as u8;
		let a = x%8;
		if (1 << a) & y == 0	{
			free_bits += 1;
		}
		else	{
			free_bits = 0;
		}
		if free_bits >= need	{
			if mark == true	{
				// Mark these bits as taken
				let mut xx = x;
				let mut free = free_bits;
				while free > 0	{
					(*((bm as uint + xx/8) as *mut u8)) |= 1 << (xx%8);
					xx -= 1;
					free -= 1;
				}
			}
			return (x-free_bits, need);
		}
		x += 1;
	}

	return (0, need);	// Nothing is found, first block is always bitmap, so an ok return value
}

// We have not stored how much is allocated
unsafe fn kheap_malloc(size : uint) -> *mut u8	{
	let mut b : *mut Header = kheap.head;

	while b != 0 as *mut Header	{
		// There is (theoretically) enough room in this block
		if (*b).size >= (size+4)	{
			// First 4 bytes is used to store length so we need an extra 4 bytes
			let (x, need) = kheap_find_block(size+4, b as *Header, true);
			if x != 0	{
				// Has already marked it as used, only need to return it
				let ret = (b as uint + ((*b).bsize * x) + size_of::<Header>())
					as *mut u8;
				// Mark how many blocks are used
				*(ret as *mut u32) = need as u32;
				return (ret as uint + 4) as *mut u8;

			}
		}
		b = (*b).next;
	}
	return 0 as *mut u8;
}

unsafe fn kheap_free(ptr : *mut u8)	{
	if ptr as uint == 0	{
		return;
	}
		
	let mut b : *mut Header = kheap.head;
	let mem : uint = ptr as uint - 4;

	while b != 0 as *mut Header	{
		if mem > b as uint && mem < (b as uint + (*b).size)	{
			// Number of blocks that should be freed
			let blocks : u32 = *(mem as *mut u32);

			// Number of bytes inside the current block
			let base : uint = mem - b as uint - size_of::<Header>();

			let end = base / (*b).bsize + blocks as uint;

			let mut a = blocks as uint - 1;
			loop	{
				let ind = (end-a) / 8;
				let off = (end-a) % 8;
				(*((b as uint + size_of::<Header>() + ind as uint) as *mut u8)) &=
					0xff & !(1 << off);
				if a == 0	{
					break;
				}
				a -= 1;
			}
		}
		b = (*b).next;
	}

}

unsafe fn memcpy(to : *mut u8, from : *mut u8, len : uint)	{
	extra::range(0, len, |i|	{
		*((to as uint + i) as *mut u8) = *((from as uint + i) as *mut u8);
	});
}


/*
Lazy way to do realloc. Just new alloc and copy over all previous data before
freeing the previous. Causes a lot of fragmentation if used extensively.
*/
unsafe fn kheap_realloc(ptr : *mut u8, sz : uint) -> *mut u8	{
	if ptr as uint == 0	{
		// If it's empty we allocate new memory
		return malloc(sz);
	}
	if sz == 0	{
		return 0 as *mut u8;
	}
	let old_len : uint = *((ptr as uint - 4) as *uint);

	// Start of the block
	let mut b : *mut Header = kheap.head;
	let mem : uint = ptr as uint - 4;

	// As long as we are not at the end of the blocks
	while b != 0 as *mut Header	{
		// If the address belong to this block
		if mem > b as uint && mem < (b as uint + (*b).size)	{
			// If new length is shorter we just ignore it, worst case scenario is that
			// we use up extra memory
			if (sz+4) <= old_len*(*b).bsize	{
				return ptr;
			}

			let ret : *mut u8 = malloc(sz);

			if ret as uint == 0	{
				// If we failed to allocate new memory we free the old one and
				// return 0
				free(ptr);
				return 0 as *mut u8;;
			}

			// Copy all the aold memory over to the new block
			memcpy(ret, ptr, (*b).bsize*old_len);
			free(ptr);
			return ret;
		}
		b = (*b).next;
	}

	// If we get to this point we where unable to find which block it belongs to
	return 0 as *mut u8;
}

unsafe fn kheap_malloc_internal(len : uint) -> *mut u8	{
	let ret : *mut u8 = kheap_malloc(len);
	if ret == 0 as *mut u8	{
		if kheap.head as uint == 0	{
			return 0 as *mut u8;
		}
		let size : uint = (*kheap.head).size + size_of::<Header>();
		let newblock : uint = kheap.head as uint + size + 1;
		let bsize : uint =  (*kheap.head).bsize;
		kheap_add_block(newblock, size, bsize);

		// If the data that should be allocated is larger than we can fit in the
		// block, this will still not work
		return kheap_malloc(len);
	}
	return ret;
}

#[no_mangle] pub extern "C" fn realloc(ptr : *mut u8, sz : uint) -> *mut u8	{
	unsafe	{
		return kheap_realloc(ptr, sz);
	}
}

// Should allocate more if there is no more memory left
#[no_mangle] pub extern "C" fn malloc(len : uint) -> *mut u8	{
	unsafe	{
		return kheap_malloc_internal(len);
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

// Test to check that reallocation is working ok. Same as above, not a very
// thorough test.
pub fn realloc_test(len : uint) -> bool {
	let mut res : int = 0;
	let mut ret : bool = true;
	extra::range(0, len, |i|	{
		res = 0;
		let mut tmp : Vec<int> = Vec::new();
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
