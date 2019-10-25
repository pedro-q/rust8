
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use std::thread;

// The chip8 fontset
static CHIP8_FONTSET: [u8; 80] =
[
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

fn read_rom() -> std::io::Result<Vec<u8>> {
	let mut file = File::open("pong.ch8")?;

	let mut data = Vec::new();
	file.read_to_end(&mut data)?;

	return Ok(data);
}

fn main() {
	// Initialize the cpu system
	let mut cpu = CPU::initialize();
	
	// Read the ROM
	let rom = match read_rom() {
		Ok(value) => value,
		Err(_error) => Vec::new() // TODO: Not let this silently fail!
	};
	
	// Load the chip8 font
	let mut i = 0;
	for val in CHIP8_FONTSET.iter() {
		cpu.memory[i] = *val;
		i += 1;
	}

	// Load ROM into the memory
	i = 512;
	for val in rom {
		cpu.memory[i] = val;
		i += 1;
	}
	
	// Main loop
	loop {
		// Fetch opcode
		cpu.fetch_opcode();

		if cpu.delay_timer > 0 {
			cpu.delay_timer -= 1;
		}

		if cpu.sound_timer > 0 {
			if cpu.sound_timer == 1 {
				println!("BEEEP");
			}
			cpu.sound_timer -= 1
		}
		
		// Draw the screen
		if cpu.draw {
			for y in 0..32 {
				println!("{:?}", &cpu.gfx[y][..])
			}
			cpu.draw = false;
		}
		
		// Just a flag to break the cycle if an unknown instruction is found
		if cpu.breakexe {
			break
		}
		
		// Let's execute 60 opcodes per second
		thread::sleep(Duration::from_millis(16))
	}
}

struct CPU {
	opcode: u16,
	pc: usize,
	memory: [u8; 4096],
	v: [usize; 16],
	i: usize,
	gfx: [[usize;64]; 32],
	delay_timer: u8,
	sound_timer: u8,
	stack: [u8; 16],
	sp: usize,
	draw: bool,
	breakexe: bool
}

impl CPU {

	fn fetch_opcode(&mut self) {
		self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc+1] as u16);
		self.decode_opcode();
		self.pc += 2
	}

	fn initialize() -> CPU {
		return CPU {
			opcode: 0,
			pc: 0x200,
			v: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
			memory: [0; 4096],
			gfx: [[0; 64]; 32],
			i: 0,
			delay_timer: 0,
			sound_timer: 0,
			stack: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
			sp: 0,
			draw: false,
			breakexe: false
		};
	}

	fn get_nnn(&self) -> usize {
		return (self.opcode & 0x0FFF) as usize;
	}

	fn get_nn(&self) -> usize {
		return (self.opcode & 0x00FF) as usize;
	}

	fn get_kk(&self) -> u8 {
		return (self.opcode & 0x00FF) as u8;
	}
	
	// This is one of those weird opcodes
	fn op_2nnn(&mut self){
		self.stack[self.sp] = (self.pc as u8);
		self.sp += 1;
		self.pc = self.get_nnn()
	}
	
	fn op_annn(&mut self){
		self.i = self.get_nnn();
	}
	
	fn op_6xnn(&mut self, x: usize){
		self.v[x] = self.get_nn();
	}
	
	fn op_fx65(&mut self, x: usize) {
		for m in 0..=x {
			self.v[m] = (self.memory[self.i + (1+m)]) as usize;
		}
	}
	
	fn op_fx33(&mut self, x: usize) {
		self.memory[self.i] = (self.v[x] / 100) as u8;
		self.memory[self.i + 1] = ((self.v[x] / 10) % 10) as u8;
		self.memory[self.i + 2] = ((self.v[x] % 100) % 10) as u8;
	}
	
	// This is the opcode that draws the screen
	fn op_dxyn(&mut self, x: usize, y: usize, height: usize){
		
		// We'll get the coordinates from where we will start drawing from the v registers
		// indicated by the opcode x and y values
		let vx = self.v[x];
		let vy = self.v[y];
		let mut pixel: u8;

		self.v[0xF] = 0;
		// The height of the sprite is determined by the last nibble of our opcode
		for yline in 0..height {
			pixel = self.memory[self.i + yline];
			// Every sprite is 8 pixels in width
			for xline in 0..8 {
				if (pixel & (0x80 >> xline)) != 0 {
					// This conditional is some collision detection stuff
					if self.gfx[vy + yline][vx + xline] == 1 {
						self.v[0xF] = 1;
					}
					self.gfx[vy + yline][vx + xline] ^= 1;
				}
			}
		}
		self.draw = true;
	}

	fn unknown(&mut self){
		println!("Unknown instruction {:x}", self.opcode);
		self.breakexe = true
	}

	fn decode_opcode(&mut self) {
		let nibbles = (
			(self.opcode & 0xF000) >> 12 as u8,
			(self.opcode & 0x0F00) >> 8 as u8,
			(self.opcode & 0x00F0) >> 4 as u8,
			(self.opcode & 0x000F) as u8,
		);
		let x = nibbles.1 as usize;
		let y = nibbles.2 as usize;
		let n = nibbles.3 as usize;

		match nibbles {
			(0x0a, _ , _, _) => self.op_annn(),
			(0x06, _ , _, _) => self.op_6xnn(x),
			(0x0d, _ , _, _) => self.op_dxyn(x,y,n),
			(0x02, _ , _, _) => self.op_2nnn(),
			(0x0f, _ , 0x03, 0x03) => self.op_fx33(x),
			(0x0f, _ , 0x06, 0x05) => self.op_fx65(x),
 			_ => self.unknown(),
		}

	}

}
