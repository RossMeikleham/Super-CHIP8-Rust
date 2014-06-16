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


/*#[test]
fn check_add_regs() {
    let mut cpu = setup_blank_cpu();
    let mut cpu2 = setup_blank_cpu();
    cpu.interpret(0x6205); /* Load 0x5 into reg 2 */
    cpu.interpret(0x6510); /* Load 0x10 into reg 5 */
    cpu.interpret(0x8254); /* Add reg 5 to reg 2 and store in reg 2 */
    cpu2.interpret(0x6205);
    assert_eq!(cpu.get_reg(0xF), 0x0);
    assert_eq!(cpu.get_reg(2), 0x15);
}*/

#[test]
fn check_add_regs_overlow_value() {
    let mut cpu = setup_blank_cpu();
    cpu.interpret(0x61FF); /* Load FF into reg 1 */ 
    cpu.interpret(0x62FF); /* Load FF into reg 2 */
    cpu.interpret(0x8124); /* Add reg2 to reg1 and store in reg 1 */

    assert_eq!(cpu.get_reg(0xF),  0x1); /* Check flag set */
    assert_eq!(cpu.get_reg(1), 0xFE); 
}





