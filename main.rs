use std::io::File; /* input/output */

use std::os;

static MAX_RAM : uint = 4096;

fn read_game(file_path: ~str) -> Vec<u8> {
    
    let memory = match File::open(&Path::new(file_path)).read_to_end() {
        Ok(mem) => mem,
        Err(e) => fail!("{}",e)
    };

    let size = memory.len();
    if size > MAX_RAM {
        fail!("game image is too large ({} bytes), 
            must be a maximum of {} bytes", size, MAX_RAM);
    }

    return memory;
} 

fn main() {
    let mut args = os::args();
    //let file_name: &~str = args.get(1);
    //print!("reading file {}",file_name);
 
   
   let file_name = match args.remove(1)  {
       Some(name) => name,
       None => fail!("No file name specified")
   };

    print!("reading file {}",file_name);
    let memory = read_game(file_name);
    print!("read file {}",memory);
}

