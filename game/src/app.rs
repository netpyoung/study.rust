// ref: https://docs.rs/gl/0.11.0/gl/
extern crate gl;
// ref: https://rust-sdl2.github.io/rust-sdl2/sdl2/
extern crate sdl2;

// TODO(pyoung): change name.


trait HasArea {
    fn area(&self) -> f64;
}

#[derive(PartialEq)]
pub enum AppState {
    Running,
    Exiting,
}

pub struct App {
    pub state: AppState,
    pub window: Option<sdl2::video::Window>,
    pub event_pump: Option<sdl2::EventPump>,
    pub gl_context: Option<sdl2::video::GLContext>,
}

pub enum EventApp {
    Pass,
    Quit,
}

impl App {
    pub fn new() -> App {
        App {
            state: AppState::Running,
            window: Option::None,
            event_pump: Option::None,
            gl_context: Option::None,
        }
    }

    pub fn start(&mut self) {
        println!("start");

        let sdl = sdl2::init().unwrap();

        let video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 1);


        let mut builder_window = video_subsystem.window("Game", 300, 300);
        let builder_sdl = builder_window
            .position_centered()
            .resizable();
        let builder_opengl = builder_sdl.opengl();
        let window = builder_opengl.build().unwrap();

        // gl init
        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        unsafe {
            gl::Viewport(0, 0, 900, 700); // set viewport
        }

        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        self.window = Some(window);
        self.event_pump = Some(sdl.event_pump().unwrap());
        self.gl_context = Some(gl_context);
    }

    pub fn get_event(&mut self) -> EventApp {

        let event_pump = self.event_pump.as_mut().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => return EventApp::Quit,
                _ => {},
            }
        }

        EventApp::Pass
    }

    pub fn clear_screen(&mut self) {
        // gl clear.
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        self.window.as_mut().unwrap().gl_swap_window();
    }

    pub fn stop(&mut self) {
        println!("stop")
    }
}
