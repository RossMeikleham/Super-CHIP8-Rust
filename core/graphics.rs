extern crate graphics_impl;
use std::bool;


static MAX_HORIZONTAL_PIXELS : uint = 128;
static MAX_VERTICAL_PIXELS : uint = 64;


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
    

    pub fn draw_pix(&mut self, x:uint, y:uint, set:bool) {
        self.screen[y][x] = set;
        self.out.draw_pix(x as int, y as int, set);
    }
        

    /* Draws a given line of 8 pixels starting at startx, starty,
     * returns whether a pixel was unset or not */
    pub fn draw_8_pix(&mut self, startx:u8, starty:u8, line:u8) -> bool {
        let mut unset_occured = false;
        for i in range(0, 8) {

            let x = ((startx + i as u8) as uint) % self.mode.get_width();
            let y = (starty as uint) % self.mode.get_height();
            let pix_state = if (line & (0x80 >> i)) != 0 {true} else {false};

            /* get set value of current pixel in line */
            let set = pix_state ^ self.screen[y][x];
            self.draw_pix(x, y, set);  
            unset_occured = unset_occured || (pix_state && (!set)); 
        }
       
        return unset_occured;
    }



    pub fn scroll_right(&mut self, n:u8) {
        for y in range(0, self.mode.get_height()) {
            /*TODO research how to properly use a decreasing
             * iterator in a range instead of this solution */
            for x1 in range(-(self.mode.get_width() - 1), - (n as uint - 1)) {
                let x = -x1;
                let set = self.screen[x - n as uint][y];
                self.draw_pix(x, y, set);            
            }
            for x  in range(0, n as uint) { 
                self.draw_pix(x, y, false);
            }
        }
    }

    pub fn scroll_left(&mut self, n:u8) {
        let x_max = self.mode.get_width();

        for y in range(0 , self.mode.get_height()) {
            for x in range(0, x_max - n as uint) {
                let set = self.screen[x + n as uint][y];
                self.draw_pix(x, y, set);
            }
            for x in range(x_max - n as uint, x_max) {
                self.draw_pix(x, y, false);
            }
        }
    }

    pub fn scroll_down(&mut self, n:u8) {
        let y_max = self.mode.get_height();
        for x in range(0, self.mode.get_width()) {
            for y in range(0, y_max - n as uint) {
                let set = self.screen[x][y + n as uint];
                self.draw_pix(x, y, set);
            } 
            
            for y in range(y_max - n as uint, y_max) {
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

    pub fn show(&self) {
        self.out.show();
    }
}

