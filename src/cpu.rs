
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


    // performs a F/D/E/WB cycle
    pub fn cycle() {
        next_opcode : u8 = fetch();
        decode_execute(

    }





    fn ld_word(&mut reg: u8, val: u16) {
        *reg = val;
    }



	fn decode_execute(&mut self, opcode : u8, 0) {

        // TODO: create cycle table later for timing accuracy
		let mut cycles : u8 = 0;
		match opcode {

            // NOTE: Here, we flesh out all the boot ROM instructions first
            0x31 => {
                // TODO replace 100 with next word in memory
                ld_word(self.SP, 100);
            }

            0xAF => {
                
            }
            
            0x21 => {

            }


            







			_ => {
                panic!("Error: Invalid opcode!");
            }
		}
	}
}











