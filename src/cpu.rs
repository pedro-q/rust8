extern crate rand;

use rand::Rng;

pub struct CPU {
    pub opcode: usize,
    pub pc: usize,
    pub memory: [u8; 4096],
    pub v: [usize; 16],
    pub i: usize,
    pub gfx: [u8; 2048], // 64 * 32 pixels
    pub delay_timer: usize,
    pub sound_timer: usize,
    pub stack: [usize; 16],
    pub key: [usize; 16],
    pub sp: usize,
    pub draw: bool,
    pub breakexe: bool
}

impl CPU {

    pub fn fetch_opcode(&mut self) {
        self.opcode = ((self.memory[self.pc] as u16) << 8 |
                                        self.memory[self.pc+1] as u16) as usize;
        self.decode_opcode();
        // There was a stack printer here but now it's gone
    }

    pub fn initialize() -> CPU {
        return CPU {
            opcode: 0,
            pc: 0x200,
            v: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            memory: [0u8; 4096],
            gfx: [0; 2048],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            stack: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            key:[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            sp: 0,
            draw: false,
            breakexe: false
        };
    }

    // Ye olde and simple stack printer
    //pub fn print_state(&self) {
    //  print!("OP: {:x} ", self.opcode);
    //  print!("PC: {:x} ", self.pc);
    //  print!("V: {:x}", self.v[0]);
    //  print!(",{:x}", self.v[1]);
    //  print!(",{:x}", self.v[2]);
    //  print!(",{:x}", self.v[3]);
    //  print!(",{:x}", self.v[4]);
    //  print!(",{:x}", self.v[5]);
    //  print!(",{:x}", self.v[6]);
    //  print!(",{:x}", self.v[7]);
    //  print!(",{:x}", self.v[8]);
    //  print!(",{:x}", self.v[9]);
    //  print!(",{:x}", self.v[10]);
    //  print!(",{:x}", self.v[11]);
    //  print!(",{:x}", self.v[12]);
    //  print!(",{:x}", self.v[13]);
    //  print!(",{:x}", self.v[14]);
    //  print!(",{:x} ", self.v[15]);
    //  print!("I: {:x}", self.i);
    //  print!(" stack: {:?} ", self.stack[0]);
    //  print!("sp: {:x} ", self.sp);
    //  print!("\n");
    //}

    pub fn get_nnn(&self) -> usize {
        return self.opcode & 0x0FFF;
    }

    pub fn get_nn(&self) -> usize {
        return self.opcode & 0x00FF;
    }

    // OPCODES

    pub fn op_00ee(&mut self){
        self.sp -= 1;
        self.pc = self.stack[self.sp];
    }

    pub fn op_00e0(&mut self){
        for y in 0..2048 {
            self.gfx[y] = 0;
        }
        self.draw = true;
        self.pc += 2;
    }

    pub fn op_1nnn(&mut self){
        self.pc = self.get_nnn();
    }

    // This is one of those weird opcodes that everybody
    // implements in different ways, this seems to pass tests
    pub fn op_2nnn(&mut self){
        self.stack[self.sp] = self.pc + 2;
        self.sp += 1;
        self.pc = self.get_nnn();
    }

    pub fn op_3xnn(&mut self, x: usize){
        if self.v[x] == self.get_nn() {
            self.pc += 4
        } else {
            self.pc += 2
        }
    }

    pub fn op_4xnn(&mut self, x: usize){
        if self.v[x] != self.get_nn() {
            self.pc += 4
        } else {
            self.pc += 2
        }
    }

    pub fn op_5xy0(&mut self, x: usize, y: usize){
        if self.v[x] == self.v[y] {
            self.pc += 4
        } else {
            self.pc += 2
        }
    }

    pub fn op_annn(&mut self){
        self.i = self.get_nnn();
        self.pc += 2
    }

    pub fn op_6xnn(&mut self, x: usize){
        self.v[x] = self.get_nn();
        self.pc += 2
    }

    pub fn op_7xnn(&mut self, x: usize){
        let vx = self.v[x] as u16;
        let val = self.get_nn() as u16;
        let result = (vx + val) as u8;
        self.v[x] = result as usize;
        self.pc += 2
    }

    pub fn op_8xy0(&mut self, x: usize, y: usize){
        self.v[x] = self.v[y];
        self.pc += 2
    }

    pub fn op_8xy1(&mut self, x: usize, y: usize){
        self.v[x] |= self.v[y];
        self.pc += 2
    }

    pub fn op_8xy2(&mut self, x: usize, y: usize){
        self.v[x] = self.v[x] & self.v[y];
        self.pc += 2
    }

    pub fn op_8xy3(&mut self, x: usize, y: usize){
        self.v[x] ^= self.v[y];
        self.pc += 2
    }

    pub fn op_8xy4(&mut self, x: usize, y: usize){
        // Let's be careful when we add numbers as
        // they can overflow and give an incorrect results
        // same as 7XNN
        let vx = self.v[x] as u16;
        let vy = self.v[y] as u16;
        let result = (vx + vy) as u8;
        self.v[x] = result as usize;
        if self.v[y] > self.v[x]{
            self.v[0xf] = 1;
        } else {
            self.v[0xf] = 0;
        }
        self.pc += 2
    }

    pub fn op_8xy5(&mut self, x: usize, y: usize){
        if self.v[y] > self.v[x] {
            self.v[0xf] = 0;
        } else {
            self.v[0xf] = 1;
        }
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
        self.pc += 2
    }

    pub fn op_8xy6(&mut self, x: usize){
        self.v[0x0f] = self.v[x] & 1;
        self.v[x] >>=1;
        self.pc += 2
    }

    pub fn op_8xy7(&mut self, x: usize, y: usize){
        if self.v[y] > self.v[x] {
            self.v[0xf] = 1
        } else {
            self.v[0xf] = 0
        }
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
        self.pc += 2
    }

    pub fn op_8x0e(&mut self, x: usize){
        self.v[0x0f] = (self.v[x] & 0b10000000) >> 7;
        self.v[x] <<= 1;
        self.pc += 2
    }

    pub fn op_9xy0(&mut self, x: usize, y: usize){
        if self.v[x] != self.v[y] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    pub fn op_exa1(&mut self, x: usize){
        if self.key[self.v[x]] != 1 {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    pub fn op_cxnn(&mut self, x: usize){
        // Let's be ultra careful with casting types here as they may not
        // give the (random) result we want
        let random_number: u8 = rand::thread_rng().gen();
        self.v[x] = (random_number & (self.get_nn() as u8)) as usize;
        self.pc += 2
    }

    pub fn op_ex9e(&mut self, x: usize){
        if self.key[self.v[x]] == 1 {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    pub fn op_fx0a(&mut self, x: usize) {
        let mut keypress = false;
        for i in 0..16 {
            if self.key[i] == 1 {
                self.v[x] = i;
                keypress = true;
            }
        }
        if keypress == true {
            self.pc += 2
        }
    }

    pub fn op_fx1e(&mut self, x: usize) {
        if self.i + self.v[x] > 0xfff {
            self.v[0xf] = 1
        } else {
            self.v[0xf] = 0
        }
        self.i += self.v[x];
        self.pc += 2
    }

    pub fn op_fx07(&mut self, x: usize) {
        self.v[x] = self.delay_timer;
        self.pc += 2
    }

    pub fn op_fx15(&mut self, x: usize) {
        self.delay_timer = self.v[x];
        self.pc += 2
    }

    pub fn op_fx18(&mut self, x: usize) {
        self.sound_timer = self.v[x];
        self.pc += 2
    }

    pub fn op_fx29(&mut self, x: usize) {
        self.i = self.v[x] * 0x5;
        self.pc += 2
    }

    pub fn op_fx55(&mut self, x: usize) {
        for m in 0..=x {
            self.memory[self.i + m] = self.v[m] as u8;
        }
        // This guide github.com/mattmikolay/chip-8/wiki/CHIP‐8-Instruction-Set
        // says that the I register must be modified but tests won't pass and
        // blinky won't work with this instruction added
        //self.i += x + 1;
        self.pc += 2
    }

    pub fn op_fx65(&mut self, x: usize) {
        for m in 0..=x {
            self.v[m] = (self.memory[self.i + m]) as usize ;
        }
        // This guide github.com/mattmikolay/chip-8/wiki/CHIP‐8-Instruction-Set
        // says that the I register must be modified but tests won't pass and
        // blinky won't work with this instruction added
        //self.i += x + 1;
        self.pc += 2
    }

    // Very weird opcode
    // Decimal representation of a binary number stored in memory
    pub fn op_fx33(&mut self, x: usize) {
        self.memory[self.i] = (self.v[x] / 100) as u8;
        self.memory[self.i + 1] = ((self.v[x] / 10) % 10) as u8;
        self.memory[self.i + 2] = (self.v[x] % 10) as u8;
        self.pc += 2
    }

    // This is the opcode that draws the screen
    pub fn op_dxyn(&mut self, x: usize, y: usize, height: usize){
        // We'll get the coordinates from where we will start drawing from
        // the v registers indicated by the opcode x and y values
        let vx = self.v[x];
        let vy = self.v[y];
        self.v[0xF] = 0;
        // We get the sprite height using the last nibble of our opcode
        for yline in 0..height {
            let pixel = self.memory[self.i + yline];
            // Every sprite is 8 pixels in width
            for xline in 0..8 {
                if pixel & (0x80 >> xline) != 0 {
                    // Our vram is linear we calculate the position from the xy coordinates
                    // Interesting to note: The sprite needs to wrap around the
                    // screen if we need to keep writing a sprite and we reach
                    // the last row or col pixel, if this is not implemented
                    // pong will break when the pong pallet reaches the border
                    // See reddit.com/r/EmuDev/comments/5so1bo/chip8_emu_questions/ddhefiw/
                    let pos = ((vx + xline) % 64) + (((vy + yline) % 32) * 64);

                    // This is some collision detection stuff
                    // In the reddit comment above it's also noted that some
                    // ch8 emulators only raise this flag when the last pixel
                    // collide, it's recommended to combine every pixel collision
                    self.v[0x0F] |= if self.gfx[pos] == 1 { 1 } else { 0 };
                    self.gfx[pos] ^= 1;
                }
            }
        }
        self.draw = true;
        self.pc += 2
    }

    // Just in case we find an unknown opcode
    pub fn unknown(&mut self){
        println!("Unknown instruction {:x}", self.opcode);
        self.breakexe = true
    }

    pub fn decode_opcode(&mut self) {

        let nibbles = (
            (self.opcode & 0xF000) >> 12 as usize,
            (self.opcode & 0x0F00) >> 8 as usize,
            (self.opcode & 0x00F0) >> 4 as usize,
            (self.opcode & 0x000F) as usize,
        );
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;

        match nibbles {
            (0x00, 0x00 , 0x0e, 0x00) => self.op_00e0(),
            (0x00, 0x00 , 0x0e, 0x0e) => self.op_00ee(),
            (0x01, _ , _, _) => self.op_1nnn(),
            (0x02, _ , _, _) => self.op_2nnn(),
            (0x03, _ , _, _) => self.op_3xnn(x),
            (0x04, _ , _, _) => self.op_4xnn(x),
            (0x05, _ , _, 0x00) => self.op_5xy0(x, y),
            (0x06, _ , _, _) => self.op_6xnn(x),
            (0x07, _ , _, _) => self.op_7xnn(x),
            (0x08, _ , _, 0x00) => self.op_8xy0(x,y),
            (0x08, _ , _, 0x01) => self.op_8xy1(x,y),
            (0x08, _ , _, 0x02) => self.op_8xy2(x,y),
            (0x08, _ , _, 0x03) => self.op_8xy3(x,y),
            (0x08, _ , _, 0x04) => self.op_8xy4(x,y),
            (0x08, _ , _, 0x05) => self.op_8xy5(x,y),
            (0x08, _ , _, 0x06) => self.op_8xy6(x), // It's 8XY6 in docs but doesn't use the y value
            (0x08, _ , _, 0x07) => self.op_8xy7(x,y),
            (0x08, _ , _, 0x0e) => self.op_8x0e(x), // Has multiple definitions ¿?
            (0x09, _ , _, 0x00) => self.op_9xy0(x,y),
            (0x0a, _ , _, _) => self.op_annn(),
            (0x0c, _ , _, _) => self.op_cxnn(x),
            (0x0d, _ , _, _) => self.op_dxyn(x,y,n),
            (0x0e, _ , 0x09, 0x0e) => self.op_ex9e(x),
            (0x0e, _ , 0x0a, 0x01) => self.op_exa1(x),
            (0x0f, _ , 0x00, 0x07) => self.op_fx07(x),
            (0x0f, _ , 0x00, 0x0a) => self.op_fx0a(x),
            (0x0f, _ , 0x01, 0x05) => self.op_fx15(x),
            (0x0f, _ , 0x01, 0x08) => self.op_fx18(x),
            (0x0f, _ , 0x01, 0x0e) => self.op_fx1e(x),
            (0x0f, _ , 0x02, 0x09) => self.op_fx29(x),
            (0x0f, _ , 0x03, 0x03) => self.op_fx33(x),
            (0x0f, _ , 0x05, 0x05) => self.op_fx55(x),
            (0x0f, _ , 0x06, 0x05) => self.op_fx65(x),
             _ => self.unknown(),
        }

    }

}
