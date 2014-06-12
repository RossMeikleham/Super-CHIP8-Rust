use std::rand::random;
use std::bool;
use graphics::Graphics; 
/* CPU, Graphics and Memory core */

mod graphics;

//enum mode = {SCHIP, CHIP}; /* differentiate between CHIP and SCHIP modes */

static U12_MAX : u16 = 4096;
static FLAG : uint = 15;


struct CPU {
    registers : [u8, ..16], /* 16 8 bit general purpose registers */
    mem : [u8, .. 0xFFF], /* 4096bytes of memory */
    I : u16, /* 16 bit index register */
    pc: u16, /* Program counter */
    sp: uint, /* Stack Pointer */
    stack : [u16, ..16], /* 16 stack frames */
    sound_timer : u8, 
    delay_timer : u8,
    hp_48_flags: [u16, ..8], /*SCHIP */
    opcode : u16,
    graphics :Graphics

}



impl CPU {
   
    fn new(mut mem: Vec<u8>) -> CPU {
        let mut cpu = CPU { registers: [0u8, ..16], 
              mem: [0u8, ..0xFFF],
              I: 0,
              pc: 0x200,
              sp: 1,
              stack : [0u16, ..16],
              sound_timer: 0,
              delay_timer: 0,
              hp_48_flags: [0u16, ..8],
              opcode : 0,
              graphics : Graphics::new()
       };
        
    
       /* Move values out of supplied vector and into
        * CPU mem */
       mem.reverse(); /* reverse vector first as poping off the end (O(1))
                         is much cheaper than shifting from the front (O(n)) */
       for val in cpu.mem.mut_iter() {
            *val = match mem.pop() {
                    Some(item) => item,
                    None => 0x0
            };
       }

       return cpu;
    }
     
    
    fn to_addr(dig1 :u8, dig2 :u8, dig3 :u8) -> u16 {
        ((dig1 << 8) | (dig2 << 4) | dig3) as u16
    }  

    fn to_val(dig1 :u8, dig2 :u8) -> u8 {
        (dig1 << 4) | dig2
    }

    fn u16_to_hex_vec(hex :u16) -> (u8, u8, u8, u8) {
         (((hex & 0xF000) >> 12) as u8,  
          ((hex & 0x0F00) >> 8)  as u8,
          ((hex & 0x00F0) >> 4)  as u8, 
           (hex & 0x000F)        as u8) 
    }

    fn get_opcode(&self) -> u16 {
        (self.registers[self.pc as uint] << 4) as u16 |
        (self.registers[(self.pc + 1) as uint]) as u16
    } 

    fn perform_cycle(&mut self) {
        
        let opcode = self.get_opcode();
        let opcode_v =  CPU::u16_to_hex_vec(opcode);
        self.inc_pc();
        
        match opcode_v {
            (0x0, 0x0 ,0xE, 0x0) => self.clear_screen(),
            (0x0, 0x0, 0xE, 0xE) => self.ret(),
            (0x0, N1, N2, N3) => {}, 
            (0x1, N1, N2, N3) => self.jump(CPU::to_addr(N1, N2, N3)),
            (0x2, N1, N2, N3) => self.call(CPU::to_addr(N1, N2, N3)),
            (0x3, X, N1, N2) => self.skip_equals_reg_val(X, CPU::to_val(N1, N2)),
            (0x4, X, N1, N2) => self.skip_not_equals_reg_val(X, CPU::to_val(N1, N2)),
            (0x5, X, Y, 0x0) => self.skip_equals_regs(X, Y),
            (0x6, X, N1, N2) => self.mov_reg_val(X, CPU::to_val(N1, N2)),
            (0x7, X, N1, N2) => self.add_reg_val(X, CPU::to_val(N1, N2)),
            (0x8, X, Y, 0x0) => self.mov_regs(X, Y),
            (0x8, X, Y, 0x1) => self.or_regs(X, Y),
            (0x8, X, Y, 0x2) => self.and_regs(X, Y),
            (0x8, X, Y, 0x3) => self.xor_regs(X, Y),
            (0x8, X, Y, 0x4) => self.add_regs(X, Y),
            (0x8, X, Y, 0x5) => self.sub_regs(X, Y),
            (0x8, X, _, 0x6) => self.shift_right(X),
            (0x8, X, Y, 0x7) => self.sub_regs(Y, X),
            (0x8, X, _, 0xE) => self.shift_left(X),
            (0x9, X, Y, 0x0) => self.skip_not_equals_regs(X, Y),
            (0xA, N1, N2, N3) => self.set_i(CPU::to_addr(N1, N2, N3)),
            (0xB, N1, N2, N3) => self.jump_val_reg0(CPU::to_addr(N1, N2, N3)),
            (0xC, X, N1, N2) => self.rand(X, CPU::to_val(N1, N2)),
            (0xD, X, Y, N) => {},
            (0xE, X, 0x9, 0xE) => {},
            (0xE, X, 0xA, 0x1) => {},
            (0xF, X, 0x0, 0x7) => {},
            (0xF, X, 0x0, 0xA) => {},
            (0xF, X, 0x1, 0x5) => self.set_reg_delay(X),
            (0xF, X, 0x1, 0x8) => self.set_reg_sound(X),
            (0xF, X, 0x1, 0xE) => self.add_reg_index(X),
            (0xF, X, 0x2, 0x9) =>  {},
            (0xF, X, 0x3, 0x3) => self.binary_decimal(X),
            (0xF, X, 0x5, 0x5) => self.store_regs(X),
            (0xF, X, 0x6, 0x5) => self.load_regs(X),
            _ => fail!("Unknown opcode {:x}",opcode)
        }
    }
     
    
    fn reset(&mut self) {
        self.pc = 0x200;
        self.sp = 0;
    }

    fn pop(&mut self) -> u16 {
        let item = self.stack[self.sp];
        self.sp -= 1;
        return item;
         
    }

    fn push(&mut self, val:u16) {
        self.sp += 1;
        self.stack[self.sp] = val;
    }

    fn inc_pc(&mut self) {
        self.pc += 2;
    }

    /* Jump to address at 0NNN */
    fn call(&mut self, addr:u16) {
        self.pc = addr & 0xFFF;
    }

    /* Clear the screen */
    fn clear_screen(&mut self) {

    }

    /* Return from a subroutine */
    fn ret(&mut self) {
        self.pc = self.pop();
    }

    fn jump(&mut self, addr: u16) {
        self.pc = addr & 0xFFF;
    }

    /* Skip next instruction if register X is equal to NN */
    fn skip_equals_reg_val(&mut self, reg:u8, val:u8) {
        if self.registers[reg as uint] == val {
            self.inc_pc();
        }
                
    }

    fn skip_not_equals_reg_val(&mut self, reg:u8, val:u8) {
        if self.registers[reg as uint] != val {
            self.inc_pc();
        }
    }

    fn skip_equals_regs(&mut self, reg1:u8, reg2:u8) {
        
        if self.registers[reg1 as uint] == self.registers[reg2 as uint] {
               self.inc_pc();
        }
    }

    fn skip_not_equals_regs(&mut self, reg1:u8, reg2:u8) {
        if self.registers[reg1 as uint] != self.registers[reg2 as uint] {
               self.inc_pc();
           }
    }

    fn mov_reg_val(&mut self, reg:u8, val:u8) {
        self.registers[reg as uint] = val;
    }


    fn add_reg_val(&mut self, reg:u8, val:u8) {
        self.registers[reg as uint] += val;
    }
    
    fn mov_regs(&mut self, reg1:u8, reg2:u8) {
        self.registers[reg1 as uint] = self.registers[reg2 as uint];
    }

    fn or_regs(&mut self, reg1:u8, reg2:u8) {
       self.registers[reg1 as uint] |= self.registers[reg2 as uint];
    }

    fn and_regs(&mut self, reg1:u8, reg2:u8) {
        self.registers[reg1 as uint] |= self.registers[reg2 as uint];
    }

    fn xor_regs(&mut self, reg1:u8, reg2:u8) {
        self.registers[reg1 as uint] ^= self.registers[reg2 as uint];
    }

    /* Add two regs, if overflow set flag register otherwise
     * unset flag register */
    fn add_regs(&mut self, reg1:u8, reg2:u8) {
        self.registers[FLAG] =  bool::to_bit::<u8>(0xFF - reg1 > reg2);
        self.registers[reg1 as uint] += self.registers[reg2 as uint];  
    }

    /* Sub reg2 from reg1, unset flag register if underflow,
     * set otherwise */
    fn sub_regs(&mut self, reg1:u8, reg2:u8) {
        self.registers[FLAG] = bool::to_bit::<u8>(reg2 < reg1);
        self.registers[reg1 as uint] -= self.registers[reg2 as uint]; 
    }

    /* Sub reg1 from reg2, unset flag register if underflow,
     * set otherwise */
    fn sub_regs_inv(&mut self, reg1:u8, reg2:u8) {
        self.registers[FLAG] = bool::to_bit::<u8>(reg1 < reg2);
        self.registers[reg2 as uint] -= self.registers[reg1 as uint]; 
    }


    /* Shift register left by 1, set flag register to most significant bit
     * before shifting */
    fn shift_left(&mut self, reg:u8) {
        self.registers[FLAG] = (self.registers[reg as uint] & 0x80) >> 7;
        self.registers[reg as uint] <<= 1;
    }


    /* Shift register right by 1, set flag register to least significant bit
     * before shifting */
    fn shift_right(&mut self, reg:u8) {
        self.registers[FLAG] = self.registers[reg as uint] & 0x1;
        self.registers[reg as uint] >>= 1;
    }

    /* Set index register to address in opcode */
    fn set_i(&mut self, addr:u16) {        
        self.I = addr;
    }

    fn jump_val_reg0(&mut self, addr:u16) {
        self.pc = self.registers[0] as u16 + addr;
    }

    fn rand(&mut self, reg:u8, val:u8) {
        self.registers[reg as uint] = val & random::<u8>();
    }

    fn set_reg_delay(&mut self, reg:u8) {
        self.registers[reg as uint] = self.delay_timer;
    }
    
    fn set_reg_sound(&mut self, reg:u8) {
        self.registers[reg as uint] = self.sound_timer;
    }

    fn add_reg_index(&mut self, reg:u8) {
        self.registers[FLAG] = 
            bool::to_bit::<u8>(0xFFF - (self.registers[reg as uint] as u16) < self.I); 
        self.I += self.registers[reg as uint] as u16
    }

    fn store_regs(&mut self, reg:u8) {
        let mut addr = self.I;
        for reg in self.registers.iter() {
            self.mem[addr as uint] = *reg;
            addr += 1;
        }
    }

    fn load_regs(&mut self, reg:u8) {
        let mut addr = self.I;
        for reg in self.registers.mut_iter() {
            *reg = self.mem[addr as uint];
            addr += 1;
        }
    }

    /*stores the Binary-coded decimal representation of VX, with the
     * most significant of three digits at the address in I, the middle digit
     * at I + 1, and the lsg at I + 2.*/
    fn binary_decimal(&mut self, reg:u8) {
        let val = self.registers[reg as uint];
        self.mem[self.I as uint] = val/100;
        self.mem[(self.I + 1) as uint] = (val % 100)/10;
        self.mem[(self.I + 2) as uint] = (val % 100)%10;
 
    }

}


fn main() {

   
    print!("yay");
}

