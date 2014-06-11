
mod graphics {

static MAX_HORIZONTAL_PIXELS : uint = 128;
static MAX_VERTICAL_PIXELS : uint = 64;


struct Dimensions {
    width:  uint,
    height: uint
}

static SCHIP_dimensions :Dimensions = Dimensions{ width: 128, height: 64};
static  CHIP_dimensions :Dimensions = Dimensions{ width: 64,  height: 32};  


/* Either CHIP or Super CHIP mode */
pub enum Mode { CHIP = CHIP_dimensions,
                SCHIP = SCHIP_dimensions }


pub struct Graphics {
    mode :Mode,
    screen : [[uint, ..MAX_VERTICAL_PIXELS], ..MAX_HORIZONTAL_PIXELS]
    
}

impl Graphics {

    pub fn new() -> Graphics {
        Graphics { mode: CHIP_dimensions,
                   /* Initialize all pixels to blank */
                   screen : Vec::from_elem(MAX_HORIZONTAL_PIXELS, 
                            Vec::from_elem(MAX_VERTICAL_PIXELS, 0))} 
    }

    fn set_mode(&mut self, mode:Mode) { 
        self.mode = mode;
    }
    

    fn draw_pix(&mut self, x:uint, y:uint, set:bool) {
        
        if set {self.screen[y][x] = 1;}
        else {  self.screen[y][x] = 0;}
    }

    pub fn draw_8_pix(&mut self, startx:uint, starty:uint, line:u8) {
        for i in range(0, 8) {

            let x = (startx + i) % self.width;
            let y = starty & self.height;
            let pix_bit = (line & (0x80 >> i)) >> (7 - i); /* get bit value of current pixel in line */

            /* pixel xor'd with current pixel value to obtain set or unset value */
            let set = match pix_bit ^ self.screen[y][x]  {
                0 => false,
                1 => true,
                _ => fail!("Error setting pixels")
            };
            self.draw_pix(x, y, set);
        }
    }
}

}
