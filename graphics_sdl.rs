extern crate sdl;
   

struct Screen {
    surface sdl::video::Surface,
    on_color : sdl::video::Color,
    off_color :sdl::video::Color,
    x_max :uint,
    y_max :uint

}

impl Screen {

    fn new(width :uint, height :uint, x_max :uint, y_max :uint) -> Screen {
        sdl::init([sdl::InitVideo]);
        sdl::wm::set_caption("CHIP-8 Emulator");  
        
        let surface =   
            match sdl::video::set_video_mode(width, height, 32, 
            [sdl::video::HWSurface], [sdl::video::DoubleBuf]) {
                
                Ok(screen) => screen,
                Err(err) => fail!("failed to set video mode: {}", err)
            };

        Screen { surface:surface, 
                 on_color:sdl::video::Color  { r:0, g:255, b:0}, 
                 off_color:sdl::video::Color { r:0, g:0,   b:0},
                 x_max:x_max,
                 y_max:y_max 
               }
        }
   }

    fn draw_pix(&mut self, x_pos :uint, y_pos :uint, set:bool) {
        let x_unit = screen.ll.w/screen.x_max;
        let y_uint = screen.ll.h/screen.y_max;

        self.screen.fill_rect(Some(sdl:Rect {
            x: x_pos * x_unit,
            y: y_pos * y_unit,
            w: x_unit,
            h: y_unit,
        }), match set { true => self.on_color, false => self.off_color }); 
    }

    pub fn clear_screen(&mut self) {
        self.screen.fill_rect(Some(sdl:Rect {
            x: 0,
            y: 0,
            w: self.ll.w,
            h: self.ll.h,
        }), self.off_color);    
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
