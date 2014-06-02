/* CPU, Graphics and Memory core */

//enum mode = {SCHIP, CHIP}; /* differentiate between CHIP and SCHIP modes */

static MAX_RAM : u16 = 4096;

struct CPU {
    registers : [u8, ..16], /* 16 8 bit general purpose registers */
    mem : [u8, .. 0xFFF], /* 4096bytes of memory */
    i : u16, /* 16 bit index register */
    pc: u16, /* Program counter */
    sp: u16, /* Stack Pointer */
    stack : [u16, ..16], /* 16 stack frames */
    sound_timer : u8, 
    delay_timer : u8,
    hp_48_flags: [u16, ..8] /*SCHIP */

}


fn dec_pc(pc:u16) -> uint {
    ((pc - 1) % MAX_RAM) as uint
}

fn dec_pc_n(pc:u16, n:u16) -> uint {
    ((pc - n) % MAX_RAM) as uint
}

fn inc_pc(pc:u16) -> uint {
    ((pc + 1) % MAX_RAM) as uint
}

fn inc_pc_n(pc:u16, n:u16) -> uint {
    ((pc + n) % MAX_RAM) as uint
}


impl CPU {

    fn call(&self) {
        self.pc = ((self.mem[dec_pc_n(self.pc, 2)] 
            & 0x0F) << 8) as u16 | self.mem[dec_pc(self.pc)] as u16;
    }
}


fn main() {

    print!("hello");
}

