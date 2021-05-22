
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


	a : u8,
	f : u8,

	b : u8,
	c : u8,

	d : u8,
	e : u8,
	
	h : u8,
	l : u8,
	

	wram : [u8 ; 8000],

	sp : u16,
	pc : u16,


	// NOTE on the Flags register 'f':
	/*
	Bit 	Name 	Explanation
	7 	z 	Zero flag
	6 	n 	Subtraction flag (BCD)
	5 	h 	Half Carry flag (BCD)
	4 	c 	Carry flag
	*/
}




impl CPU {

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






	fn decode(&mut self) {

		let mut cycles : u8 = 0;
		// TODO fix this
		let opcode : u8 = 0;
		match opcode {
			// nop
			0x00 => {
				cycles = 4;		
				self.pc += 1;
			},

			// return	
			0xC9 => {
				cycles = 16;
				self.pc = self.wram[self.sp as usize] as u16;
				self.sp += 2;
			}
			
			// ALU
	
			/*
			0x87 => {
				cycles = 4;
				addA(self.A);
				self.pc += 1
			}
			*/
			0xA7 => {
				cycles = 4;				
				self.and(self.a);				
				self.pc += 1;
			}
	
			0xA0 => {
				cycles = 4;				
				self.and(self.b);				
				self.pc += 1;
			}

			0xA1 => {
				cycles = 4;				
				self.and(self.c);				
				self.pc += 1;
			}

			0xA2 => {
				cycles = 4;				
				self.and(self.d);				
				self.pc += 1;
			}

			0xA3 => {
				cycles = 4;				
				self.and(self.e);				
				self.pc += 1;
			}

			0xA4 => {
				cycles = 4;				
				self.and(self.h);				
				self.pc += 1;
			}

			0xA5 => {
				cycles = 4;				
				self.and(self.l);				
				self.pc += 1;
			}

			0xA6 => {
				cycles = 8;				
				let hl : u16 = self.get_hl();
				self.and(self.wram[hl as usize]);				
				self.pc += 1;
			}
			
			0xE8 => {
				cycles = 8;				
				self.and(self.wram[(self.pc as usize)+1]);
				self.pc += 2;
			}

		
			0xB7 => {
				cycles = 4;
				self.or(self.a);
				self.pc += 1;
			}
			
			0xB0 => {
				cycles = 4;
				self.or(self.b);
				self.pc += 1;
			}

			0xB1 => {
				cycles = 4;
				self.or(self.c);
				self.pc += 1;
			}
		
			0xB2 => {
				cycles = 4;
				self.or(self.d);
				self.pc += 1;
			}
		
			0xB3 => {
				cycles = 4;
				self.or(self.e);
				self.pc += 1;
			}
		
			0xB4 => {
				cycles = 4;
				self.or(self.h);
				self.pc += 1;
			}
			0xB5 => {
				cycles = 4;
				self.or(self.l);
				self.pc += 1;
			}
			0xB6 => {
				cycles = 8;
				let hl : u16 = self.get_hl();
				self.or(self.wram[hl as usize]);				
				self.pc += 1;
			}
			0xF6 => {
				cycles = 8;
				self.or(self.wram[(self.pc as usize)+1]);
				self.pc += 2;
			}

			0xAF => {
				cycles = 4;
				self.xor(self.a);
				self.pc += 1;
			}
			
			0xA8 => {
				cycles = 4;
				self.xor(self.b);
				self.pc += 1;
			}

			0xA9 => {
				cycles = 4;
				self.xor(self.c);
				self.pc += 1;
			}
		
			0xAA => {
				cycles = 4;
				self.xor(self.d);
				self.pc += 1;
			}
		
			0xAB => {
				cycles = 4;
				self.xor(self.e);
				self.pc += 1;
			}
		
			0xAC => {
				cycles = 4;
				self.xor(self.h);
				self.pc += 1;
			}
			0xAD => {
				cycles = 4;
				self.xor(self.l);
				self.pc += 1;
			}
			0xAE => {
				cycles = 8;
				let hl : u16 = self.get_hl();
				self.xor(self.wram[hl as usize]);				
				self.pc += 1;
			}
			0xEE => {
				cycles = 8;
				self.xor(self.wram[(self.pc as usize)+1]);
				self.pc += 2;
			}
	
			/*
			0x03 => self.inc_nn(&mut self.b, &mut self.c),
			0x13 => self.inc_nn(&mut self.d, &mut self.e),
			0x23 => self.inc_nn(&mut self.h, &mut self.l),
			0x33 => self.inc_nn(&mut self.s, &mut self.p),
			*/

			0x37 => swap_n(&mut self.a),
			0x30 => swap_n(&mut self.b),
			0x31 => swap_n(&mut self.c),
			0x32 => swap_n(&mut self.d),
			0x33 => swap_n(&mut self.e),
			0x34 => swap_n(&mut self.h),
			0x35 => swap_n(&mut self.l),
			0x36 => {
				let hl = self.get_hl();
				swap_n(&mut self.wram[hl as usize]);
			},
			


			0x3F => ccf(&mut self.f),
			0x2F => cpl(&mut self.a),
			0x37 => scf(&mut self.f),




			_ => {
				panic!("Error: Invalid opcode!");
			}

		}
		





	}




}











