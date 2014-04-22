
use super::core::vec::Vec;
use super::core::container::Container;

// Needed, but not detected as needed by the compiler
use super::core::option::{Some, None};
use extra::range;

use screen;

// All the keys pressed in this expression
static mut keys_pressed : [u8, ..256] = [0, ..256];
static mut len_keys_pressed : uint = 0;

// Constants stored throughout the program
static mut letters : [f32, ..27] = [0.0, ..27];

// Each part of the expression is either an operator or a number, if we have an
// error the expression is invalid.
enum Oper	{
	Number(f32),
	Operator(u8),
	Error
}

// Associativity of operators
enum Associativity	{
	Left,
	Right,
	No
}

fn power_of(base : f32, exp : i32) -> f32	{
	let mut ret = base;
	if exp > 0	{
		range(0, exp as uint - 1, |_i|	{
			ret *= base;
		});
	}
	else if exp < 0	{
		ret = 1.0;
		range(0, (-exp) as uint, |_i|	{
			ret /= base;
		});
	}
	else	{
		return 1.0;
	}
	return ret;
}


// Add one key to the expression
pub unsafe fn add_key(key : u8)	{
	if len_keys_pressed == 0	{
		// If this is the first key pressed, we need to clear the screen
		screen::clear_screen(0);
		screen::write_string("\n\t\t", 0);
	}
	
	// Enter means evaluate the expression
	if key != '\n' as u8	{
		if len_keys_pressed < 256	{
			screen::putc(key, 0);
			// If it is a backspace, we should delete a keypress
			if key == 0x08	{
				if len_keys_pressed > 0	{
					len_keys_pressed -= 1;
				}
			} else	{
				keys_pressed[len_keys_pressed] = key;
				len_keys_pressed += 1;
			}
		}
	}
	else	{
		// If enter is pressed we should get the result and reset our expression
		get_result();
		len_keys_pressed = 0;
	}
}

// Get the result of the globally stored expression
pub unsafe fn get_result()	{
	// Parse the expression and get where to place the expression or -1 in
	// num_letter
	// Expression is parsed into a postfix stack
	let (mut res, num_letter) = parse_expression(keys_pressed, len_keys_pressed);
	if num_letter == -1	{
		screen::write_string(" = ERROR", 0);
		return;
	}
	// Stack with all the numbers as they are being evaluated, cannot be longer
	// than the length of the result we got
	let mut nums : Vec<f32> = Vec::with_capacity(res.len());

	// If it's empty, loop will not happen and we must set it to false, otherwise
	// it is valid by default.
	let mut valid : bool = res.len() != 0;

	while res.len() > 0	{
		// We know that we can pop at least 1 element
		let tmp = res.pop().get();

		match tmp	{
			Operator(o) =>	{
				// Try to get two numbers from the stack
				let mut tmp = nums.pop();
				match tmp	{
					None => {valid = false;	break;},
					_ => {}
				}
				let num1 = tmp.get();
				
				tmp = nums.pop();
				match tmp	{
					None => {valid = false;	break;},
					_ => {}
				}
				let num2 = tmp.get();
				
				match o as u8 as char	{
					'+' => nums.push(num2 + num1),
					'/' => {
						if num1 == 0 as f32 {
							valid = false; break;
						} else	{
							nums.push(num2 / num1)
						}
					},
					'-' => nums.push(num2 - num1),
					'*' => nums.push(num2 * num1),
					
					// For modulus, we just convert it to integers
					'%' => {
						if num1 as int == 0	{
							valid = false;	break;
						} else	{
							nums.push((num2 as int % num1 as int) as f32)
						}
					},
					'^' => nums.push(power_of(num2, num1 as i32)),
					_   => {valid = false; break;},
				}
			},
			Number(n) => nums.push(n),
			Error => {valid = false; break;}
		}
	}

	if valid	{
		// The expression is valid and the last number in the stack contains the
		// result
		let result : f32 = nums.pop().get();

		// Store the result in our array
		letters[num_letter] = result;

		// Write result in input box
		screen::write_string(" = ", 0);
		screen::write_float(result, 0);
		screen::putc('\n' as u8, 0);

		// Write saved variable
		screen::clear_screen(num_letter as u8 + 5);
		if num_letter == 26	{
			screen::write_string("ANS", num_letter as u8 + 5);
		} else	{
			screen::putc(num_letter as u8 + 'A' as u8, num_letter as u8 + 5);
		}
		screen::write_string(" = ", num_letter as u8 + 5);
		screen::write_float(result, num_letter as u8 + 5);
	}
	else	{
		// Write error if result is invalid
		screen::write_string(" = ERROR", 0);
	}
}

// Get the precedense of one operator
fn get_prec(op : u8) -> int	{
	match op as char	{
		'+' => 1,
		'-' => 1,
		'*' => 2,
		'%' => 2,
		'/' => 2,
		'^' => 3,
		'(' => 4,
		')' => 4,
		_ => 0,
	}
}

// Get associativity of the operator, for now, it is always Left
fn get_associativity(op : u8) -> Associativity	{
	match op as char	{
		'+' => Left,
		'-' => Left,
		'*' => Left,
		'/' => Left,
		'%' => Left,
		'^' => Left,
		'(' => Left,
		')' => Left,
		_ => No,
	}
}

// Check if character is an operator
fn is_oper(c : char) -> bool	{
	c == '+' || c == '*' || c == '%' || c == '-' || c == '/' || c == '^'
}

// If character is a valid letter that should be replaced by a number
fn is_letter(c : u8) -> bool	{
	c >= 'A' as u8 && c <= 'Z' as u8
}


// Parses the expression and creates a postfix stack of the expression
// the integer returned represent where the final result should be placed or -1
// if there is an error in the expression.
fn parse_expression(s : &[u8], len : uint) -> (Vec<Oper>, int)	{
	// Placeholders for numbers we find
	let mut num : f32 = 0.0;

	// We have found and calculated the value of a number
	let mut num_used : bool = false;

	// Used for parsing floating point numbers
	let mut dot_used : uint = 0;

	// Says that the next should be a number or "("
	let mut last_op : bool = true;

	// If the next number we find should be negative
	let mut next_negative : bool = false;

	// Default is to place result in ANS
	let mut letter_answer : int = 26;

	// Temporary stack with operators
	let mut tmp_stack : Vec<u8> = Vec::new();

	// Return (output) stack in reverse order.
	let mut ret_stack : Vec<Oper> = Vec::new();

	// Where we are in the expression
	let mut i : uint = 0;
	
	while i < len	{
		if (s[i] >= '0' as u8 && s[i] <= '9' as u8) || (num_used == true &&
		s[i] == '.' as u8 && dot_used == 0)	{
			if s[i] == '.' as u8	{
				dot_used = 10;
			}
			else if dot_used > 0	{
				num += ((s[i] as u8 - '0' as u8) as f32) / dot_used as f32;
				dot_used *= 10;
			}
			else	{
				num *= 10.0;
				num += (s[i] as u8 - '0' as u8) as f32;
			}
			num_used = true;
			i += 1;
			continue;
		}
		if num_used == true	{
			// Numbers are only allowed at the beginning or after an operator
			if last_op == false	{
				return (ret_stack, -1);				
			}
			if next_negative == true	{
				ret_stack.push(Number(-num));
				next_negative = false;
			}
			else	{
				ret_stack.push(Number(num));
			}
			num = 0.0;
			dot_used = 0;
			num_used = false;
			last_op = false;
		}

		if s[i] == '(' as u8	{
			// Should only have '(' in the beginning or after an operator
			if last_op == false	{
				return (ret_stack, -1);
			}
			tmp_stack.push(s[i]);
		}
		else if s[i] == ')' as u8	{
			// This must be preceeded by a number
			if last_op == true	{
				return (ret_stack, -1);
			}
			last_op = false;
			let mut tmp : u8 = tmp_stack.pop().get();
			while tmp != '(' as u8	{
				ret_stack.push(Operator(tmp));
				tmp = tmp_stack.pop().get();
			}
		}
		else if is_letter(s[i])	{
			// Letter that should be replaced by a number
			let mut j = i+1;
			let mut ans_used = false;
			
			// Check if the "letter" is "ANS"
			if i+2 < len && s[i+1] == 'N' as u8 && s[i+2] == 'S' as u8	{
				j = i+3;
				ans_used = true;
			}
			// Ignore any spaces
			while j < len && s[j] == ' ' as u8	{
				j += 1;
			}

			// Check if this is where the result value goes or if the letter
			// should be replaced in the expression.
			if s[j] == '=' as u8	{
				if ans_used == false	{
					letter_answer = (s[i] as u8 - 'A' as u8) as int;
					j += 1;	// One extra character to ignore
				}
			}
			else	{
				if ans_used == true	{
					unsafe {num = letters[26]; }
				} else	{
					unsafe {num = letters[s[i] as u8 - 'A' as u8];}
				}
				num_used = true;
			}
			// Increment i and go back to beginning of loop
			i = j;
			continue;
		}
		else if is_oper(s[i] as char)	{
			if last_op == true	{
				if s[i] == '-' as u8 && s[i+1] >= '0' as u8 && s[i+1] <= '9' as u8	{
					next_negative = true;
				}
				else	{
					return (ret_stack, -1);
				}
			}
			else if tmp_stack.len() == 0	{
				tmp_stack.push(s[i]);
				last_op = true;
			}
			else	{
				last_op = true;
				let mut tmp : u8 = tmp_stack.pop().get();
				while tmp_stack.len() > 0 && is_oper(tmp as char) && (get_prec(s[i]) < get_prec(tmp) ||
					(get_prec(s[i]) == get_prec(tmp) && get_associativity(s[i])
					as int == Left as int ) )
				{
					ret_stack.push(Operator(tmp));
					tmp = tmp_stack.pop().get();
				}
				if is_oper(tmp as char) && get_prec(s[i]) <= get_prec(tmp)	{
					ret_stack.push(Operator(tmp));
				}
				else	{
					tmp_stack.push(tmp);
				}
				tmp_stack.push(s[i]);
			}
		}
		else if s[i] != ' ' as u8	{
			// All other characters are invalid
			return (ret_stack, -1);
		}
		i += 1;
	}

	// Need to push the last number
	if num_used == true	{
		if next_negative == true	{
			ret_stack.push(Number(-num));
		} else	{
			ret_stack.push(Number(num));
		}
	}
	// Pop off the rest of the operators
	while tmp_stack.len() > 0	{
		ret_stack.push(Operator(tmp_stack.pop().get() as u8));
	}

	// Need to reverse the stack
	let mut ret : Vec<Oper> = Vec::with_capacity(ret_stack.len());
	while ret_stack.len() > 0	{
		let t = ret_stack.pop().get();
		ret.push(t);

	}
	return (ret, letter_answer);
}

