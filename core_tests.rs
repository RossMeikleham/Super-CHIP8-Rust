use core::CPU;
mod core;



fn setup_blank_cpu() -> CPU {
    CPU::new(Vec::from_elem(0x1, 0u8))
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
    assert_eq!(cpu.get_reg(0x7), (0x12 - 0x20) as u8);

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
    assert_eq!(cpu.get_reg(0xC), (0x30 - 0xD3) as u8); 
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


















