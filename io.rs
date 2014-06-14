

static default_keys : [char, ..16] = ('z', 'x', 'c', 'v',
                                     'a', 's', 'd', 'f',
                                     'q', 'w', 'e', 'r',
                                     '1', '2', '3', '4');

struct IO {
    keys : [char, ..16] 
}

impl IO {
    fn new() -> IO {
       IO { keys:default_keys }
        
    }
    
    fn wait_for_key() -> u8 {
        0
    }

    fn is_key_pressed() -> bool {
        false
    }
}
