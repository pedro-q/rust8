// This is a comment, and is ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function
use std::fs::File;
use std::io::Read;

fn read_rom() -> std::io::Result<Vec<u8>> {
    let mut file = File::open("pong.ch8")?;

    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    return Ok(data);
}

fn main() {
	// Statements here are executed when the compiled binary is called
	// Initialize system
	let mut system = System {
		opcode: 0,
		pc: 0,
		v: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
		memory: [0; 4096],
		i: 0,
		delay_timer: 0,
		sound_timer: 0,
		stack: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
		sp: 0
	};

	let rom = match read_rom() {
		Ok(value) => value,
		Err(_error) => Vec::new() // TODO: Not let this silently fail!
	};
	
	// Fetch opcode
	system.opcode = 162 << 8 | 240;
	println!("{:x}", system.opcode);
}

struct System {
	opcode: u16,
	pc: u8,
	memory: [u8; 4096],
	v: [u8; 16],
	i: u8,
	//gfx: [[bool;64]; 32],
	delay_timer: u8,
	sound_timer: u8,
	stack: [u8; 16],
	sp: u8
}
