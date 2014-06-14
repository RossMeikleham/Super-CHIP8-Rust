use std::rand::random;
use std::bool;
use core::graphics::Graphics; 
use core::io::IO;

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
    hp_48_flags: [u16, ..8], /*SCHIP */
    opcode : u16,
    graphics :Graphics,
    io :IO

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

  pub fn new(mut mem: Vec<u8>) -> CPU {
        let mut cpu = CPU { registers: [0u8, ..16], 
              mem: [0u8, ..MAX_RAM],
              I: 0,
              pc: 0x200,
              sp: 1,
              stack : [0u16, ..16],
              sound_timer: 0,
              delay_timer: 0,
              hp_48_flags: [0u16, ..8],
              opcode : 0,
              graphics : Graphics::new(),
              io : IO::new()
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
     
    /* converts 3 hex digits into a 12 bit address */    
    fn to_addr(dig1 :u8, dig2 :u8, dig3 :u8) -> u16 {
        ((dig1 << 8) | (dig2 << 4) | dig3) as u16
    }  

    /* combines 2 hex digits into a 8 bit value */
    fn to_val(dig1 :u8, dig2 :u8) -> u8 {
        (dig1 << 4) | dig2
    }

    /* takes a 12 bit value and extracts each hex digit
     * and stores in a tuple */ 
    fn u16_to_hex_vec(hex :u16) -> (u8, u8, u8, u8) {
         (((hex & 0xF000) >> 12) as u8,  
          ((hex & 0x0F00) >> 8)  as u8,
          ((hex & 0x00F0) >> 4)  as u8, 
           (hex & 0x000F)        as u8) 
    }

    /* obtains the current 16 bit opcode from memory */
    fn get_opcode(&self) -> u16 {
        (self.registers[self.pc as uint] << 4) as u16 |
        (self.registers[(self.pc + 1) as uint]) as u16
    } 

    /* perform 1 CPU instruction */
    pub fn perform_cycle(&mut self) {
        
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
            (0xD, X, Y, N) => self.draw_sprite(X, Y, N),
            (0xE, X, 0x9, 0xE) => self.skip_key_pressed(X),
            (0xE, X, 0xA, 0x1) => self.skip_not_key_pressed(X),
            (0xF, X, 0x0, 0x7) => self.set_reg_delay(X),
            (0xF, X, 0x0, 0xA) => self.wait_for_key(X),
            (0xF, X, 0x1, 0x5) => self.set_delay_reg(X),
            (0xF, X, 0x1, 0x8) => self.set_sound_reg(X),
            (0xF, X, 0x1, 0xE) => self.add_reg_index(X),
            (0xF, X, 0x2, 0x9) => self.load_sprite(X),
            (0xF, X, 0x3, 0x3) => self.binary_decimal(X),
            (0xF, X, 0x5, 0x5) => self.store_regs(X),
            (0xF, X, 0x6, 0x5) => self.load_regs(X),
            _ => fail!("Unknown opcode {:x}",opcode)
        }
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
        self.push(self.pc);
        self.pc = addr & 0xFFF;
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
        self.registers[reg1 as uint] |= self.registers[reg2 as uint];
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
            bool::to_bit::<u8>(0xFF - register1 > register2);
        self.registers[reg1 as uint] += register2;  
    }

    /* subtract the value of the second register from the first register 
     * if causes negative overflow unset the flag register, otherwise
     * set flag register, performs flag setting before subtraction
     * takes place.
     * Store subtraction result in first register*/
    fn sub_regs(&mut self, reg1:u8, reg2:u8) {
        let register1 = self.registers[reg1 as uint];
        let register2 = self.registers[reg2 as uint];
        self.registers[FLAG] = bool::to_bit::<u8>(register2 < register1);
        self.registers[reg1 as uint] -= register2; 
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
        self.pc = self.registers[0] as u16 + addr;
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
        self.I += self.registers[reg as uint] as u16
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

    fn draw_sprite(&mut self, x:u8, y:u8, n:u8) {
        self.registers[FLAG] = 0;
        for i in range(0, n + 1) {
            if self.graphics.draw_8_pix(x, y + i, 
                self.mem[(self.I + (i as u16)) as uint]) {

                self.registers[FLAG] = 1;
            }
        }

        self.graphics.show();
    }

    fn load_sprite(&mut self, reg:u8) {
        self.I = (5 * self.registers[reg as uint]) as u16;
    }

    fn wait_for_key(&mut self, reg:u8) {
        self.registers[reg as uint] = self.io.wait_for_key();  
    }
    
    fn skip_key_pressed(&mut self, reg:u8) {
        if self.io.is_key_pressed(self.registers[reg as uint]) {
            self.inc_pc();
        }
    }   

    fn skip_not_key_pressed(&mut self, reg:u8) {
        if !self.io.is_key_pressed(self.registers[reg as uint]) {
           self.inc_pc();
        }
    
    }


}
