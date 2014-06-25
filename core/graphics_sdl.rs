#![crate_id = "graphics_impl"]
extern crate sdl;

pub struct Screen {
    surface :sdl::video::Surface,
    on_color : sdl::video::Color,
    off_color :sdl::video::Color,
    width: uint,
    height: uint,
    x_max :uint,
    y_max :uint

}

impl Screen {

    pub fn new(width :int, height :int, x_max :uint, y_max :uint) -> Screen {
        sdl::init([sdl::InitVideo]);
        sdl::wm::set_caption("CHIP-8 Emulator", "sdl");  
        
        let surface =   
            match sdl::video::set_video_mode(width, height, 32, 
            [sdl::video::HWSurface], [sdl::video::DoubleBuf]) {
                
                Ok(screen) => screen,
                Err(err) => fail!("failed to set video mode: {}", err)
            };

        Screen { surface:surface, 
                 on_color:  sdl::video::RGB(0, 255, 0), /* Green */ 
                 off_color: sdl::video::RGB(0, 0,   0), /* Black */
                 width:width as uint,
                 height:height as uint,
                 x_max:x_max,
                 y_max:y_max 
               }
        }
  
    pub fn set_x_max(&mut self, x:uint) {
        self.x_max = x;
    }

    pub fn set_y_max(&mut self, y:uint) {
        self.y_max = y;
    }

    pub fn draw_pix(&mut self, x_pos :int, y_pos :int, set:bool) {
        let x_unit = (self.width/self.x_max) as u16;
        let y_unit = (self.height/self.y_max) as u16;

        self.surface.fill_rect(Some(sdl::Rect {
            x: x_pos as i16 * x_unit as i16,
            y: y_pos as i16 * y_unit as i16,
            w: x_unit,
            h: y_unit,
        }), match set { true => self.on_color, false => self.off_color });
        
    }

    pub fn clear_screen(&mut self) {
        self.surface.fill_rect(Some(sdl::Rect {
            x: 0,
            y: 0,
            w: self.width as u16,
            h: self.height as u16,
        }), self.off_color);   
    }


    pub fn show(&mut self) {
        
        self.surface.flip();
    }
    
}
