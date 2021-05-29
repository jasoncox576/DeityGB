
pub struct MMU {
    memory : [u8 ; 0x10000],
}

impl MMU {
    fn set_byte(&mut self, addr: usize, data : u8) {
        self.memory[addr] = data;
    }
    fn get_byte(self, addr: usize) {
        return self.memory[addr];
    }
    // TODO more stuff needed? Most likely
}
