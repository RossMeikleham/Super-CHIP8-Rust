extern crate sdl_io;

static default_keys : [char, ..16] = ['z', 'x', 'c', 'v',
                                          'a', 's', 'd', 'f',
                                          'q', 'w', 'e', 'r',
                                          '1', '2', '3', '4'];

pub struct IO {
    keyboard: sdl_io::IO_impl,
    keys : [char, ..16]
}

impl IO {

    pub fn new() -> IO {        
       IO { keyboard: sdl_io::IO_impl::new(default_keys),
            keys: default_keys }

        
    }
    
    pub fn wait_for_key(&mut self) -> u8 {
        self.keyboard.get_key()
    }

    pub fn is_key_pressed(&mut self, key_index:u8) -> bool {
        self.keyboard.key_pressed(self.keys[key_index as uint])
    }
}
