use crate::mmu as mmu;
use crate::cpu_tables as cpu_tables;

macro_rules! ternary {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {$v} else {$v1}
    };
}

fn ld_word(r1: &mut u8, r2: &mut u8, nn : u16) {
	//*reg = nn;
	*r1 = ((nn & 0b11110000) >> 4) as u8;
	*r2 = (nn & 0b00001111) as u8;
}

fn xor(reg: &mut u8, n : u8, flags : &mut u8) {
	*reg = n ^ (*reg);
	//*flags = ((ternary!(*reg == 0, 0, 1) as u8) << 7) & 15;
}

fn and(reg : &mut u8, n : u8, flags: &mut u8) {
	*reg = n & (*reg);
	*flags = ternary!(*reg == 0, 0b10100000, 0b00100000) | (*flags & 0x0F);
}


fn test_bit(reg: u8, n : u8, flags : &mut u8) {
	let res : u8 = ternary!(((reg) & (1u8 << n)) == 0, 0b10100000, 0b00100000);
	*flags = res | (*flags & 0b00011111);
}

fn swap(reg : u8, flags : &mut u8) -> u8 {
	let res = ((reg & 0x0F) << 4 ) | ((reg & 0xF0) >> 4);
	*flags = ternary!(res == 0, 0b10000000, 0b00000000) | (*flags & 0x0F);
	return res;
}

/*
// this instruction is weird
fn sra(reg : &mut u8, flags : &mut u8) {
}
*/

// rotate register left by one bit
fn rlc(reg : u8, flags : &mut u8) -> u8 {
	let ret = (reg << 1) | ((reg & 0b10000000) >> 7);
	*flags = ternary!(ret == 0, 0b10000000, 0b00000000) | ((ret & 0b00000001) << 4);
	return ret;
}

// rotate register right by one bit
fn rrc(reg : u8, flags : &mut u8) -> u8 {
	let ret = (reg >> 1) | ((reg & 0b00000001) << 7);
	*flags = ternary!(ret == 0, 0b10000000, 0b00000000) | ((ret & 0b10000000) >> 3);
	return ret;
}


// shift register left by one bit
fn sla(reg : u8, flags : &mut u8) -> u8 {
	let saved_bit = (reg & 0b10000000) >> 3;	
	let ret = (reg << 1);
	*flags = ternary!(ret == 0, 0b10000000, 0b00000000) | saved_bit;
	return ret;
}

// shift register right by one bit
fn sra(reg : u8, flags : &mut u8) -> u8 {
	let saved_bit = (reg & 0b00000001) << 4;
	let ret = (reg >> 1) | (reg & 0b10000000);
	*flags = ternary!(ret == 0, 0b10000000, 0b00000000) | saved_bit;
	return ret;
}



fn complement_reg(reg : &mut u8, flags : &mut u8) {
	*reg = !*reg;
	*flags = 0b01100000;
}









#[derive(Copy, Clone)]
pub struct CPU {
	// NOTE on the Flags register 'f':
	/*
	Bit 	Name 	Explanation
	7 	z 	Zero flag
	6 	n 	Subtraction flag (BCD)
	5 	h 	Half Carry flag (BCD)
	4 	c 	Carry flag
	... the other bits are ALWAYS 0
	*/
	a : u8,
	f : u8,

	b : u8,
	c : u8,

	d : u8,
	e : u8,
	
	h : u8,
	l : u8,
	
    mmu_ref : mmu::MMU,

	// yes, we split the stack pointer in half.
	// it makes defining general cpu functions easier
	//sp : u16,
	s : u8,
	p : u8,
	pc : u16,



	// flags


	//Halt CPU & LCD display until button pressed.
	stop_flag : bool,
	
	 //Power down CPU until an interrupt occurs. Use this  when ever possible to reduce energy consumption
	halt_flag : bool,
}


impl CPU {

	pub fn new() -> CPU {
		CPU {
			a : 0, f : 0,
			b : 0, c : 0,
			d : 0, e : 0,
			h : 0, l : 0,
			mmu_ref : mmu::MMU::new(),
			s : 0, p : 0,
			pc : 0,
			stop_flag : false,
			halt_flag : false,
		}
	}

	fn reg_val(self, reg : &u8) -> u8 {
		return *reg;
	}


	fn get_a(self) -> u8 {
		return self.a;
	}

	// swaps the order of bytes in a word to account for little-endian
	fn l_e_word_conversion(self, nn : u16) -> u16 {
		return ((nn & 0x00FF) << 8) | ((nn & 0xFF00) >> 8);
	}

	fn get_af(self) -> u16 {
		return ((self.a as u16) << 8) | (self.f as u16);
	}
	

	fn get_bc(self) -> u16 {
		return ((self.b as u16) << 8) | (self.c as u16);
	}


	fn get_de(self) -> u16 {
		return ((self.d as u16) << 8) | (self.e as u16);
	}

	fn get_hl(self) -> u16 {
		return ((self.h as u16) << 8) | (self.l as u16);
	}


	fn get_sp(self) -> u16 {
		return ((self.s as u16) << 8) | (self.p as u16);
	}

	fn set_af(&mut self, val : u16) {
		self.a = ((val & 0xFF00) >> 8) as u8;
		self.f = (val & 0x00FF) as u8;
	}

	fn set_bc(&mut self, val : u16) {
		self.b = ((val & 0xFF00) >> 8) as u8;
		self.c = (val & 0x00FF) as u8;
	}

	fn set_de(&mut self, val : u16) {
		self.d = ((val & 0xFF00) >> 8) as u8;
		self.e = (val & 0x00FF) as u8;
	}

	// TODO shift first register to the right
	fn set_hl(&mut self, val : u16) {
		self.h = ((val & 0xFF00) >> 8) as u8;
		self.l = (val & 0x00FF) as u8;
	}

	fn set_sp(&mut self, val : u16) {
		self.s = ((val & 0xFF00) >> 8) as u8;
		self.p = (val & 0x00FF) as u8;
	}


    // performs a F/D/E/WB cycle
    pub fn cycle(&mut self) {
		println!("Cycle");
        let next_opcode : u8 = self.fetch(self.pc);
		println!("fetched");
        self.decode_execute(next_opcode);
    }


	// fetches the next byte in memory
	fn fetch(self, addr : u16) -> u8 {
		let next_byte : u8 = self.mmu_ref.get_byte(addr as usize);
		return next_byte;
	}

	fn next_word(self, addr : u16) -> u16 {
		println!("next word");
		let b1 = (self.fetch(addr) as u16) << 8;
		println!("next word 2");
		//let b2 = self.fetch(addr+1) as u16;
		let b2 = 0;
		println!("next word 3");
		return b1 | b2;
	}



	fn decode_execute(&mut self, mut opcode : u8) {
	
		// check for 0xCB prefix
		let cb_prefix : bool = (opcode == 0xCB);
		if cb_prefix {
			self.pc += 1;
			opcode = self.fetch(self.pc);
		}
		let i1 = ((opcode & 0xF0) >> 4) as usize;
		let i2 = (opcode & 0x0F) as usize;
		let cycles : u8 = ternary!(cb_prefix, cpu_tables::cb_prefixed_cycle_times[i1][i2], cpu_tables::cycle_times[i1][i2]);
		let	instruction_size : u8 = ternary!(cb_prefix, 2, cpu_tables::instruction_sizes[i1][i2]);
	
		// pre-emptive execution to save space below
		println!("Test");
		let nn = self.next_word(self.pc+1);
		let n = self.fetch(self.pc+1);

		if cb_prefix {
			// decrement to 'back up' once
			self.pc = self.pc - 1;
			match opcode {

				0x00 => self.b = rlc(self.b, &mut self.f),
				0x01 => self.c = rlc(self.c, &mut self.f),
				0x02 => self.d = rlc(self.d, &mut self.f),
				0x03 => self.e = rlc(self.e, &mut self.f),
				0x04 => self.h = rlc(self.h, &mut self.f),
				0x05 => self.l = rlc(self.l, &mut self.f),
				0x06 => self.mmu_ref.set_byte(self.get_hl() as usize, rlc(self.mmu_ref.get_byte(self.get_hl() as usize), &mut self.f)),
				0x07 => self.a = rlc(self.a, &mut self.f),

				0x08 => self.b = rrc(self.b, &mut self.f),
				0x09 => self.c = rrc(self.c, &mut self.f),
				0x0A => self.d = rrc(self.d, &mut self.f),
				0x0B => self.e = rrc(self.e, &mut self.f),
				0x0C => self.h = rrc(self.h, &mut self.f),
				0x0D => self.l = rrc(self.l, &mut self.f),
				0x0E => self.mmu_ref.set_byte(self.get_hl() as usize, rrc(self.mmu_ref.get_byte(self.get_hl() as usize), &mut self.f)),
				0x0F => self.a = rrc(self.a, &mut self.f),

				0x20 => self.b = sla(self.b, &mut self.f),
				0x21 => self.c = sla(self.c, &mut self.f),
				0x22 => self.d = sla(self.d, &mut self.f),
				0x23 => self.e = sla(self.e, &mut self.f),
				0x24 => self.h = sla(self.h, &mut self.f),
				0x25 => self.l = sla(self.l, &mut self.f),
				0x26 => self.mmu_ref.set_byte(self.get_hl() as usize, sla(self.mmu_ref.get_byte(self.get_hl() as usize), &mut self.f)),
				0x27 => self.a = sla(self.a, &mut self.f),

				0x28 => self.b = sra(self.b, &mut self.f),
				0x29 => self.c = sra(self.c, &mut self.f),
				0x2A => self.d = sra(self.d, &mut self.f),
				0x2B => self.e = sra(self.e, &mut self.f),
				0x2C => self.h = sra(self.h, &mut self.f),
				0x2D => self.l = sra(self.l, &mut self.f),
				0x2E => self.mmu_ref.set_byte(self.get_hl() as usize, sra(self.mmu_ref.get_byte(self.get_hl() as usize), &mut self.f)),
				0x2F => self.a = sra(self.a, &mut self.f),
				0x30 => self.b = swap(self.b, &mut self.f),
				0x31 => self.c = swap(self.c, &mut self.f),
				0x32 => self.d = swap(self.d, &mut self.f),
				0x33 => self.e = swap(self.e, &mut self.f),
				0x34 => self.h = swap(self.h, &mut self.f),
				0x35 => self.l = swap(self.l, &mut self.f),
				0x36 => self.mmu_ref.set_byte(self.get_hl() as usize, swap(self.mmu_ref.get_byte(self.get_hl() as usize), &mut self.f)),
				0x37 => self.a = swap(self.a, &mut self.f),



				// test bit n of register
				0x40 => test_bit(self.b, 0, &mut self.f),
				0x41 => test_bit(self.c, 0, &mut self.f),
				0x42 => test_bit(self.d, 0, &mut self.f),
				0x43 => test_bit(self.e, 0, &mut self.f),
				0x44 => test_bit(self.h, 0, &mut self.f),
				0x45 => test_bit(self.l, 0, &mut self.f),
				0x46 => test_bit(self.mmu_ref.get_byte(self.get_hl() as usize), 0, &mut self.f),
				0x47 => test_bit(self.a, 0, &mut self.f),

				0x48 => test_bit(self.b, 1, &mut self.f),
				0x49 => test_bit(self.c, 1, &mut self.f),
				0x4A => test_bit(self.d, 1, &mut self.f),
				0x4B => test_bit(self.e, 1, &mut self.f),
				0x4C => test_bit(self.h, 1, &mut self.f),
				0x4D => test_bit(self.l, 1, &mut self.f),
				0x4E => test_bit(self.mmu_ref.get_byte(self.get_hl() as usize), 1, &mut self.f),
				0x4F => test_bit(self.a, 1, &mut self.f),

				0x50 => test_bit(self.b, 2, &mut self.f),
				0x51 => test_bit(self.c, 2, &mut self.f),
				0x52 => test_bit(self.d, 2, &mut self.f),
				0x53 => test_bit(self.e, 2, &mut self.f),
				0x54 => test_bit(self.h, 2, &mut self.f),
				0x55 => test_bit(self.l, 2, &mut self.f),
				0x56 => test_bit(self.mmu_ref.get_byte(self.get_hl() as usize), 2, &mut self.f),
				0x57 => test_bit(self.a, 2, &mut self.f),

				0x58 => test_bit(self.b, 3, &mut self.f),
				0x59 => test_bit(self.c, 3, &mut self.f),
				0x5A => test_bit(self.d, 3, &mut self.f),
				0x5B => test_bit(self.e, 3, &mut self.f),
				0x5C => test_bit(self.h, 3, &mut self.f),
				0x5D => test_bit(self.l, 3, &mut self.f),
				0x5E => test_bit(self.mmu_ref.get_byte(self.get_hl() as usize), 3, &mut self.f),
				0x5F => test_bit(self.a, 3, &mut self.f),

				0x60 => test_bit(self.b, 4, &mut self.f),
				0x61 => test_bit(self.c, 4, &mut self.f),
				0x62 => test_bit(self.d, 4, &mut self.f),
				0x63 => test_bit(self.e, 4, &mut self.f),
				0x64 => test_bit(self.h, 4, &mut self.f),
				0x65 => test_bit(self.l, 4, &mut self.f),
				0x66 => test_bit(self.mmu_ref.get_byte(self.get_hl() as usize), 4, &mut self.f),
				0x67 => test_bit(self.a, 4, &mut self.f),

				0x68 => test_bit(self.b, 5, &mut self.f),
				0x69 => test_bit(self.c, 5, &mut self.f),
				0x6A => test_bit(self.d, 5, &mut self.f),
				0x6B => test_bit(self.e, 5, &mut self.f),
				0x6C => test_bit(self.h, 5, &mut self.f),
				0x6D => test_bit(self.l, 5, &mut self.f),
				0x6E => test_bit(self.mmu_ref.get_byte(self.get_hl() as usize), 5, &mut self.f),
				0x6F => test_bit(self.a, 5, &mut self.f),

				0x70 => test_bit(self.b, 6, &mut self.f),
				0x71 => test_bit(self.c, 6, &mut self.f),
				0x72 => test_bit(self.d, 6, &mut self.f),
				0x73 => test_bit(self.e, 6, &mut self.f),
				0x74 => test_bit(self.h, 6, &mut self.f),
				0x75 => test_bit(self.l, 6, &mut self.f),
				0x76 => test_bit(self.mmu_ref.get_byte(self.get_hl() as usize), 6, &mut self.f),
				0x77 => test_bit(self.a, 6, &mut self.f),

				0x78 => test_bit(self.b, 7, &mut self.f),
				0x79 => test_bit(self.c, 7, &mut self.f),
				0x7A => test_bit(self.d, 7, &mut self.f),
				0x7B => test_bit(self.e, 7, &mut self.f),
				0x7C => test_bit(self.h, 7, &mut self.f),
				0x7D => test_bit(self.l, 7, &mut self.f),
				0x7E => test_bit(self.mmu_ref.get_byte(self.get_hl() as usize), 7, &mut self.f),
				0x7F => test_bit(self.a, 7, &mut self.f),

				// reset bit n of register
				0x80 => self.b = (self.b & !(1 << n)),
				0x81 => self.c = (self.c & !(1 << n)),
				0x82 => self.d = (self.d & !(1 << n)),
				0x83 => self.e = (self.e & !(1 << n)),
				0x84 => self.h = (self.h & !(1 << n)),
				0x85 => self.l = (self.l & !(1 << n)),
				0x86 => self.mmu_ref.set_byte(self.get_hl() as usize, (self.mmu_ref.get_byte(self.get_hl() as usize) & !(1 << n))),
				0x87 => self.a = (self.a & !(1 << n)),

				0x90 => self.b = (self.b & !(1 << 2)),
				0x91 => self.c = (self.c & !(1 << 2)),
				0x92 => self.d = (self.d & !(1 << 2)),
				0x93 => self.e = (self.e & !(1 << 2)),
				0x94 => self.h = (self.h & !(1 << 2)),
				0x95 => self.l = (self.l & !(1 << 2)),
				0x96 => self.mmu_ref.set_byte(self.get_hl() as usize, (self.mmu_ref.get_byte(self.get_hl() as usize) & !(1 << 2))),

				0x97 => self.a = (self.a & !(1 << 2)),
				0x98 => self.b = (self.b & !(1 << 3)),
				0x99 => self.c = (self.c & !(1 << 3)),
				0x9A => self.d = (self.d & !(1 << 3)),
				0x9B => self.e = (self.e & !(1 << 3)),
				0x9C => self.h = (self.h & !(1 << 3)),
				0x9D => self.l = (self.l & !(1 << 3)),
				0x9E => self.mmu_ref.set_byte(self.get_hl() as usize, (self.mmu_ref.get_byte(self.get_hl() as usize) & !(1 << 3))),

				0x9F => self.a = (self.a & !(1 << 3)),

				0xA0 => self.b = (self.b & !(1 << 4)),
				0xA1 => self.c = (self.c & !(1 << 4)),
				0xA2 => self.d = (self.d & !(1 << 4)),
				0xA3 => self.e = (self.e & !(1 << 4)),
				0xA4 => self.h = (self.h & !(1 << 4)),
				0xA5 => self.l = (self.l & !(1 << 4)),
				0xA6 => self.mmu_ref.set_byte(self.get_hl() as usize, (self.mmu_ref.get_byte(self.get_hl() as usize) & !(1 << 4))),

				0xA7 => self.a = (self.a & !(1 << 4)),
				0xA8 => self.b = (self.b & !(1 << 5)),
				0xA9 => self.c = (self.c & !(1 << 5)),
				0xAA => self.d = (self.d & !(1 << 5)),
				0xAB => self.e = (self.e & !(1 << 5)),
				0xAC => self.h = (self.h & !(1 << 5)),
				0xAD => self.l = (self.l & !(1 << 5)),
				0xAE => self.mmu_ref.set_byte(self.get_hl() as usize    , (self.mmu_ref.get_byte(self.get_hl() as usize) & !(1 << 5))),
				0xAF => self.a = (self.a & !(1 << 5)),

				0xB0 => self.b = (self.b & !(1 << 6)),
				0xB1 => self.c = (self.c & !(1 << 6)),
				0xB2 => self.d = (self.d & !(1 << 6)),
				0xB3 => self.e = (self.e & !(1 << 6)),
				0xB4 => self.h = (self.h & !(1 << 6)),
				0xB5 => self.l = (self.l & !(1 << 6)),
				0xB6 => self.mmu_ref.set_byte(self.get_hl() as usize    , (self.mmu_ref.get_byte(self.get_hl() as usize) & !(1 << 6))),
				0xB7 => self.a = (self.a & !(1 << 6)),

				0xB8 => self.b = (self.b & !(1 << 7)),
				0xB9 => self.c = (self.c & !(1 << 7)),
				0xBA => self.d = (self.d & !(1 << 7)),
				0xBB => self.e = (self.e & !(1 << 7)),
				0xBC => self.h = (self.h & !(1 << 7)),
				0xBD => self.l = (self.l & !(1 << 7)),
				0xBE => self.mmu_ref.set_byte(self.get_hl() as usize    , (self.mmu_ref.get_byte(self.get_hl() as usize) & !(1 << 7))),
				0xBF => self.a = (self.a & !(1 << 7)),

				0xC0 => self.b = (self.b | (1 << 0)),
				0xC1 => self.c = (self.c | (1 << 0)),
				0xC2 => self.d = (self.d | (1 << 0)),
				0xC3 => self.e = (self.e | (1 << 0)),
				0xC4 => self.h = (self.f | (1 << 0)),
				0xC5 => self.l = (self.l | (1 << 0)),
				0xC6 => self.mmu_ref.set_byte(self.get_hl() as usize, (self.mmu_ref.get_byte(self.get_hl() as usize) | (1 << 0))),
				0xC7 => self.a = (self.a | (1 << 0)),
				0xC8 => self.b = (self.b | (1 << 1)),
				0xC9 => self.c = (self.c | (1 << 1)),
				0xCA => self.d = (self.d | (1 << 1)),
				0xCB => self.e = (self.e | (1 << 1)),
				0xCC => self.h = (self.h | (1 << 1)),
				0xCD => self.l = (self.l | (1 << 1)),
				0xCE => self.mmu_ref.set_byte(self.get_hl() as usize, (self.mmu_ref.get_byte(self.get_hl() as usize) | (1 << 1))),
				0xCF => self.a = (self.a | (1 << 1)),
				0xD0 => self.b = (self.b | (1 << 2)),
				0xD1 => self.c = (self.c | (1 << 2)),
				0xD2 => self.d = (self.d | (1 << 2)),
				0xD3 => self.e = (self.e | (1 << 2)),
				0xD4 => self.h = (self.f | (1 << 2)),
				0xD5 => self.l = (self.l | (1 << 2)),
				0xD6 => self.mmu_ref.set_byte(self.get_hl() as usize, (self.mmu_ref.get_byte(self.get_hl() as usize) | (1 << 2))),
				0xD7 => self.a = (self.a | (1 << 2)),
				0xD8 => self.b = (self.b | (1 << 3)),
				0xD9 => self.c = (self.c | (1 << 3)),
				0xDA => self.d = (self.d | (1 << 3)),
				0xDB => self.e = (self.e | (1 << 3)),
				0xDC => self.h = (self.h | (1 << 3)),
				0xDD => self.l = (self.l | (1 << 3)),
				0xDE => self.mmu_ref.set_byte(self.get_hl() as usize, (self.mmu_ref.get_byte(self.get_hl() as usize) | (1 << 3))),
				0xDF => self.a = (self.a | (1 << 3)),
				0xE0 => self.b = (self.b | (1 << 4)),
				0xE1 => self.c = (self.c | (1 << 4)),
				0xE2 => self.d = (self.d | (1 << 4)),
				0xE3 => self.e = (self.e | (1 << 4)),
				0xE4 => self.h = (self.f | (1 << 4)),
				0xE5 => self.l = (self.l | (1 << 4)),
				0xE6 => self.mmu_ref.set_byte(self.get_hl() as usize, (self.mmu_ref.get_byte(self.get_hl() as usize) | (1 << 4))),
				0xE7 => self.a = (self.a | (1 << 4)),
				0xE8 => self.b = (self.b | (1 << 5)),
				0xE9 => self.c = (self.c | (1 << 5)),
				0xEA => self.d = (self.d | (1 << 5)),
				0xEB => self.e = (self.e | (1 << 5)),
				0xEC => self.h = (self.h | (1 << 5)),
				0xED => self.l = (self.l | (1 << 5)),
				0xEE => self.mmu_ref.set_byte(self.get_hl() as usize, (self.mmu_ref.get_byte(self.get_hl() as usize) | (1 << 5))),
				0xEF => self.a = (self.a | (1 << 5)),
				0xF0 => self.b = (self.b | (1 << 6)),
				0xF1 => self.c = (self.c | (1 << 6)),
				0xF2 => self.d = (self.d | (1 << 6)),
				0xF3 => self.e = (self.e | (1 << 6)),
				0xF4 => self.h = (self.f | (1 << 6)),
				0xF5 => self.l = (self.l | (1 << 6)),
				0xF6 => self.mmu_ref.set_byte(self.get_hl() as usize, (self.mmu_ref.get_byte(self.get_hl() as usize) | (1 << 6))),
				0xF7 => self.a = (self.a | (1 << 6)),
				0xF8 => self.b = (self.b | (1 << 7)),
				0xF9 => self.c = (self.c | (1 << 7)),
				0xFA => self.d = (self.d | (1 << 7)),
				0xFB => self.e = (self.e | (1 << 7)),
				0xFC => self.h = (self.h | (1 << 7)),
				0xFD => self.l = (self.l | (1 << 7)),
				0xFE => self.mmu_ref.set_byte(self.get_hl() as usize, (self.mmu_ref.get_byte(self.get_hl() as usize) | (1 << 7))),
				0xFF => self.a = (self.a | (1 << 7)),
				_ => {
					panic!("Error: Invalid opcode!");
				}
			}
		}
		else {
			match opcode {

				0x00 => (),
				0x02 => self.mmu_ref.set_byte(self.get_bc() as usize, self.a),
				0x03 => self.set_bc(self.get_bc()+1),

				0x10 => self.stop_flag = true,
				0x12 => self.mmu_ref.set_byte(self.get_de() as usize, self.a),
				0x13 => self.set_de(self.get_de()+1),
				0x20 => if ((self.f & 128) != 0) {self.pc = nn}, // JMP if non-zero
				0x21 => ld_word(&mut self.h, &mut self.l, nn),

				0x23 => self.set_hl(self.get_hl()+1),
				0x2F => complement_reg(&mut self.a, &mut self.f),

				0x31 => ld_word(&mut self.s, &mut self.p, nn),
				0x32 => self.mmu_ref.set_byte(self.get_hl() as usize, n),
				0x33 => self.set_sp(self.get_sp()+1),


				0x7F => self.a = self.a,
				0x78 => self.a = self.b,
				0x79 => self.a = self.c,
				0x7A => self.a = self.d,
				0x7B => self.a = self.e,
				0x7C => self.a = self.h,
				0x7D => self.a = self.l,
				0x0A => self.a = self.mmu_ref.get_byte(self.get_bc() as usize),
				0x1A => self.a = self.mmu_ref.get_byte(self.get_de() as usize),
				0x7E => self.a = self.mmu_ref.get_byte(self.get_hl() as usize),
				0xFA => self.a = self.mmu_ref.get_byte(self.l_e_word_conversion(nn) as usize),
				0x3E => self.a = n,	


				0x40 => self.b = self.b,
				0x41 => self.b = self.c,
				0x42 => self.b = self.d,
				0x43 => self.b = self.e,
				0x44 => self.b = self.h,
				0x45 => self.b = self.l,
				0x46 => self.b = self.mmu_ref.get_byte(self.get_hl() as usize),
				0x47 => self.b = self.a,
				0x48 => self.c = self.b,
				0x49 => self.c = self.c,
				0x4A => self.c = self.d,
				0x4B => self.c = self.e,
				0x4C => self.c = self.h,
				0x4D => self.c = self.l,
				0x4E => self.c = self.mmu_ref.get_byte(self.get_hl() as usize),
				0x4F => self.c = self.a,

				0x50 => self.d = self.b,
				0x51 => self.d = self.c,
				0x52 => self.d = self.d,
				0x53 => self.d = self.e,
				0x54 => self.d = self.h,
				0x55 => self.d = self.l,
				0x56 => self.d = self.mmu_ref.get_byte(self.get_hl() as usize),
				0x57 => self.d = self.a,
				0x58 => self.e = self.b,
				0x59 => self.e = self.c,
				0x5A => self.e = self.d,
				0x5B => self.e = self.e,
				0x5C => self.e = self.h,
				0x5D => self.e = self.l,
				0x5E => self.e = self.mmu_ref.get_byte(self.get_hl() as usize),
				0x5F => self.e = self.a,

				0x60 => self.h = self.b,
				0x61 => self.h = self.c,
				0x62 => self.h = self.d,
				0x63 => self.h = self.e,
				0x64 => self.h = self.h,
				0x65 => self.h = self.l,
				0x66 => self.h = self.mmu_ref.get_byte(self.get_hl() as usize),
				0x67 => self.h = self.a,
				0x68 => self.l = self.b,
				0x69 => self.l = self.c,
				0x6A => self.l = self.d,
				0x6B => self.l = self.e,
				0x6C => self.l = self.h,
				0x6D => self.l = self.l,
				0x6E => self.l = self.mmu_ref.get_byte(self.get_hl() as usize),
				0x6F => self.l = self.a,

				0x70 => self.mmu_ref.set_byte(self.get_hl() as usize, self.b),
				0x71 => self.mmu_ref.set_byte(self.get_hl() as usize, self.c),
				0x72 => self.mmu_ref.set_byte(self.get_hl() as usize, self.d),
				0x73 => self.mmu_ref.set_byte(self.get_hl() as usize, self.e),
				0x74 => self.mmu_ref.set_byte(self.get_hl() as usize, self.h),
				0x75 => self.mmu_ref.set_byte(self.get_hl() as usize, self.l),
				0x76 => self.halt_flag = true,
				0x77 => self.mmu_ref.set_byte(self.get_hl() as usize, self.a),
				0x78 => self.a = self.b,
				0x79 => self.a = self.c,
				0x7A => self.a = self.d,
				0x7B => self.a = self.e,
				0x7C => self.a = self.h,
				0x7D => self.a = self.l,
				0x7E => self.a = self.mmu_ref.get_byte(self.get_hl() as usize),
				0x7F => self.a = self.a,
				


				0xEA => self.mmu_ref.set_byte(self.l_e_word_conversion(nn) as usize, self.a),


				/*
				//0xA7 => and(&mut self.a, self.reg_val(&self.a), &mut self.f),
				0xA0 => and(&mut self.a, self.b, &mut self.f),
				0xA1 => and(&mut self.a, self.c, &mut self.f),
				0xA2 => and(&mut self.a, self.d, &mut self.f),
				0xA3 => and(&mut self.a, self.e, &mut self.f),
				0xA4 => and(&mut self.a, self.h, &mut self.f),
				0xA5 => and(&mut self.a, self.l, &mut self.f),
				0xA6 => and(&mut self.a, self.mmu_ref.get_byte(self.get_hl() as usize), &mut self.f),
				0xA6 => and(&mut self.a, n, &mut self.f),
				*/

				
				// decrement registers
				0x0B => self.set_bc(self.get_bc()-1),
				0x1B => self.set_de(self.get_de()-1),
				0x2B => self.set_hl(self.get_hl()-1),
				0x3B => self.set_sp(self.get_sp()-1),


				0xAF => xor(&mut self.a, n, &mut self.f),
				0xC2 => self.pc = ternary!((self.f & 0b10000000) == 0, nn, self.pc),
				0xC3 => self.pc = self.l_e_word_conversion(nn) - (instruction_size as u16),
				0xCA => self.pc = ternary!((self.f & 0b10000000) != 0, nn, self.pc),
				0xD2 => self.pc = ternary!((self.f & 0b00010000) == 0, nn, self.pc),
				0xDA => self.pc = ternary!((self.f & 0b00010000) != 0, nn, self.pc),
				//0xE9 => self.pc = self.mmu_ref.get_byte(
				


				






				_ => {
					panic!("Error: Invalid opcode!");
				}
			}
		
		}
		self.pc += instruction_size as u16;
	}
}











