extern crate sdl;
extern crate libc;

use libc::uint8_t;
use libc::c_int;
use std::ptr::RawPtr;

pub struct IO_impl {
    keyboard_state: *uint8_t,
    size: c_int,
    key_set: [char, ..16]
}

fn index(keyset:[char, ..16], key:char) -> Option<u8> {
    for i in range(0u, keyset.len()) {
        if keyset[i] == key {
            Some(i);
    }      
    }   

    None
}


impl IO_impl {
    
    pub fn new(key_set: [char, ..16]) -> IO_impl {
        let a = 0;
        IO_impl { keyboard_state: unsafe {sdl::event::ll::SDL_GetKeyState(&a)},
                  key_set: key_set,
                  size :a }
    }



    pub fn get_key(&mut self) -> u8 {
        'key_loop : loop {
            match sdl::event::poll_event() {

            sdl::event::KeyEvent(k , _, _, _) =>  {
                match index(self.key_set, ((k as int) as u8) as char) {
                    Some(index) => { return index;},
                    None => {}
                }; 
            },

            _ => {}
            }
        }
    }


    pub fn key_pressed(&mut self, key:char) -> bool {
            unsafe {sdl::event::ll::SDL_PumpEvents(); /* Update keyboard state */
                    return *(self.keyboard_state.offset((key as u8) as int)) != 0; 
            }
    } 
 
}
