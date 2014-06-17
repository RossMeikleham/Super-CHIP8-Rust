

#[test]
fn check_address_converter() {
    let get_addr = core::CPU::to_addr;
    assert!(get_addr(1, 2, 3) == 0x123);
    assert!(get_addr(0xF, 0xF, 0xF) == 0xFFF);
    assert!(get_addr(0, 0 ,0) == 0x000);
} 
