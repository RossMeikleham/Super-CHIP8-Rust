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
        
        println!("size {:d}, {:d}",width, height);
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


    pub fn show(&self) {
        self.surface.flip();
    }
    
}



/*
  // Note: You'll want to put this and the flip call inside the main loop
  // but we don't as to not startle epileptics
  for i in range(0, 10) {
      for j in range(0, 10) {
          screen.fill_rect(Some(sdl::Rect {
              x: (i as i16) * 800 / 10,
              y: (j as i16) * 600 / 10,
              w: 800 / 10,
              h: 600 / 10
          }), rng.gen::<sdl::video::Color>());
      }
  }

  screen.flip();

  'main : loop {
      'event : loop {
          match sdl::event::poll_event() {
              sdl::event::QuitEvent => break 'main,
              sdl::event::NoEvent => break 'event,
              sdl::event::KeyEvent(k, _, _, _)
                  if k == sdl::event::EscapeKey
                      => break 'main,
              _ => {}
          }
      }
  }

  sdl::quit();
}
*/
