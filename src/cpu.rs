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







#[derive(Copy, Clone)]
pub struct CPU {
	// NOTE on the Flags register 'f':
	/*
	Bit 	Name 	Explanation
	7 	z 	Zero flag
	6 	n 	Subtraction flag (BCD)
	5 	h 	Half Carry flag (BCD)
	4 	c 	Carry flag
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
		println!("Cycle!");
        let next_opcode : u8 = self.fetch();
        self.decode_execute(next_opcode);
    }


	// fetches the next byte in memory
	fn fetch(&mut self) -> u8 {
		let next_byte : u8 = self.mmu_ref.get_byte(self.pc as usize);
		return next_byte;
	}

	fn next_word(&mut self) -> u16 {
		let b1 = (self.fetch() as u16) << 8;
		let b2 = self.fetch() as u16;
		return b1 | b2;
	}



	fn decode_execute(&mut self, mut opcode : u8) {
	
		// check for 0xCB prefix
		let cb_prefix : bool = (opcode == 0xCB);
		if cb_prefix {
			self.pc += 1;
			opcode = self.fetch();
		}
        // TODO: create cycle table later for timing accuracy
		let i1 = ((opcode & 0xF0) >> 4) as usize;
		let i2 = (opcode & 0x0F) as usize;
		let cycles : u8 = ternary!(cb_prefix, cpu_tables::cb_prefixed_cycle_times[i1][i2], cpu_tables::cycle_times[i1][i2]);
		let	instruction_size : u8 = ternary!(cb_prefix, 2, cpu_tables::instruction_sizes[i1][i2]);
	
		// pre-emptive execution to save space below
		let nn = self.next_word();
		let n = self.fetch();

		if cb_prefix {
			match opcode {

				// test bit n of register
				0x47 => test_bit(self.a, n, &mut self.f),
				0x40 => test_bit(self.b, n, &mut self.f),
				0x41 => test_bit(self.c, n, &mut self.f),
				0x42 => test_bit(self.d, n, &mut self.f),
				0x43 => test_bit(self.e, n, &mut self.f),
				0x44 => test_bit(self.h, n, &mut self.f),
				0x45 => test_bit(self.l, n, &mut self.f),
				0x46 => test_bit(self.mmu_ref.get_byte(self.get_hl() as usize), n, &mut self.f),





				
				// reset bit n of register
				0x87 => self.a = (self.a & !(1 << n)),
				0x80 => self.b = (self.b & !(1 << n)),
				0x81 => self.c = (self.c & !(1 << n)),
				0x82 => self.d = (self.d & !(1 << n)),
				0x83 => self.e = (self.e & !(1 << n)),
				0x84 => self.h = (self.h & !(1 << n)),
				0x85 => self.l = (self.l & !(1 << n)),
				0x86 => self.mmu_ref.set_byte(self.get_hl() as usize, (self.mmu_ref.get_byte(self.get_hl() as usize) & !(1 << n))),

				// reset bit n of register
				0xC7 => self.a = (self.a | (1 << n)),
				0x80 => self.b = (self.b | (1 << n)),
				0x81 => self.c = (self.c | (1 << n)),
				0x82 => self.d = (self.d | (1 << n)),
				0x83 => self.e = (self.e | (1 << n)),
				0x84 => self.h = (self.h | (1 << n)),
				0x85 => self.l = (self.l | (1 << n)),
				0x86 => self.mmu_ref.set_byte(self.get_hl() as usize, (self.mmu_ref.get_byte(self.get_hl() as usize) | (1 << n))),







				_ => {
					panic!("Error: Invalid opcode!");
				}
			}
		}
		else {
			match opcode {

				0x00 => (),

				0x31 => ld_word(&mut self.s, &mut self.p, nn),
				0xAF => xor(&mut self.a, n, &mut self.f),
				0x21 => ld_word(&mut self.h, &mut self.l, nn),

				0x32 => self.mmu_ref.set_byte(self.get_hl() as usize, n),
				//0x44 => bit(&self.h, 7, &mut self.f),
				0x20 => if ((self.f & 128) != 0) {self.pc = nn}, // JMP if non-zero


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
			
				0x47 => self.b = self.a,
				0x4F => self.c = self.a,
				0x57 => self.d = self.a,
				0x5F => self.e = self.a,
				0x67 => self.h = self.a,
				0x6F => self.l = self.a,
				0x02 => self.mmu_ref.set_byte(self.get_bc() as usize, self.a),
				0x12 => self.mmu_ref.set_byte(self.get_de() as usize, self.a),
				0x77 => self.mmu_ref.set_byte(self.get_hl() as usize, self.a),
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

				// increment registers
				0x03 => self.set_bc(self.get_bc()+1),
				0x13 => self.set_de(self.get_de()+1),
				0x23 => self.set_hl(self.get_hl()+1),
				0x33 => self.set_sp(self.get_sp()+1),
				
				// decrement registers
				0x0B => self.set_bc(self.get_bc()-1),
				0x1B => self.set_de(self.get_de()-1),
				0x2B => self.set_hl(self.get_hl()-1),
				0x3B => self.set_sp(self.get_sp()-1),


				

				// jump
				0xC3 => self.pc = self.l_e_word_conversion(nn) - (instruction_size as u16),


				






				_ => {
					panic!("Error: Invalid opcode!");
				}
			}
		}

	}
}











