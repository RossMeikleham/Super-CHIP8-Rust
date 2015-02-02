extern crate time;

use std::old_io::File; /* input/output */
use std::string::String;
use std::os;
use system::CPU;
use std::old_io::timer;
use std::time::duration::Duration;
use std::iter;


mod system;


static MAX_RAM : usize = 0x1000;
static START_RAM :usize = 0x200;

static INSTRUCTIONS_PER_SEC : u64 = 500;
static CYCLES_CHECK : u64 = 5;

/* Reads a ROM file and returns a vector containing
 * its bytes if successful. Otherwise an error string
 * if the file is too large or IoError is raised */
fn read_rom(file_path: String) -> Result<Vec<u8>,String> {
    
    match File::open(&Path::new(file_path)).read_to_end() {

        Ok(rom_contents) =>  {
            /* Programs start at address 0x200, in original implementation
             * 0x000 - 0x1FF reserved for VM, just pad start of mem with 
             * 0s in this case */
            let start_mem : Vec<u8>  = iter::repeat(0u8).take(START_RAM).collect();
            let mem = start_mem + rom_contents.as_slice();
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


fn wait_for_next_cycle(old_time:u64, instructions:u64, ins_per_sec:u64 )  {
    let current_time = time::precise_time_ns();
    if old_time < current_time {
        let expired_ns = current_time - old_time;    
        let overall_duration_ns = (1000000000 * instructions)/ins_per_sec; /*Calculate duration instruction should take */
        if overall_duration_ns > expired_ns { /* ensure that duration has expired until next cycles begin */
            timer::sleep(Duration::nanoseconds((overall_duration_ns - expired_ns) as i64));            
       }
    } 
}


fn run_program(mut chip8 :system::CPU, cycle_max: u64, ins_per_sec: u64)  {
    
    'run : loop {
        let start_timer = time::precise_time_ns();
        for _ in range(0, cycle_max) {
            chip8.perform_cycle();
            /* Check if execution is finished */
            if chip8.is_finished() {
                break 'run;
            }
        }
        wait_for_next_cycle(start_timer, cycle_max, ins_per_sec);
    }
}


fn main() {
    let mut args = os::args();   
    let file_name = args.remove(1);

    let memory = match read_rom(file_name) {
        Ok(mem) => mem,
        Err(e) => panic!("{}",e)
    };
    
    assert!(memory.len() <=  MAX_RAM);

    run_program(system::CPU::new(memory), CYCLES_CHECK, INSTRUCTIONS_PER_SEC);
}






