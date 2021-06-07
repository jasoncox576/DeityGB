//mod mmu;


fn swap_n(n : &mut u8) {
	// swap upper and lower nibbles of reg n
	let lower : u8 = *n & 0x0F;
	let upper : u8 = *n & 0xF0;
	*n = lower | upper;
}

fn ccf(f : &mut u8) {
	// complement carry flag
	*f = (*f ^ 0b00010000) & 0b10011111;
}

fn cpl(a : &mut u8) {
	*a = !*a;
}


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
	
    //mmu_ref : &mut mmu::MMU,

	sp : u16,
	pc : u16,

}





impl CPU {

	pub fn new() -> CPU {
		CPU {
			a : 0, f : 0,
			b : 0, c : 0,
			d : 0, e : 0,
			h : 0, l : 0,
			//mmu_ref : &mut mmu,
			sp : 0,
			pc : 0,
		}
	}



	/*
	fn get_hl(&mut self) -> u16 {
		return ((self.h as u16) << 8) | (self.l as u16);
	}

	fn and(&mut self, n : u8) {
		// bitwise AND, set zero (if zero) and half-carry flag
		self.a = self.a & n;
		let z : u8 = ((self.a == 0) as u8) << 7;
		self.f = z | 0b00100000;
	}

	fn or(&mut self, n : u8) {
		// bitwise OR, set zero (if zero)
		self.a = self.a | n;
		let z : u8 = ((self.a == 0) as u8) << 7;
		self.f = z;
	}

	fn xor(&mut self, n : u8) {
		// bitwise XOR, set zero (if zero)
		self.a = self.a ^ n;
		let z : u8 = ((self.a == 0) as u8) << 7;
		self.f = z;
	}

    fn load_word(&self) -> u16 {
        let b1 : u16 = (self.mmu_ref.get_byte(self.pc + 1 as usize)] as u16) << 8;
        let b2 : u16 = (self.mmu_ref.get_byte(self.pc + 2 as usize) as u16;
        return b1 | b2;
    }

    fn set_hl(&mut self, word : u16) {
        self.h = ((word & 0xFF00) >> 8) as u8;
        self.l = (word & 0x00FF) as u8;
    }
	*/




	/*
	fn cp(&mut self, n: u8) {
		// Compare A with n (basically subtraction, but results thrown away)
		let res = self.a - n;
		let z : u8 = ((self.a == 0) as u8) << 7;
		let n : u8 = 1;
		let h : u8 = 
		let c : u8 = ((self.a < n) as u8) << 7;
		self.f = 
	}
	*/


	/*
	fn inc_nn(&mut self, n1: &mut u8, n0: &mut u8) {
		if *n0 == 255 {
			*n0 = 0;		
			*n1 += 1	
		}
		else {*n0 += 1};	
	}
	*/

    // TODO explicitly write out fetch functoin?



	/*
	fn decode_execute(&mut self, opcode : u8, 0) {

		let mut cycles : u8 = 0;
		match opcode {
			// nop
			0x00 => {
				cycles = 4;		
				self.pc += 1;
			},

			// return	
			0xC9 => {
				cycles = 16;
				self.pc = self.mmu_ref.get_byte(self.sp as usize) as u16;
				self.sp += 2;
			}


            // NOTE: Here are all the instructions used by the
            // bootstrap ROM
            0x31 => {
                cycles = 12;
                self.sp = self.load_word();
            }
			
			0xAF => {
				cycles = 4;
				self.xor(self.a);
				self.pc += 1;
			}

            0x21 => {
                cycles = 12;
                self.set_hl(self.load_word());
                self.pc += 3;
            }

            /*
            0x32 => {
                cycles = 8;
                self.ldd
            }
            */

			_ => {
                panic!("Error: Invalid opcode!");
            }
		}
	}
	*/
}











