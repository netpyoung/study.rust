extern crate gl;
extern crate sdl2;

fn main() {
    println!("Hello, world!");
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
    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 900, 700); // set viewport
    }

    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }



    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        // gl clear.
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.gl_swap_window();
    }
}
