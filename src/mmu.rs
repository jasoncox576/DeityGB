
#[derive(Clone)]
pub struct MMU {

	/*
	00 	3FFF 	16KB ROM bank 00 	From cartridge, usually a fixed bank
	4000 	7FFF 	16KB ROM Bank 01~NN 	From cartridge, switchable bank via MBC (if any)
	8000 	9FFF 	8KB Video RAM (VRAM) 	Only bank 0 in Non-CGB mode

	Switchable bank 0/1 in CGB mode
	A000 	BFFF 	8KB External RAM 	In cartridge, switchable bank if any
	C000 	CFFF 	4KB Work RAM (WRAM) bank 0 	
	D000 	DFFF 	4KB Work RAM (WRAM) bank 1~N 	Only bank 1 in Non-CGB mode

	Switchable bank 1~7 in CGB mode
	E000 	FDFF 	Mirror of C000~DDFF (ECHO RAM) 	Typically not used
	FE00 	FE9F 	Sprite attribute table (OAM) 	
	FEA0 	FEFF 	Not Usable 	
	FF00 	FF7F 	I/O Registers 	
	FF80 	FFFE 	High RAM (HRAM) 	
	FFFF 	FFFF 	Interrupts Enable Register (IE) 	
	*/

	/*

	Interrupt flags data

	Bit 0: VBlank   Interrupt Enable  (INT 40h)  (1=Enable)
	Bit 1: LCD STAT Interrupt Enable  (INT 48h)  (1=Enable)
	Bit 2: Timer    Interrupt Enable  (INT 50h)  (1=Enable)
	Bit 3: Serial   Interrupt Enable  (INT 58h)  (1=Enable)
	Bit 4: Joypad   Interrupt Enable  (INT 60h)  (1=Enable)
	*/



	pub memory : Vec<u8>, 
}



impl MMU {

	pub fn new() -> MMU {
		MMU {
			//memory : [0 ; 0x10000],
			memory : vec![0; 0x10000],
		}
	}

    pub fn set_byte(&mut self, addr: usize, data : u8) {
        self.memory[addr] = data;
    }
    pub fn get_byte(&self, addr: usize) -> u8 {
        return self.memory[addr];
    }

	pub fn enable_interrupts(&mut self) {
		// TODO: Not sure if the whole bit should be set
		self.memory[0xFFFF] = 0xFF;
	}

	pub fn disable_interrupts(&mut self) {
		// TODO: Not sure if the whole bit should be set
		self.memory[0xFFFF] = 0x00;
	}


	

	pub fn load_rom(&mut self, rom_data : Vec<u8>) {
		// TODO assuming, for now, that ROM data leq than 4000 bytes
		for i in 0..(rom_data.len()) {
			self.memory[i] = rom_data[i];
			println!("{:#02x}", rom_data[i]);
		}
		println!("=================================");

	}


}
