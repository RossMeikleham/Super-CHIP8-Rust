use std::rand::random;
use std::bool;
use core::graphics::Graphics; 
use core::graphics::Mode;
use core::io::IO;
use std::io::timer;
pub mod graphics;
pub mod io;
/* CPU, Graphics and Memory core */

static MAX_RAM : u16 = 4096;
static FLAG : uint = 15;

pub struct CPU {
     registers : [u8, ..16], /* 16 8 bit general purpose registers */
     mem : [u8, .. MAX_RAM], /* 4096bytes of memory */
     I : u16, /* 16 bit index register */
     pc: u16, /* Program counter */
     sp: uint, /* Stack Pointer */
     stack : [u16, ..16], /* 16 stack frames */
     sound_timer : u8, 
     delay_timer : u8,
     hp_48_flags: [u8, ..8], /*SCHIP */
     graphics :Graphics,
     io :IO,
     halt:bool,
     mode:Mode

}


static chip8_fontset: [u8, ..80] = //5x16
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
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
  ];
  
static schip8_fontset: [u8, ..160] = //10x16
  [
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
   

impl CPU {

  pub fn new(mem: Vec<u8>) -> CPU {
        let mut cpu = CPU { registers: [0u8, ..16], 
              mem: [0u8, ..MAX_RAM],
              I: 0,
              pc: 0x200,
              sp: 1,
              stack : [0u16, ..16],
              sound_timer: 0,
              delay_timer: 0,
              hp_48_flags: [0u8, ..8],
              graphics : Graphics::new(),
              io : IO::new(),
              halt:false,
              mode:Mode::get_CHIP() 
       };
        
       
       /* Fill up emulator emu values with supplied memory */
       for (m, v) in cpu.mem.mut_iter().zip(mem.iter()) {
           *m = *v;
       }
      
       /* Load CHIP8 fontset into unused locations 0x0 - 0x50 in memory */ 
       for (m, v) in cpu.mem.mut_iter().zip(chip8_fontset.iter()) {
            *m = *v;
       }
 
       return cpu;
    }

   pub fn get_mem(&self, loc:u16) -> u8 {
       self.mem[(loc % MAX_RAM) as uint]
   }

   pub fn get_reg(&self, reg:u8) -> u8 {
       self.registers[reg as uint]
   }

   pub fn get_pc(&self) -> u16 {
       self.pc
   }

   pub fn get_index_reg(&self) -> u16 {
       self.I
   }
     
    /* converts 3 hex digits into a 12 bit address */    
    pub fn to_addr(dig1 :u8, dig2 :u8, dig3 :u8) -> u16 {
        (dig1 as u16 << 8) | (dig2 as u16 << 4)  | dig3 as u16
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
        let x = (self.mem[self.pc as uint] as u16) << 8;

        (((self.mem[self.pc as uint]) as u16) << 8)  |
        (self.mem[(self.pc + 1) as uint]) as u16
    } 

    /* perform 1 CPU instruction */
    fn execute(&mut self, opcode:u16) {
        
        let opcode_v =  CPU::u16_to_hex_vec(opcode);
        self.inc_pc();

        match opcode_v {
            (0x0, 0x0 ,0xE, 0x0) => self.clear_screen(),
            (0x0, 0x0, 0xE, 0xE) => self.ret(), 
           
            (0x0, a, b, c) => {
                match (self.mode, (a, b, c)) {
                    (SCHIP, (0x0, 0xC, n)) => self.scroll_n_down(n),
                    (SCHIP, (0x0, 0xF, 0xB)) => self.scroll_4_right(),
                    (SCHIP, (0x0, 0xF, 0xC)) => self.scroll_4_left(),
                    (SCHIP, (0x0, 0xF, 0xD)) => self.exit(),
                    (SCHIP, (0x0, 0xF, 0xE)) => self.set_chip_mode(),
                    (SCHIP, (0x0, 0xF, 0xF)) => self.set_super_chip_mode(),
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

            (0xD, x, y, n) => match (self.mode, (x, y, n)) {
                (SCHIP, (x, y, 0x0)) => self.draw_extended_sprite(x, y),
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
                (SCHIP, (x, 0x3, 0x0)) => self.load_extended_sprite(x),
                (SCHIP, (x, 0x7, 0x5)) => self.store_hp_regs(x),
                (SCHIP, (x, 0x8, 0x5)) => self.load_hp_regs(x),
                _ => fail!("Unknown opcode {:x}", opcode)
            },

            _ => fail!("Unknown opcode {:x}",opcode)
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
        if self.registers[reg as uint] == val {
            self.inc_pc();
        }
                
    }
    /* Skip next instruction if register value is not equal
     * to supplied value */
    fn skip_not_equals_reg_val(&mut self, reg:u8, val:u8) {
        if self.registers[reg as uint] != val {
            self.inc_pc();
        }
    }

    /* Skip next instruction if both register values are equal */
    fn skip_equals_regs(&mut self, reg1:u8, reg2:u8) {
        
        if self.registers[reg1 as uint] == self.registers[reg2 as uint] {
               self.inc_pc();
        }
    }

    /* Skip next instruction if both register values are not equal */
    fn skip_not_equals_regs(&mut self, reg1:u8, reg2:u8) {
        if self.registers[reg1 as uint] != self.registers[reg2 as uint] {
               self.inc_pc();
           }
    }

    /* copy the supplied value into the register */
    fn mov_reg_val(&mut self, reg:u8, val:u8) {
        self.registers[reg as uint] = val;
    }

    /* add the supplied value to the value currently
     * stored in the register and store the result in the register */
    fn add_reg_val(&mut self, reg:u8, val:u8) {
        self.registers[reg as uint] += val;
    }
    
    /* Copy the value of the second register into the first register */
    fn mov_regs(&mut self, reg1:u8, reg2:u8) {
        self.registers[reg1 as uint] = self.registers[reg2 as uint];
    }

    /* perform a binary or on the values of the first and second
     * registers, store the result in the first register*/
    fn or_regs(&mut self, reg1:u8, reg2:u8) {
       self.registers[reg1 as uint] |= self.registers[reg2 as uint];
    }

    /* performa binary and on the values of the first and second
     * registers, store the result in the first register */
    fn and_regs(&mut self, reg1:u8, reg2:u8) {
        self.registers[reg1 as uint] &= self.registers[reg2 as uint];
    }

    /* perform binary xor on the values of the first and second
     * registers, store the result in the first register */
    fn xor_regs(&mut self, reg1:u8, reg2:u8) {
        self.registers[reg1 as uint] ^= self.registers[reg2 as uint];
    }

    /* Add two regs, if overflow set flag register otherwise
     * unset flag register */
    fn add_regs(&mut self, reg1:u8, reg2:u8) {
        let register1 = self.registers[reg1 as uint];
        let register2 = self.registers[reg2 as uint];
        self.registers[FLAG] =  
            bool::to_bit::<u8>(0xFF - register1 < register2);
        self.registers[reg1 as uint] += register2;  
    }


    /* subtract the value of the second register from the first register 
     * if causes negative overflow unset the flag register, otherwise
     * set flag register, performs flag setting before subtraction
     * takes place. Store result in "store_reg".*/
    fn sub_regs(&mut self, reg1:u8, reg2:u8, store_reg:u8) {
        let register1 = self.registers[reg1 as uint];
        let register2 = self.registers[reg2 as uint];
        
        self.registers[FLAG] = bool::to_bit::<u8>(register2 < register1);
        self.registers[store_reg as uint] = register1 -  register2;
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

    /* Set index register to supplied address */
    fn set_i(&mut self, addr:u16) {        
        self.I = addr;
    }

    /* jump to the supplied address + value in register 0 */
    fn jump_val_reg0(&mut self, addr:u16) {
        self.pc = (self.registers[0] as u16 + addr) % MAX_RAM;
    }

    /* set register to supplied value and a random integer between 0 and 255 */
    fn rand(&mut self, reg:u8, val:u8) {
        self.registers[reg as uint] = val & random::<u8>();
    }

    /* Set the delay timer to the value in the register */
    fn set_delay_reg(&mut self, reg:u8) {
        self.delay_timer = self.registers[reg as uint];
    }

    /* set the register to the value in the display timer */
    fn set_reg_delay(&mut self, reg:u8) {
        self.registers[reg as uint] = self.delay_timer;
    }
    
    /* set the sound timer to the value in the register */
    fn set_sound_reg(&mut self, reg:u8) {
        self.sound_timer = self.registers[reg as uint];
    }

    /* add the value in the register to value in the index register and store
     * the result in the index register. If this operation causes overflow
     * set the flag register, otherwise unset it */
    fn add_reg_index(&mut self, reg:u8) {
        self.registers[FLAG] = 
            bool::to_bit::<u8>(0xFFF - (self.registers[reg as uint] as u16) < self.I); 
        self.I += self.registers[reg as uint] as u16;
        self.I %= MAX_RAM;
    }

    /* store the values from register 0 up to and including
     * the supplied register number starting from memory location
     * pointed to by the index register */
    fn store_regs(&mut self, max_reg:u8) {
        let regs = self.registers.slice_to(max_reg as uint + 1).iter();
        let store = self.mem.mut_slice_from(self.I as uint).mut_iter();
        /* itterate through both memory and registers*/
        for (mem, reg) in store.zip(regs) {
            *mem = *reg;
        }

    }


    /* load values into register 0 up to and including
     * the supplied register number starting from memory location
     * pointed to by the index register */
    fn load_regs(&mut self, max_reg:u8) {
        let regs = self.registers.mut_slice_to(max_reg as uint + 1).mut_iter();
        let store = self.mem.slice_from(self.I as uint).iter();
        /* itterate through both memory and registers */
        for (mem, reg) in store.zip(regs) {
            *reg = *mem;
        }
   }


    /*stores the Binary-coded decimal representation of VX, with the
     * most significant of three digits at the address in I, the middle digit
     * at I + 1, and the LSD at I + 2.*/
    fn binary_decimal(&mut self, reg:u8) {
        let val = self.registers[reg as uint];
        self.mem[self.I as uint] = val/100;
        self.mem[(self.I + 1) as uint] = (val % 100)/10;
        self.mem[(self.I + 2) as uint] = (val % 100)%10;
 
    }
    
    /* Draw sprite starting at x,y which is n lines
     * of 8 pixels stored starting at memory location 
     * of the contents of register I*/
    fn draw_sprite(&mut self, x:u8, y:u8, n:u8) {
       
        self.registers[FLAG] = 0;
        for i in range(0, n) {
            if self.graphics.draw_8_pix(
                    self.registers[x as uint], 
                    self.registers[y as uint] + i,  
                    self.mem[(self.I + (i as u16)) as uint]) {

                self.registers[FLAG] = 1;
            } 
        }

        self.graphics.show();       
    }

    /* set I reg to sprite number stored in the given register */
    fn load_sprite(&mut self, reg:u8) {
        self.I = (5 * self.registers[reg as uint]) as u16;
    }

    /* Wait for a keypress and set the contents of the
     * given register to that keypress */
    fn wait_for_key(&mut self, reg:u8) {
        self.registers[reg as uint] = self.io.wait_for_key();  
    }
    
    /* if key in given register is being pressed then
     * skip the next instruction */
    fn skip_key_pressed(&mut self, reg:u8) {
        if self.io.is_key_pressed(self.registers[reg as uint]) {
            self.inc_pc();
        }
    }   

    /* if key in given register is not being pressed then
     * skip the next instruction */
    fn skip_not_key_pressed(&mut self, reg:u8) {
        if !self.io.is_key_pressed(self.registers[reg as uint]) {
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
        self.mode = Mode::get_CHIP();
        self.graphics.set_mode(self.mode);
    }
    
    fn set_super_chip_mode(&mut self) {
        self.mode = Mode::get_SCHIP();
        self.graphics.set_mode(self.mode);
    }


    /* Draw 16*16 sprite at x,y */
    fn draw_extended_sprite(&mut self, start_x:u8, start_y:u8) {

        for y in range(0u8, 16u8) {
            let line = ((self.mem[(2 * y) as uint] as u16) << 4) 
                | (self.mem[((2 * y) +1) as uint] as u16);
            self.graphics.draw_16_pix(start_x, start_y + y, line); 
        }
    }

    /* load extended sprite 4x10 pixels */
    fn load_extended_sprite(&mut self, reg:u8) {
        self.I = (0x50 + (0xA * self.registers[reg as uint])) as u16;
    }

    fn store_hp_regs(&mut self, max_reg:u8) {
        let regs = self.registers.slice_to(max_reg as uint + 1).iter();
        let store = self.hp_48_flags.mut_iter();
        /* itterate through both hp registers and general registers*/
        for (hp_reg, reg) in store.zip(regs) {
            *hp_reg = *reg;
        }

    }

    fn load_hp_regs(&mut self, max_reg:u8) {
        let regs = self.registers.mut_slice_to(max_reg as uint + 1).mut_iter();
        let store = self.hp_48_flags.iter();
        /* itterate through both memory and registers */
        for (hp_reg, reg) in store.zip(regs) {
            *reg = *hp_reg;
        }
        
    }
}
