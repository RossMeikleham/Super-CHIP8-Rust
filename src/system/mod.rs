extern crate rand;

pub mod graphics;
pub mod io;
/* CPU, Graphics and Memory core */


const MAX_RAM : u16 = 4096;
const FLAG : usize = 15;
const CHIP_MODE : bool = false;
const SCHIP_MODE : bool = true;

pub struct CPU {
     registers : [u8; 16], /* 16 8 bit general purpose registers */
     mem : [u8; MAX_RAM as usize], /* 4096bytes of memory */
     index_reg : u16, /* 16 bit index register */
     pc: u16, /* Program counter */
     sp: usize, /* Stack Pointer */
     stack : [u16; 16], /* 16 stack frames */
     sound_timer : u8, 
     delay_timer : u8,
     hp_48_flags: [u8; 8], /*SCHIP */
     graphics :graphics::Graphics,
     io :io::IO,
     halt:bool,
     mode: bool

}


static SPRITE_SET: [u8; (80 + 160)] =   
   [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80,  //F 

    0x00, 0x3C, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3C, 0x00, //0
    0x00, 0x08, 0x38, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3E, 0x00, //1
    0x00, 0x38, 0x44, 0x04, 0x08, 0x10, 0x20, 0x44, 0x7C, 0x00, //2
    0x00, 0x38, 0x44, 0x04, 0x18, 0x04, 0x04, 0x44, 0x38, 0x00, //3
    0x00, 0x0C, 0x14, 0x24, 0x24, 0x7E, 0x04, 0x04, 0x0E, 0x00, //4
    0x00, 0x3E, 0x20, 0x20, 0x3C, 0x02, 0x02, 0x42, 0x3C, 0x00, //5
    0x00, 0x0E, 0x10, 0x20, 0x3C, 0x22, 0x22, 0x22, 0x1C, 0x00, //6
    0x00, 0x7E, 0x42, 0x02, 0x04, 0x04, 0x08, 0x08, 0x08, 0x00, //7
    0x00, 0x3C, 0x42, 0x42, 0x3C, 0x42, 0x42, 0x42, 0x3C, 0x00, //8
    0x00, 0x3C, 0x42, 0x42, 0x42, 0x3E, 0x02, 0x04, 0x78, 0x00, //9
    0x00, 0x18, 0x08, 0x14, 0x14, 0x14, 0x1C, 0x22, 0x77, 0x00, //A
    0x00, 0x7C, 0x22, 0x22, 0x3C, 0x22, 0x22, 0x22, 0x7C, 0x00, //B
    0x00, 0x1E, 0x22, 0x40, 0x40, 0x40, 0x40, 0x22, 0x1C, 0x00, //C
    0x00, 0x78, 0x24, 0x22, 0x22, 0x22, 0x22, 0x24, 0x78, 0x00, //D
    0x00, 0x7E, 0x22, 0x28, 0x38, 0x28, 0x20, 0x22, 0x7E, 0x00, //E
    0x00, 0x7E, 0x22, 0x28, 0x38, 0x28, 0x20, 0x20, 0x70, 0x00  //F
  ]; 
   

#[allow(dead_code)]
impl CPU {

  pub fn new(mem: Vec<u8>) -> CPU {
        let mut cpu = CPU { registers: [0u8; 16], 
              mem: [0u8; (MAX_RAM as usize)],
              index_reg: 0,
              pc: 0x200,
              sp: 1,
              stack : [0u16; 16],
              sound_timer: 0,
              delay_timer: 0,
              hp_48_flags: [0u8; 8],
              graphics : graphics::Graphics::new(),
              io : io::IO::new(),
              halt:false,
              mode:CHIP_MODE 
       };
        
       
       /* Fill up emulator emu values with supplied memory */
       for (m, v) in cpu.mem.iter_mut().zip(mem.iter()) {
           *m = *v;
       }
      
       /* Load CHIP8 fontset into unused locations 0x0 - 0x50 in memory */ 
       for (m, v) in cpu.mem.iter_mut().zip(SPRITE_SET.iter()) {
            *m = *v;
       }

       return cpu;
    }

   pub fn get_mem(&self, loc:u16) -> u8 {
       self.mem[(loc % MAX_RAM) as usize]
   }

   pub fn get_reg(&self, reg:u8) -> u8 {
       self.registers[reg as usize]
   }

   pub fn get_pc(&self) -> u16 {
       self.pc
   }

   pub fn get_index_reg(&self) -> u16 {
       self.index_reg
   }
     
    /* converts 3 hex digits into a 12 bit address */    
    pub fn to_addr(dig1 :u8, dig2 :u8, dig3 :u8) -> u16 {
        ((dig1 as u16) << 8) | ((dig2 as u16) << 4)  | dig3 as u16
    }  

    /* combines 2 hex digits into a 8 bit value */
    pub fn to_val(dig1 :u8, dig2 :u8) -> u8 {
        (dig1 << 4) | dig2
    }

    /* takes a 12 bit value and extracts each hex digit
     * and stores in a tuple */ 
    pub fn u16_to_hex_vec(hex :u16) -> (u8, u8, u8, u8) {
         (((hex & 0xF000) >> 12) as u8,  
          ((hex & 0x0F00) >> 8)  as u8,
          ((hex & 0x00F0) >> 4)  as u8, 
           (hex & 0x000F)        as u8) 
    }

    /* obtains the current 16 bit opcode from memory */
    fn get_opcode(&self) -> u16 {

        (((self.mem[self.pc as usize]) as u16) << 8)  |
        (self.mem[(self.pc + 1) as usize]) as u16
    } 

    /* perform 1 CPU instruction */
    fn execute(&mut self, opcode:u16) {
        
        let opcode_v =  CPU::u16_to_hex_vec(opcode);
        self.inc_pc();

        match opcode_v {
            (0x0, 0x0 ,0xE, 0x0) => self.clear_screen(),
            (0x0, 0x0, 0xE, 0xE) => self.ret(), 
           
            (0x0, a, b, c) => {
                let mode = self.mode;
                match (mode, (a, b, c)) {
                    (SCHIP_MODE, (0x0, 0xC, n)) => self.scroll_n_down(n),
                    (SCHIP_MODE, (0x0, 0xF, 0xB)) => self.scroll_4_right(),
                    (SCHIP_MODE, (0x0, 0xF, 0xC)) => self.scroll_4_left(),
                    (SCHIP_MODE, (0x0, 0xF, 0xD)) => self.exit(),
                    (SCHIP_MODE, (0x0, 0xF, 0xE)) => self.set_chip_mode(),
                    (CHIP_MODE, (0x0, 0xF, 0xF)) => self.set_super_chip_mode(),
                     _ => {}, 
                }
            },

            (0x1, n1, n2, n3) => self.jump(CPU::to_addr(n1, n2, n3)),
            (0x2, n1, n2, n3) => self.call(CPU::to_addr(n1, n2, n3)),
            (0x3, x, n1, n2) => self.skip_equals_reg_val(x, CPU::to_val(n1, n2)),
            (0x4, x, n1, n2) => self.skip_not_equals_reg_val(x, CPU::to_val(n1, n2)),
            (0x5, x, y, 0x0) => self.skip_equals_regs(x, y),
            (0x6, x, n1, n2) => self.mov_reg_val(x, CPU::to_val(n1, n2)),
            (0x7, x, n1, n2) => self.add_reg_val(x, CPU::to_val(n1, n2)),
            (0x8, x, y, 0x0) => self.mov_regs(x, y),
            (0x8, x, y, 0x1) => self.or_regs(x, y),
            (0x8, x, y, 0x2) => self.and_regs(x, y),
            (0x8, x, y, 0x3) => self.xor_regs(x, y),
            (0x8, x, y, 0x4) => self.add_regs(x, y),
            (0x8, x, y, 0x5) => self.sub_regs(x, y, x),
            (0x8, x, _, 0x6) => self.shift_right(x),
            (0x8, x, y, 0x7) => self.sub_regs(y, x, x),
            (0x8, x, _, 0xE) => self.shift_left(x),
            (0x9, x, y, 0x0) => self.skip_not_equals_regs(x, y),
            (0xA, n1, n2, n3) => self.set_i(CPU::to_addr(n1, n2, n3)),
            (0xB, n1, n2, n3) => self.jump_val_reg0(CPU::to_addr(n1, n2, n3)),
            (0xC, x, n1, n2) => self.rand(x, CPU::to_val(n1, n2)),

            (0xD, x, y, n) =>  match (self.mode, (x, y, n)) {
                (SCHIP_MODE, (x, y, 0x0)) => self.draw_extended_sprite(x, y),
                (_, (x, y, n))       => self.draw_sprite(x, y, n),
            },

            (0xE, x, 0x9, 0xE) => self.skip_key_pressed(x),
            (0xE, x, 0xA, 0x1) => self.skip_not_key_pressed(x),
            (0xF, x, 0x0, 0x7) => self.set_reg_delay(x),
            (0xF, x, 0x0, 0xA) => self.wait_for_key(x),
            (0xF, x, 0x1, 0x5) => self.set_delay_reg(x),
            (0xF, x, 0x1, 0x8) => self.set_sound_reg(x),
            (0xF, x, 0x1, 0xE) => self.add_reg_index(x),
            (0xF, x, 0x2, 0x9) => self.load_sprite(x),
            (0xF, x, 0x3, 0x3) => self.binary_decimal(x),
            (0xF, x, 0x5, 0x5) => self.store_regs(x),
            (0xF, x, 0x6, 0x5) => self.load_regs(x),

            (0xF, a, b, c) =>  match (self.mode, (a, b, c)) {
                (SCHIP_MODE, (x, 0x3, 0x0)) => self.load_extended_sprite(x),
                (SCHIP_MODE, (x, 0x7, 0x5)) => self.store_hp_regs(x),
                (SCHIP_MODE, (x, 0x8, 0x5)) => self.load_hp_regs(x),
                _ => panic!("Unknown opcode {:x}", opcode)
            },

            _ => panic!("Unknown opcode {:x}",opcode)
        }

        if self.delay_timer > 0 { self.delay_timer -= 1;}
        if self.sound_timer > 0 { self.sound_timer -= 1;}
    }

    pub fn perform_cycle(&mut self) {
        let opcode = self.get_opcode();
        self.execute(opcode);
    }

    pub fn interpret(&mut self, opcode:u16) {
        self.execute(opcode);
    }

    pub fn is_finished(&self) -> bool {
        self.halt
    }

    
    /* pop an item from the top of the stack,
     * decrements the stack pointer after popping */
    fn pop(&mut self) -> u16 {
        let item = self.stack[self.sp];
        self.sp -= 1;
        return item;
         
    }
    
    /* push an item on to the stack,
     * increments the stack pointer before
     * pushing */
    fn push(&mut self, val:u16) {
        self.sp += 1;
        self.stack[self.sp] = val;
    }

    /* Increments the program counter */
    fn inc_pc(&mut self) {
        self.pc += 2;
        self.pc %= MAX_RAM;
    }

    /* Store the current program counter
     * on the stack and jump to the supplied address */
    fn call(&mut self, addr:u16) {
        let pc = self.pc;
        self.push(pc);
        self.pc = addr % MAX_RAM;
    }

    /* Clear the display */
    fn clear_screen(&mut self) {
        self.graphics.clear_screen();
        self.graphics.show();
    }

    /* Return from a subroutine by
     * popping the return address from the stack */
    fn ret(&mut self) {
        self.pc = self.pop();
    }

    /* Set the program counter to the supplied address */
    fn jump(&mut self, addr: u16) {
        self.pc = addr;
    }

    /* Skip next instruction if register value is equal to 
     * supplied value */
    fn skip_equals_reg_val(&mut self, reg:u8, val:u8) {
        if self.registers[reg as usize] == val {
            self.inc_pc();
        }
                
    }
    /* Skip next instruction if register value is not equal
     * to supplied value */
    fn skip_not_equals_reg_val(&mut self, reg:u8, val:u8) {
        if self.registers[reg as usize] != val {
            self.inc_pc();
        }
    }

    /* Skip next instruction if both register values are equal */
    fn skip_equals_regs(&mut self, reg1:u8, reg2:u8) {
        
        if self.registers[reg1 as usize] == self.registers[reg2 as usize] {
               self.inc_pc();
        }
    }

    /* Skip next instruction if both register values are not equal */
    fn skip_not_equals_regs(&mut self, reg1:u8, reg2:u8) {
        if self.registers[reg1 as usize] != self.registers[reg2 as usize] {
               self.inc_pc();
           }
    }

    /* copy the supplied value into the register */
    fn mov_reg_val(&mut self, reg:u8, val:u8) {
        self.registers[reg as usize] = val;
    }

    /* add the supplied value to the value currently
     * stored in the register and store the result in the register */
    fn add_reg_val(&mut self, reg:u8, val:u8) {
        self.registers[reg as usize] = self.registers[reg as usize].wrapping_add(val);
    }
    
    /* Copy the value of the second register into the first register */
    fn mov_regs(&mut self, reg1:u8, reg2:u8) {
        self.registers[reg1 as usize] = self.registers[reg2 as usize];
    }

    /* perform a binary or on the values of the first and second
     * registers, store the result in the first register*/
    fn or_regs(&mut self, reg1:u8, reg2:u8) {
       self.registers[reg1 as usize] |= self.registers[reg2 as usize];
    }

    /* performa binary and on the values of the first and second
     * registers, store the result in the first register */
    fn and_regs(&mut self, reg1:u8, reg2:u8) {
        self.registers[reg1 as usize] &= self.registers[reg2 as usize];
    }

    /* perform binary xor on the values of the first and second
     * registers, store the result in the first register */
    fn xor_regs(&mut self, reg1:u8, reg2:u8) {
        self.registers[reg1 as usize] ^= self.registers[reg2 as usize];
    }

    /* Add two regs, if overflow set flag register otherwise
     * unset flag register */
    fn add_regs(&mut self, reg1:u8, reg2:u8) {
        let register1 = self.registers[reg1 as usize];
        let register2 = self.registers[reg2 as usize];
        self.registers[FLAG] =  
            match (0xFF - register1) < register2 { true => 1, false => 0};
        self.registers[reg1 as usize] = self.registers[reg1 as usize].wrapping_add(register2);  
    }


    /* subtract the value of the second register from the first register 
     * if causes negative overflow unset the flag register, otherwise
     * set flag register, performs flag setting before subtraction
     * takes place. Store result in "store_reg".*/
    fn sub_regs(&mut self, reg1:u8, reg2:u8, store_reg:u8) {
        let register1 = self.registers[reg1 as usize];
        let register2 = self.registers[reg2 as usize];
        
        self.registers[FLAG] = 
            match register2 < register1 { true => 1, false => 0};
        self.registers[store_reg as usize] = register1.wrapping_sub(register2);
    }


    /* Shift register left by 1, set flag register to most significant bit
     * before shifting */
    fn shift_left(&mut self, reg:u8) {
        self.registers[FLAG] = (self.registers[reg as usize] & 0x80) >> 7;
        self.registers[reg as usize] <<= 1;
    }


    /* Shift register right by 1, set flag register to least significant bit
     * before shifting */
    fn shift_right(&mut self, reg:u8) {
        self.registers[FLAG] = self.registers[reg as usize] & 0x1;
        self.registers[reg as usize] >>= 1;
    }

    /* Set index register to supplied address */
    fn set_i(&mut self, addr:u16) {        
        self.index_reg = addr;
    }

    /* jump to the supplied address + value in register 0 */
    fn jump_val_reg0(&mut self, addr:u16) {
        self.pc = (self.registers[0] as u16 + addr) % MAX_RAM;
    }

    /* set register to supplied value and a random integer between 0 and 255 */
    fn rand(&mut self, reg:u8, val:u8) {
        self.registers[reg as usize] = val & rand::random::<u8>();
    }

    /* Set the delay timer to the value in the register */
    fn set_delay_reg(&mut self, reg:u8) {
        self.delay_timer = self.registers[reg as usize];
    }

    /* set the register to the value in the display timer */
    fn set_reg_delay(&mut self, reg:u8) {
        self.registers[reg as usize] = self.delay_timer;
    }
    
    /* set the sound timer to the value in the register */
    fn set_sound_reg(&mut self, reg:u8) {
        self.sound_timer = self.registers[reg as usize];
    }

    /* add the value in the register to value in the index register and store
     * the result in the index register. If this operation causes overflow
     * set the flag register, otherwise unset it */
    fn add_reg_index(&mut self, reg:u8) {
        let reg = reg as usize;
        self.registers[FLAG] = 
            match (0xFFF - (self.registers[reg] as u16)) < self.index_reg {
                true => 1,
                false => 0
            }; 
        self.index_reg += self.registers[reg] as u16;
        self.index_reg %= MAX_RAM;
    }

    /* store the values from register 0 up to and including
     * the supplied register number starting from memory location
     * pointed to by the index register */
    fn store_regs(&mut self, max_reg:u8) {
        let regs = (&self.registers[.. max_reg as usize + 1]).iter();
        let store = (&mut self.mem[self.index_reg as usize ..]).iter_mut();
        /* itterate through both memory and registers*/
        for (mem, reg) in store.zip(regs) {
            *mem = *reg;
        }

    }


    /* load values into register 0 up to and including
     * the supplied register number starting from memory location
     * pointed to by the index register */
    fn load_regs(&mut self, max_reg:u8) {
        let regs = (&mut self.registers[.. max_reg as usize + 1]).iter_mut();
        let store = (&self.mem[self.index_reg as usize ..]).iter();
        /* itterate through both memory and registers */
        for (mem, reg) in store.zip(regs) {
            *reg = *mem;
        }
   }


    /*stores the Binary-coded decimal representation of VX, with the
     * most significant of three digits at the address in I, the middle digit
     * at I + 1, and the LSD at I + 2.*/
    fn binary_decimal(&mut self, reg:u8) {
        let val = self.registers[reg as usize];
        self.mem[self.index_reg as usize] = val/100;
        self.mem[(self.index_reg + 1) as usize] = (val % 100)/10;
        self.mem[(self.index_reg + 2) as usize] = (val % 100)%10;
 
    }
    
    /* Draw sprite starting at x,y which is n lines
     * of 8 pixels stored starting at memory location 
     * of the contents of register I*/
    fn draw_sprite(&mut self, x:u8, y:u8, line_count:u8) {
       
        self.registers[FLAG] = 0;
        let n = if line_count == 0 {16} else {line_count};
        for i in 0 .. n {

            let line : u8 = self.mem[(self.index_reg + (i as u16)) as usize];

            if self.graphics.draw_line(
                    self.registers[x as usize], 
                    self.registers[y as usize] + i,  
                    line as usize, 8) {

                self.registers[FLAG] = 1;
            }
        }

        self.graphics.show();       
    }

    /* set I reg to sprite number stored in the given register */
    fn load_sprite(&mut self, reg:u8) {
        self.index_reg = (5 * self.registers[reg as usize]) as u16;
    }

    /* Wait for a keypress and set the contents of the
     * given register to that keypress */
    fn wait_for_key(&mut self, reg:u8) {
        self.registers[reg as usize] = self.io.wait_for_key();  
    }
    
    /* if key in given register is being pressed then
     * skip the next instruction */
    fn skip_key_pressed(&mut self, reg:u8) {
        if self.io.is_key_pressed(self.registers[reg as usize]) {
            self.inc_pc();
        }
    }   

    /* if key in given register is not being pressed then
     * skip the next instruction */
    fn skip_not_key_pressed(&mut self, reg:u8) {
        if !self.io.is_key_pressed(self.registers[reg as usize]) {
           self.inc_pc();
        }
    
    }

    /**** Extended Super Chip Instructions ****/
    fn scroll_n_down(&mut self, n:u8) {
        self.graphics.scroll_down(n);
        self.graphics.show();
    }

    fn scroll_4_right(&mut self) {
        self.graphics.scroll_right(4);
        self.graphics.show();
    }

    fn scroll_4_left(&mut self) {
        self.graphics.scroll_left(4);
        self.graphics.show();
    }

    fn exit(&mut self) {
        self.halt = true;
       
    }

    fn set_chip_mode(&mut self)  {
        self.mode = CHIP_MODE;
        self.graphics.set_mode(self.mode);
    }
    
    fn set_super_chip_mode(&mut self) {
        self.mode = SCHIP_MODE;
        self.graphics.set_mode(self.mode);
    }


    /* Draw 16*16 sprite at x,y */
    fn draw_extended_sprite(&mut self, start_x:u8, start_y:u8) {
        self.registers[FLAG] = 0;

        for y in 0usize .. 16 {
            let line = ((self.mem[self.index_reg as usize + (2 * y)] as u16) << 4) 
                | (self.mem[self.index_reg as usize + (2 * y) + 1] as u16);
            if self.graphics.draw_line(
                    self.registers[start_x as usize], 
                    self.registers[start_y as usize] + y as u8, 
                    line as usize, 16) 
                {
                self.registers[FLAG] = 1;
            }
        }

        self.graphics.show();
    }

    /* load extended sprite 4x10 pixels */
    fn load_extended_sprite(&mut self, reg:u8) {
        self.index_reg = (0x50 + (0xA * self.registers[reg as usize])) as u16;
    }

    fn store_hp_regs(&mut self, max_reg:u8) {
        let regs =  (&self.registers[.. max_reg as usize + 1]).iter();
        let store = (&mut self.hp_48_flags).iter_mut();
        /* itterate through both hp registers and general registers*/
        for (hp_reg, reg) in store.zip(regs) {
            *hp_reg = *reg;
        }

    }

    fn load_hp_regs(&mut self, max_reg:u8) {
        let regs = (&mut self.registers[(max_reg as usize + 1) ..]).iter_mut();
        let store = (&self.hp_48_flags).iter();
        /* itterate through both memory and registers */
        for (hp_reg, reg) in store.zip(regs) {
            *reg = *hp_reg;
        }
        
    }
}



#[cfg(test)]
mod tests {
    use super::CPU;
    use std::iter;

fn setup_blank_cpu() -> CPU {
    CPU::new(iter::repeat(0u8).take(0x1).collect()) 
}


#[test]
fn check_address_converter() {
    let get_addr = CPU::to_addr;
    assert_eq!(get_addr(0x1, 0x2, 0x3), 0x123);
    assert_eq!(get_addr(0xF, 0xF, 0xF), 0xFFF);
    assert_eq!(get_addr(0x0, 0x0 ,0x0), 0x000);
}


#[test]
fn check_value_converter() {
    let get_val = CPU::to_val;
    assert_eq!(get_val( 0xF, 0xF), 0xFF);
    assert_eq!(get_val( 0x0, 0x0), 0x0);
    assert_eq!(get_val( 0x9, 0x5), 0x95); 
} 

#[test]
fn check_hex_to_digits_converter() {
    let get_vec = CPU::u16_to_hex_vec;
    assert_eq!(get_vec(0xFFFF), (0xF, 0xF, 0xF, 0xF));
    assert_eq!(get_vec(0x0000), (0x0, 0x0, 0x0, 0x0));
    assert_eq!(get_vec(0x0456), (0x0, 0x4, 0x5, 0x6));
}


#[test]
fn check_add_regs() {
    let mut cpu = setup_blank_cpu();    
    cpu.interpret(0x6205); /* Load 0x5 into reg 2 */
    cpu.interpret(0x6510); /* Load 0x10 into reg 5 */
    cpu.interpret(0x8254); /* Add reg 5 to reg 2 and store in reg 2 */
    
    assert_eq!(cpu.get_reg(0xF), 0x0);
    assert_eq!(cpu.get_reg(2), 0x15);
}

#[test]
fn check_add_regs_overlow_value() {
    let mut cpu = setup_blank_cpu();
    cpu.interpret(0x61FF); /* Load FF into reg 1 */ 
    cpu.interpret(0x62FF); /* Load FF into reg 2 */
    cpu.interpret(0x8124); /* Add reg2 to reg1 and store in reg 1 */

    assert_eq!(cpu.get_reg(0xF),  0x1); /* Check flag set */
    assert_eq!(cpu.get_reg(1), 0xFE); 
}

/*** Test opcode 8XY5 ***/

/* Check a subtraction which results
 * in a value greater than zero */
#[test]
fn check_sub_regs() {
    let mut cpu = setup_blank_cpu();
    cpu.interpret(0x64F0); /* Load F0 into reg 4 */
    cpu.interpret(0x6301); /* load 1 into reg 3 */
    cpu.interpret(0x8435); /* sub reg 3 from reg 4 and store in reg 4 */

    assert_eq!(cpu.get_reg(0xF), 0x1); /* check no overflow */
    assert_eq!(cpu.get_reg(4), 0xF0 - 0x1);

}

/* Check a subtraction which results
 * in a values less than zero */
#[test]
fn check_sub_regs_overflow() {
    let mut cpu = setup_blank_cpu();
    cpu.interpret(0x6712); /* Load 0x12 into reg 7 */
    cpu.interpret(0x6920); /* Load 020 into reg 9 */
    cpu.interpret(0x8795); /* sub r9 from r7 store result in r7 */

    assert_eq!(cpu.get_reg(0xF), 0x0); /* check overflow */
    assert_eq!(cpu.get_reg(0x7), 0x12 - 0x20 as u8);

}


/*** Test opcode 8XY7, subbing registers ***
 *** the other way round than from 8XY5  ***/


/* Check a subtraction which results
 * in a value greater than zero */
#[test]
fn check_sub_inverted_regs() {
    let mut cpu = setup_blank_cpu();
    cpu.interpret(0x6A23);
    cpu.interpret(0x6B30);
    cpu.interpret(0x8AB7);

    assert_eq!(cpu.get_reg(0xF), 0x1);
    assert_eq!(cpu.get_reg(0xA), 0x30 - 0x23);
}



/* Check a subtraction which results
 * in a value less than zero */
#[test]
fn check_sub_inverted_regs_overflow() {
    let mut cpu = setup_blank_cpu();
    cpu.interpret(0x6CD3);
    cpu.interpret(0x6E30);
    cpu.interpret(0x8CE7);

    assert_eq!(cpu.get_reg(0xF), 0x0);
    assert_eq!(cpu.get_reg(0xC), 0x30 - 0xD3 as u8); 
}



#[test]
fn check_index_add_reg() {
    let mut cpu = setup_blank_cpu();
    cpu.interpret(0xA111); //set I to 0x111
    cpu.interpret(0x6120); // set reg 1 to 20
    cpu.interpret(0xF11E); // Add reg 1 to I

    assert_eq!(cpu.get_reg(0xF), 0x0);
    assert_eq!(cpu.get_index_reg(), 0x20 + 0x111);
}



#[test]
fn check_index_add_reg_overflow() {
    let mut cpu = setup_blank_cpu();
    cpu.interpret(0xAFFF); //set I to 0xFFF
    cpu.interpret(0x6C56); // set reg C to 0x56
    cpu.interpret(0xFC1E); // Add reg C to I

    assert_eq!(cpu.get_reg(0xF), 0x1);
    assert_eq!(cpu.get_index_reg(), (0xFFF + 0x056) % 0x1000);
}



/*** Jump instructions */

/*** Check instruction 3XNN ***/

/* Check skip occurs if
 * reg equals value */
#[test]
fn check_skip_reg_equals_val() {
    let mut cpu = setup_blank_cpu();
    cpu.interpret(0x6544); /* set reg 5 to 0x44 */
    let before_pc = cpu.get_pc();
    cpu.interpret(0x3544);
    assert_eq!(cpu.get_pc(), before_pc + 4);
}



/* Check skip doesn't occur
 * if reg doesn't equal value */
#[test]
fn check_no_skip_reg_equals_val() {
    let mut cpu = setup_blank_cpu();
    cpu.interpret(0x6067); /* set reg 0 to 0x67 */
    let before_pc = cpu.get_pc();
    cpu.interpret(0x3081);
    assert_eq!(cpu.get_pc(), before_pc + 2);
}

/*** Check instruction 4XNN ***/


/*Check skip occurs if
 * reg not equal to value */
#[test]
fn check_skip_reg_not_equals_val() {
    let mut cpu = setup_blank_cpu();
    cpu.interpret(0x6794); /* set reg 7 to 0x94 */
    let before_pc = cpu.get_pc();
    cpu.interpret(0x4700);
    assert_eq!(cpu.get_pc(), before_pc + 4);

} 



/* Check skip doesn't occur if
 * reg  equals value  */
#[test]
fn check_no_skip_reg_not_equals_val() {
    let mut cpu = setup_blank_cpu();
    cpu.interpret(0x6DAB); /* set reg D to 0xAB */
    let before_pc = cpu.get_pc();
    cpu.interpret(0x4DAB);
    assert_eq!(cpu.get_pc(), before_pc + 2);
}

/*** Check instruction 5XY0 ***/

/* Check skip occurs if
 * registers contain the same value */
#[test]
fn check_skip_regs_equal() {
    let mut cpu = setup_blank_cpu();
    cpu.interpret(0x6CF2); /* set reg C to 0xF2 */
    cpu.interpret(0x69F2); /* set reg 9 to 0xF2 */
    let before_pc = cpu.get_pc();
    cpu.interpret(0x5C90);
    assert_eq!(cpu.get_pc(), before_pc + 4);
}

/* Check skip doesn't occur
 * if registers don't contain
 * the same value */
#[test]
fn check_no_skip_regs_equal() {
    let mut cpu = setup_blank_cpu();
    cpu.interpret(0x6A71); /* set reg A to 0x71 */
    cpu.interpret(0x69BA); /* set reg 9 to 0xBA */
    let before_pc = cpu.get_pc();
    cpu.interpret(0x5A90);
    assert_eq!(cpu.get_pc(), before_pc + 2);
}




/*** Check instruction 9XY0 ***/

/* Check skip occurs when regsisters
 * not equal */
#[test]
fn check_skip_regs_not_equal() {  
    let mut cpu = setup_blank_cpu();
    cpu.interpret(0x6661); /* set reg 6 to 0x66 */
    cpu.interpret(0xEEE); /* set reg E to 0xEE */
    let before_pc = cpu.get_pc();
    cpu.interpret(0x96E0);
    assert_eq!(cpu.get_pc(), before_pc + 4);
}


/* Check skip doesn't occur when registers
 * are equal */
#[test]
fn check_no_skip_regs_not_equal() {
    let mut cpu = setup_blank_cpu();
    cpu.interpret(0x6555); /* set reg 5 to 0x55 */
    cpu.interpret(0x6DDD); /* set reg D to 0xDD */
    let before_pc = cpu.get_pc();
    cpu.interpret(0x5D0);
    assert_eq!(cpu.get_pc(), before_pc + 2);

}


/*** Check instruction BNNN ***/

#[test]
fn check_address_add_r0() {
    let mut cpu = setup_blank_cpu();
    cpu.interpret(0x6043);
    cpu.interpret(0xB151);
    assert_eq!(cpu.get_pc(), 0x043 + 0x151);
    
}


#[test]
/*** Check instruction 1NNN ***/
fn check_jump() {
    let mut cpu = setup_blank_cpu();
    cpu.interpret(0x1FFF);
    assert_eq!(cpu.get_pc(), 0xFFF);
}


#[test]
/*** Check instruction 2NNN ***/
fn check_call() {
    let mut cpu = setup_blank_cpu();
    cpu.interpret(0x2342);
    assert_eq!(cpu.get_pc(), 0x342);
}



#[test]
fn check_return() {
    let mut cpu = setup_blank_cpu();
    let before_pc = cpu.get_pc();
    cpu.interpret(0x2268);
    cpu.interpret(0x00EE);
    assert_eq!(cpu.get_pc(), before_pc + 2);

}


}


