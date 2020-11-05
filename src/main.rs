/* Usage
   ./brainfuck filename
*/

use std::io;
use std::str;
use std::fs;
use std::env;

// Standard brainf**k capactiy
const CAPACITY: usize = 30000;

// The main brainf**k memory
pub struct Memory {
	pub pointer: usize,
	pub data: Vec<u16>,
}

impl Memory {
	// Returns a new memory
	pub fn new(capacity: usize) -> Memory {
		Memory {
			pointer: 0,
			data: vec![0; capacity]
		}
	}
}

// Operation enum, each brainfuck operation is parsed into an Operation enum and given to apply function
pub enum Operation {
	IncPointer,
	DecPointer,
	IncValue,
	DecValue,
	Loop(usize, usize), // (start, end)
	Print,
	Scan,
}

// Implementation of different brainfuck operations
impl Operation {
	pub fn inc_pointer(memory: &mut Memory) -> () {
		memory.pointer += 1;
	}

	pub fn dec_pointer(memory: &mut Memory) -> () {
		memory.pointer -= 1;
	}

	pub fn inc_value(memory: &mut Memory) -> () {
		memory.data[memory.pointer] += 1;
	}

	pub fn dec_value(memory: &mut Memory) -> () {
		memory.data[memory.pointer] -= 1;
	}

	pub fn eval_loop(code: &str, memory: &mut Memory, operation: Operation) -> () {
		if let Operation::Loop(a, b) = operation {
			eval(&code[a+1..b], memory);
		}
	}

	pub fn print(memory: &mut Memory) -> () {
		print!("{}", memory.data[memory.pointer] as u8 as char);
	}

	pub fn scan(memory: &mut Memory) -> () {
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Unable to read");
		let raw_data: u16 = input.bytes().nth(0).expect("no byte read") as u16;
		memory.data[memory.pointer] = raw_data;
	}

}

fn main() {

	// Program arguments
	let args: Vec<String> = env::args().collect(); 

	// Read code from a filename provided as a program argument
	if args.len() < 2 {
		panic!("Missing arguments: enter the filename")
	}
	let filename = &args[1];
	let code = fs::read_to_string(filename).expect("Could not read the file.").trim().to_string();

	// Memory::new returns a 0 initialized vector
	let mut memory = Memory::new(CAPACITY);

	eval(&code, &mut memory);
}

// The 'parser', generates Operation enum based on the operation and calls apply
fn eval(code: &str, memory: &mut Memory) -> () {
	let mut i: usize = 0;
	while i < code.len() {

		match &code[i..i+1] {

			">" => {
				let operation = Operation::IncPointer;
				apply(operation, code, memory);
			}

			"<" => {
				let operation = Operation::DecPointer;
				apply(operation, code, memory);
			}

			"+" => {
				let operation = Operation::IncValue;
				apply(operation, code, memory);
			}

			"-" => {
				let operation = Operation::DecValue;
				apply(operation, code, memory);
			}

			"." => {
				let operation = Operation::Print;
				apply(operation, code, memory);
			}

			"," => {
				let operation = Operation::Scan;
				apply(operation, code, memory);
			}

			"[" => {

				let start = i;
				let end = match find_next(code, i) {
					Some(end) => end,
					None => panic!("Invalid code, mismatch brackets.")
				};


				while memory.data[memory.pointer] != 0 {

					// End is non-inclusive just like string indexing
					let operation = Operation::Loop(start, end);

					// Here we give apply the Operation and apply for the loop operation will intern
					// call eval with a smaller code (code inside loop), we keep doing this 
					// until the pointer points to a 0
					// Eval<->Apply loop in place so that we can process loop(s) within loop
					apply(operation, code, memory);
				}

				// Processed the loop, move to the next operation
				i += end - start + 1;
				continue;
			}

			_ => {
				// No match statement for "]" as the code should never reach that character,
				// it is always skipped 
			}
		}
		i += 1;
	}
}

// The 'executioner', executes the Operation
fn apply(operation: Operation, code: &str, memory: &mut Memory) -> () {
	match operation {

		Operation::IncPointer => {
			Operation::inc_pointer(memory);
		},
		Operation::DecPointer => {
			Operation::dec_pointer(memory);
		},
		Operation::IncValue => {
			Operation::inc_value(memory);
		},
		Operation::DecValue => {
			Operation::dec_value(memory)
		},
		// (_a, _b) = (start, end) 
		Operation::Loop(_a, _b) => {
			Operation::eval_loop(code, memory, operation);
		},
		Operation::Print => {
			Operation::print(memory);
		},
		Operation::Scan => {
			Operation::scan(memory);
		},
	}
}

// Find the next matching bracket, i is the opening bracket index
fn find_next(code: &str, i: usize) -> Option<usize> {
	
	// Create a stack for bracket matching
	let mut stack: Vec<usize> = Vec::new();

	// Main loop, the return value in Some is non-inclusive
	for (p, c) in code[i..].chars().enumerate(){
		match c {
			'[' => stack.push(1),
			']' => {
				if stack.len() == 1 {
					return Some(p + i);
				}
				else {
					stack.pop();
				}
			},
			_ => continue,
		}
	}
	return None;
}
