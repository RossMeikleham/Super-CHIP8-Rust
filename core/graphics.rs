extern crate graphics_impl;

use std::num::Bounded;
use std::iter::range_step;

const MAX_HORIZONTAL_PIXELS : uint = 128;
const MAX_VERTICAL_PIXELS : uint = 64;


struct Dimensions {
    width:  uint,
    height: uint
}

static SCHIP_dimensions :Dimensions = Dimensions{ width: 128, height: 64};
static  CHIP_dimensions :Dimensions = Dimensions{ width: 64,  height: 32};  


/* Either CHIP or Super CHIP mode */
pub enum Mode { CHIP,
                SCHIP}


impl Mode {
    

    fn get_width(&self) -> uint {
        match *self {
            CHIP => CHIP_dimensions.width,
            SCHIP => SCHIP_dimensions.width
        }
    }
    
    fn get_height(&self) -> uint {
        match *self {
            CHIP  =>  CHIP_dimensions.width,
            SCHIP => SCHIP_dimensions.height
        }
    } 
}



pub struct Graphics {
    mode :Mode,
    screen : [[bool, ..MAX_HORIZONTAL_PIXELS], ..MAX_VERTICAL_PIXELS],
    out : graphics_impl::Screen
}

impl Graphics {

    pub fn new() -> Graphics {
        Graphics { mode: CHIP,
                   /* Initialize all pixels to blank */
                   screen: [[false, ..MAX_HORIZONTAL_PIXELS], ..MAX_VERTICAL_PIXELS],
                   out: graphics_impl::Screen::new(256, 128, 
                    CHIP_dimensions.width, CHIP_dimensions.height)
         }
    }

    pub fn set_mode(&mut self, mode:Mode) { 
        self.mode = mode;
        self.out.set_x_max(mode.get_width());
        self.out.set_y_max(mode.get_height());
    }
    

    pub fn draw_pix(&mut self, x:uint, y:uint, state:bool) {
        self.screen[y][x] = state;
    }


    /* Create a bit vector for the supplied number from
     * MS bit to LS bit */
    fn to_bit_vec<N: Unsigned + Int>(num :N) -> Vec<u8> {
        let max : N = Bounded::max_value();

        let max_u = match max.to_uint() {
            Some(val) => val,
            None => fail!("cannot convert num to uint")
        };

        let bit_count = max_u.count_ones();
        let largest_bit = max_u - (max_u >> 1);

        let num_u = match num.to_uint() {
            Some(val) => val,
            None => fail!("cannot convert num to uint")
        };
        Vec::from_fn(bit_count, 
            |idx| if num_u & (largest_bit >> idx ) != 0 {1u8} else {0u8} )            
    }


    
    pub fn draw_line<N: Unsigned + Int>(&mut self, startx:u8, starty:u8, line:N) -> bool {       
      
       let mut unset_occured = false;

       let  pixel_states :Vec<bool>  = Graphics::to_bit_vec(line)
                       .iter()
                       .map(|&x| if x == 0 {false} else {true})
                       .collect();
      
        let current_states = self.screen[starty as uint]
                        .mut_slice_from(startx as uint);
        
        let mut zipped_states = current_states.mut_iter().zip(pixel_states.iter());
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
        let n = n as uint;
        for y in range(0, self.mode.get_height()) {
            for x in range_step(self.mode.get_width() - 1, n  - 1, -1) {
                let set = self.screen[y][x - n];
                self.draw_pix(x, y, set);            
            }
            for x  in range(0, n) { 
                self.draw_pix(x, y, false);
            }
        }
    }

    pub fn scroll_left(&mut self, n:u8) {
        let x_max = self.mode.get_width();
        let n = n as uint;
        for y in range(0 , self.mode.get_height()) {
            for x in range(0, x_max - n) {
                let set = self.screen[y][x + n];
                self.draw_pix(x, y, set);
            }
            for x in range(x_max - n, x_max) {
                self.draw_pix(x, y, false);
            }
        }
    }

    pub fn scroll_down(&mut self, n:u8) {
        let y_max = self.mode.get_height();
        let n = n as uint;
        for x in range(0, self.mode.get_width()) {
            for y in range_step(y_max - 1, n - 1, -1) {                
                let set = self.screen[y - n][x];
                self.draw_pix(x, y, set);            
            }
            for y  in range(0, n) { 
                self.draw_pix(x, y, false);
            }
        }
    }

    pub fn clear_screen(&mut self) {
        for y in range(0, MAX_VERTICAL_PIXELS) {
            for x in range(0, MAX_HORIZONTAL_PIXELS) {
                self.screen[y][x] = false;
            }
        }
        self.out.clear_screen();
    }
    
    //TODO improve as iinefficient (causes redrawing of entire screen)
    pub fn show(&mut self) {
        for y in range(0, MAX_VERTICAL_PIXELS) {
            for x in range(0, MAX_HORIZONTAL_PIXELS) {
                self.out.draw_pix(x as int, y as int, self.screen[y][x]);
            }
        }
        self.out.show();
    }
}
