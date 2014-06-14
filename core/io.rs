
static default_keys : [char, ..16] = ['z', 'x', 'c', 'v',
                                          'a', 's', 'd', 'f',
                                          'q', 'w', 'e', 'r',
                                          '1', '2', '3', '4'];

pub struct IO {
    keys : [char, ..16] 
}

impl IO {
    pub fn new() -> IO {
       IO { keys:default_keys }
        
    }
    
    pub fn wait_for_key(&self) -> u8 {
        0
    }

    pub fn is_key_pressed(&self, key_index:u8) -> bool {
        false
    }
}
