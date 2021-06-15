
#[derive(Copy, Clone)]
pub struct MMU {
    // TODO: Explicitly write out the memory map
    memory : [u8 ; 0x10000],
}

impl MMU {

	pub fn new() -> MMU {
		MMU {
			memory : [0 ; 0x10000],
		}
	}

    pub fn set_byte(&mut self, addr: usize, data : u8) {
        self.memory[addr] = data;
    }
    pub fn get_byte(self, addr: usize) -> u8 {
        return self.memory[addr];
    }
    // TODO more stuff needed? Most likely
}
