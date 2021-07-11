use crate::mmu as mmu;

//#[derive(Copy, Clone)]
pub struct PPU {
}

impl PPU {

	pub fn new(system_mmu : & mut mmu::MMU) -> PPU {
		PPU {
			// PPU Registers	
	

			// rgba8 data to be drawn to screen
		}
	}

	/*
	fn get_tile(mmu_ref : &mut mmu::MMU) {
			

	}





	fn get_bg_data(bg_1 : bool, mmu_ref : &mut mmu::MMU) {
		//get_tile();

		start_addr : usize = ternary!(bg_1, 0x9800, 0x9C00);
		for i in 0..1024 {
			let index = 

		}

	}



	pub fn render_screen(mmu_ref : &mut mmu::MMU) {
		
		// TODO should be set according to lcdc i/o register
		let bg_1 = true;	
		get_bg_data(bg_1);

	}

	*/


}
