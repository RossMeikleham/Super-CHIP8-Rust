extern crate sdl;

pub struct IOImpl { 
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
        IOImpl {  key_set: key_set}
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


    fn is_pressed(keyboard_state : Vec<(sdl::event::Key, bool)>,  key: char) -> bool {
        for i in keyboard_state.iter() {
            match *i { 
                (k, state) if (k as u8) == (key as u8) => return state,
                 _ => {} };
            }   
        false
    }

    pub fn key_pressed(&mut self, key:char) -> bool {
            sdl::event::pump_events(); /* Update current keyboard state */
            let state = sdl::event::get_key_state();
            IOImpl::is_pressed(state, key)
    }
}
