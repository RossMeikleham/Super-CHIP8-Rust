extern crate sdl;
extern crate libc;

use libc::uint8_t;
use std::ptr::RawPtr;

pub struct IOImpl {
    keyboard_state: *uint8_t,
    key_set: [char, ..16]
}

fn index(keyset:[char, ..16], key:char) -> Option<u8> {
    for i in range(0u, keyset.len()) {
        if keyset[i] == key {
            return Some(i as u8);
    }      
    }   

    None
}


impl IOImpl {
    
    pub fn new(key_set: [char, ..16]) -> IOImpl {
        let a = 0;
        IOImpl { keyboard_state: unsafe {sdl::event::ll::SDL_GetKeyState(&a)},
                  key_set: key_set,
                }
    }



    pub fn get_key(&mut self) -> u8 {
        'key_loop : loop {
            match sdl::event::wait_event() {
            sdl::event::KeyEvent(k , _, _, _) =>  {
                match index(self.key_set, ((k as u8) as char)) {
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
