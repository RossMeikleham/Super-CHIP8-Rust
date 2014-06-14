use std::io::File; /* input/output */
use std::string::String;
use std::os;
use core::CPU;

mod core;

static MAX_RAM : uint = 0xFFF;
static START_RAM :uint = 0x200;



/* Reads a ROM file and returns a vector containing
 * its bytes if successful. Otherwise an error string
 * if the file is too large or IoError is raised */
fn read_rom(file_path: String) -> Result<Vec<u8>,String> {
    
    match File::open(&Path::new(file_path)).read_to_end() {

        Ok(rom_contents) =>  {
            /* Programs start at address 0x200, in original implementation
             * 0x000 - 0x1FF reserved for VM, just pad start of mem with 
             * 0s in this case */
            let mem = Vec::from_elem(START_RAM, 0u8) + rom_contents;
            let size = mem.len();

            if size <= MAX_RAM { 
                    Ok(mem)                    
            } else { /* Memory read in from game ROM is too large */
                Err(format!("game image is too large 
                    ({} bytes), must be a maximum of {} bytes"
                    ,size , MAX_RAM - START_RAM).to_string())
            }
        },
        /* Error reading file */
        Err(e) => Err(e.detail.unwrap_or("".to_string()))       
    }
} 




fn main() {
    let mut args = os::args();
   
   let file_name = match args.remove(1)  {
       Some(name) => name,
       None => fail!("No file name specified")
   };

    //debug!("reading file {}",file_name);

    let memory = match read_rom(file_name) {
        Ok(mem) => mem,
        Err(e) => fail!("{}",e)
    };
    
    assert!(memory.len() <=  MAX_RAM);

    let mut chip8 = core::CPU::new(memory);
    loop {
        chip8.perform_cycle();
    }
}






