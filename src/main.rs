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
	let cpu = CPU {
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
	cpu.fetch_opcode()
}

struct CPU {
	opcode: u16,
	pc: u8,
	memory: [u8; 4096],
	v: [u8; 16],
	i: usize,
	//gfx: [[bool;64]; 32],
	delay_timer: u8,
	sound_timer: u8,
	stack: [u8; 16],
	sp: u8
}

impl CPU {

	fn fetch_opcode(mut self: Self) {
		self.opcode = 0xa2 << 8 | 0xf0;
		println!("{:x}", self.opcode);
		self.decode_opcode(self.opcode);
	}

	fn get_nnn(&self, opcode: u16) -> usize {
		return (opcode & 0x0FFF) as usize;
	}

	fn get_kk(&self, opcode: u16) {
		(opcode & 0x00FF) as u8;
	}

	fn op_annn(self, opcode: u16){
		let nnn = self.get_nnn(opcode);
		self.i = nnn;
		println!("{:x}", nnn);
		return ();
	}
	
	fn decode_opcode(&self, opcode: u16) {
		let nibbles = (
			(opcode & 0xF000) >> 12 as u8,
			(opcode & 0x0F00) >> 8 as u8,
			(opcode & 0x00F0) >> 4 as u8,
			(opcode & 0x000F) as u8,
		); 
		
		let x = nibbles.1 as usize;
		let y = nibbles.2 as usize;
		let n = nibbles.3 as usize;
		
		match nibbles {
			(A, _ , _, _) => self.op_annn(opcode)
		}
	}

}
