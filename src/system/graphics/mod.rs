use std::iter::range_step;

pub mod graphics_sdl;

const MAX_HORIZONTAL_PIXELS : usize = 128;
const MAX_VERTICAL_PIXELS : usize = 64;


struct Dimensions {
    width:  usize,
    height: usize
}

static SCHIP_DIMENSIONS :Dimensions = Dimensions{ width: 128, height: 64};
static  CHIP_DIMENSIONS :Dimensions = Dimensions{ width: 64,  height: 32};  


// Enums are uncopyable wtf, now completely useless >.>
// Use bools instead to check CHIP or SCHIP mode
fn get_width(mode : bool) -> usize { 
    match mode {
        false => CHIP_DIMENSIONS.width,
        true => SCHIP_DIMENSIONS.width
    }
}

fn get_height(mode : bool) -> usize { 
    match mode {
        false => CHIP_DIMENSIONS.height,
        true => SCHIP_DIMENSIONS.height
    }
}


pub struct Graphics {
    mode :bool,
    screen : [[bool; MAX_HORIZONTAL_PIXELS]; MAX_VERTICAL_PIXELS],
    out : graphics_sdl::Screen
}

impl Graphics {

    pub fn new() -> Graphics {
        Graphics { mode: false,
                   /* Initialize all pixels to blank */
                   screen: [[false; MAX_HORIZONTAL_PIXELS]; MAX_VERTICAL_PIXELS],
                   out: graphics_sdl::Screen::new(256, 128, 
                    CHIP_DIMENSIONS.width, CHIP_DIMENSIONS.height)
         }
    }

    pub fn set_mode(&mut self, new_mode:bool) { 
        self.mode = new_mode;
        self.out.set_x_max(get_width(new_mode));
        self.out.set_y_max(get_height(new_mode));
    }
    

    pub fn draw_pix(&mut self, x:usize, y:usize, state:bool) {
        self.screen[y][x] = state;
    }


    /* Create a bit vector for the supplied number from
     * MS bit to LS bit */
    fn to_bit_vec(n:usize, bit_count:usize) -> Vec<u8> {
              
        let largest_bit : usize = 1 << (bit_count - 1); 

        (0 .. bit_count)
            .map(|idx| if n & (largest_bit >> idx ) != 0 {1u8} else {0u8}) 
            .collect()           
    }


    
    pub fn draw_line(&mut self, startx:u8, starty:u8, line:usize, bits:usize) -> bool {       
      
       let mut unset_occured = false;

       let  pixel_states :Vec<bool>  = Graphics::to_bit_vec(line, bits)
                       .iter()
                       .map(|&x| if x == 0 {false} else {true})
                       .collect();
      
        let current_states = &mut (self.screen[starty as usize % 64])[startx as usize ..];
                        
        
        let mut zipped_states = current_states.iter_mut().zip(pixel_states.iter());
        /* Set pixel to old pixel xor new pixel */
        for (old, new) in zipped_states { 
            if !unset_occured && *old == true && *new == true {
                unset_occured = true;
            } 
            *old = *old ^ *new; 
        }

        return unset_occured;;
   }
       


    pub fn scroll_right(&mut self, n:u8) {
        let n = n as usize;
        for y in 0 .. get_height(self.mode) {
            for x in range_step(get_width(self.mode) - 1, n  - 1, -1) {
                let set = self.screen[y][x - n];
                self.draw_pix(x, y, set);            
            }
            for x in 0 .. n { 
                self.draw_pix(x, y, false);
            }
        }
    }

    pub fn scroll_left(&mut self, n:u8) {
        let x_max = get_width(self.mode);
        let n = n as usize;
        for y in 0  .. get_height(self.mode) {
            for x in 0 .. (x_max - n) {
                let set = self.screen[y][x + n];
                self.draw_pix(x, y, set);
            }
            for x in (x_max - n .. x_max) {
                self.draw_pix(x, y, false);
            }
        }
    }

    pub fn scroll_down(&mut self, n:u8) {
        let y_max = get_height(self.mode);
        let n = n as usize;
        for x in 0 .. get_width(self.mode) {
            for y in range_step(y_max - 1, n - 1, -1) {                
                let set = self.screen[y - n][x];
                self.draw_pix(x, y, set);            
            }
            for y  in 0 .. n { 
                self.draw_pix(x, y, false);
            }
        }
    }

    pub fn clear_screen(&mut self) {
        for y in 0 .. MAX_VERTICAL_PIXELS {
            for x in 0 .. MAX_HORIZONTAL_PIXELS {
                self.screen[y][x] = false;
            }
        }
        self.out.clear_screen();
    }
    
    //TODO improve as iinefficient (causes redrawing of entire screen)
    pub fn show(&mut self) {
        for y in 0 .. MAX_VERTICAL_PIXELS {
            for x in 0 .. MAX_HORIZONTAL_PIXELS {
                self.out.draw_pix(x as isize, y as isize, self.screen[y][x]);
            }
        }
        self.out.show();
    }
}
