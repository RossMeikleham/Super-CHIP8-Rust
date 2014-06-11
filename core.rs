use std::rand::random;
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


fn dec_u12(val:u16) -> uint {
    ((val - 1) % U12_MAX) as uint
}

fn dec_u12_n(val:u16, n:u16) -> uint {
    ((val - n) % U12_MAX) as uint
}

fn inc_u12(val:u16) -> uint {
    ((val + 1) % U12_MAX) as uint
}

fn inc_u12_n(val:u16, n:u16) -> uint {
    ((val + n) % U12_MAX) as uint
}


/* returns 1 if true, 0 if false */
fn bool_to_int(b:bool) -> u8 {
    match b {
        true => 1,
        false => 0
    }
}


impl CPU {
   
    fn new(mut mem: Vec<u8>) -> CPU {
        let mut cpu = CPU { registers: [0u8, ..16], 
              mem: [0u8, ..0xFFF],
              I: 0,
              pc: 0,
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
    
    fn to_addr(dig1 :u8, dig2 :u8, dig :u8) -> u16 {
        ((dig1 << 8) | (dig2 << 4) | dig3) as u16
    }  

    fn to_val(dig1 :u8, dig2 :u8) -> u8 {
        (dig1 << 4) | dig2
    }

    fn u16_to_hex_vec(hex :u16) => (u8, u8, u8, u8) {
         (hex & 0xF000) >> 12,  (hex & 0x0F00 >> 8),
         (hex & 0x00F0 >> 4, hex & 0x000F)
    }

    fn perform_cycle(&mut self) {
        
        opcode = u16_to_hex_vec(self.get_opcode());
        self.inc_pc();
        
        match opcode {
            (0x0, 0x0 ,0xE, 0x0) => self.clear_screen(),
            (0x0, 0x0, 0xE, 0xE) => self.ret(),
            (0x0, N1, N2, N3)  => {}, 
            (0x1, N1, N2, N3)  => self.jump(to_addr(N1, N2, N3)),
            (0x2, N1, N2, N3)  => self.call(to_addr(N1, N2, N3)),
            (0x3, X, N1, N2)  => self.skip_equals_reg_val(X, to_val(N1, N2)),
            (0x4, X, N, N)  => self.skip_not_equals_reg_val(X, to_val(N1, N2)),
            (0x5  X, Y, 0x0)  => self.skip_equals_regs(X, Y),
            (0x6  X, N1, N2)  => self.mov_reg_val(X, to_val(N1, N2)),
            (0x7, X, N1, N2)  => self.add_reg_val(X, to_val(N1, N2)),
            (0x8, X, Y, 0x0) => self.mov_regs(X, Y),
            (0x8, X, Y, 0x1) => self.or_regs(X, Y),
            (0x8, X, Y, 0x2) => self.and_regs(X, Y),
            (0x8, X, Y, 0x3) => self.xor_regs(X, Y),
            (0x8, X, Y, 0x4) => self.add_regs(X, Y),
            (0x8, X, Y, 0x5) => self.sub_regs(X, Y),
            (0x8, X, Y, 0x6) => self.shift_right(X, Y),
            (0x8, X, Y, 0x7) => self.sub_regs(Y, X),
            (0x8, X, Y, 0xE) => self.shift_left(X, Y),
            (0x9, X, Y, 0x0) => self.skip_not_equals_regs(X, Y),
            (0xA, N1, N2, N3) => self.set_i(to_addr(N1, N2, N3)),
            (0xB, N1, N2, N3) => self.jump_val_reg0(to_addr(N1, N2, N3)),
            (0xC, X, N1, N2) => self.rand(X, to_val(N1, N2)),
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
            (0xF, X, 0x6, 0x5) => self.load_regs(X) 
        }
    }
     

    fn regs_op(&mut self, op: |u8, u8| -> u8) {
        let reg1 = self.get_reg_no_1();
        let reg2 = self.get_reg_no_1();

        self.registers[reg1] = 
            op(self.registers[reg1], self.registers[reg2]);

    }

    fn regs_op_flags(&mut self, op: |u8, u8| -> u8, flag_val: |u8, u8| -> u8) {
        let reg1 = self.get_reg_no_1();
        let reg2 = self.get_reg_no_2();

        self.registers[FLAG] = 
            flag_val(self.registers[reg1], self.registers[reg2]);

        self.registers[reg1] = 
            flag_val(self.registers[reg1], self.registers[reg2]);
    }


    fn reg_val_op(&mut self, op: |u8, u8| -> u8) {
        let reg = self.get_reg_no_1();
        let val = self.get_val();

        self.registers[reg] = op(self.registers[reg], val)
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
    fn call(&mut self) {
        self.pc = self.opcode & 0xFFF;
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
        if self.registers[reg] == val {
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
    
    fn mov_regs(&mut self) {
        self.regs_op(|i, j| j);
    }

    fn or_regs(&mut self) {
        self.regs_op(|i, j| i | j); 
    }

    fn and_regs(&mut self) {
        self.regs_op(|i, j| i & j);
    }

    fn xor_regs(&mut self) {
        self.regs_op(|i, j| i ^ j);
    }

    /* Add two regs, if overflow set flag register otherwise
     * unset flag register */
    fn add_regs(&mut self) {
        self.regs_op_flags(|reg1, reg2| reg1 + reg2, 
                           |reg1, reg2| bool_to_int(0xFF - reg1 > reg2)); 
    }

    /* Sub reg2 from reg1, unset flag register if underflow,
     * set otherwise */
    fn sub_regs(&mut self) {
        self.regs_op_flags(|reg1, reg2| reg1 - reg2, 
                           |reg1, reg2| bool_to_int(reg2 < reg1)); 
      
    }

    /* Sub reg1 from reg2, unset flag register if underflow,
     * set otherwise */
    fn sub_regs_inv(&mut self) {
        self.regs_op_flags(|reg1, reg2| reg2 - reg1, 
                           |reg1, reg2| bool_to_int(reg1 < reg2));
    }


    /* Shift register left by 1, set flag register to most significant bit
     * before shifting */
    fn shift_left(&mut self) {
        self.regs_op_flags(|reg1, reg2| reg1 << 1, |reg1, reg2| reg1 & 0x80 >> 7);
    }

    /* Shift register right by 1, set flag register to least significant bit
     * before shifting */
    fn shift_right(&mut self) {
        self.regs_op_flags(|reg1, reg2| reg1 >> 1, |reg1, reg2| reg1 & 0x1);
    }

    /* Set index register to address in opcode */
    fn set_i(&mut self) {        
        self.I = self.get_addr();
    }

    fn jump_val_reg0(&mut self) {
        self.pc = self.registers[0] as u16 + self.get_addr();
    }

    fn rand(&mut self) {
        self.reg_val_op( |reg, val| val & random::<u8>());
    }

    fn set_reg_delay(&mut self) {
        self.registers[self.get_reg_no_1()] = self.delay_timer;
    }
    
    fn set_reg_sound(&mut self) {
        self.registers[self.get_reg_no_1()] = self.sound_timer;
    }

    fn add_reg_index(&mut self) {
        let temp = self.I;
        self.I += self.registers[self.get_reg_no_1()] as u16;
        if (temp > self.I)  {
            self.registers[FLAG] = 1
        } 
    }

    fn store_regs(&mut self) {
        let mut addr = self.I;
        for reg in self.registers.iter() {
            self.mem[addr as uint] = *reg;
            addr += 1;
        }
    }

    fn load_regs(&mut self) {
        let mut addr = self.I;
        for reg in self.registers.mut_iter() {
            *reg = self.mem[addr as uint];
            addr += 1;
        }
    }

    /*stores the Binary-coded decimal representation of VX, with the
     * most significant of three digits at the address in I, the middle digit
     * at I + 1, and the lsg at I + 2.*/
    fn binary_decimal(&mut self) {
        let val = self.registers[self.get_reg_no_1()];
        self.mem[self.I as uint] = val/100;
        self.mem[(self.I + 1) as uint] = (val % 100)/10;
        self.mem[(self.I + 2) as uint] = (val % 100)%10;
 
    }

}


fn main() {

   
    print!("yay");
}

