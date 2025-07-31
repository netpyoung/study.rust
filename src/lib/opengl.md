# OpenGL
gl = "0.10.0"


    let builder_sdl = builder_window
        .position_centered()
        .resizable();
    let builder_opengl = builder_sdl.opengl();
    let window = builder_opengl.build().unwrap();


    // gl-init
    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }


    window.gl_swap_window();


# shader
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

unsafe {
    gl::Viewport(0, 0, 900, 700); // set viewport
}







    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);



image
polygon
shader


script - lisp
editor - clojure
