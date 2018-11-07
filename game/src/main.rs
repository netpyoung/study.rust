extern crate rustpyoung;
use std::ffi::{CString};
extern crate gl;

fn main() {
    let mut app = rustpyoung::core::App::new();
    app.start();
    app.clear_screen();

    // compile shader
    println!("[frag]\n{}", include_str!("triangle_frag.glsl"));
    println!("[vert]\n{}", include_str!("triangle_vert.glsl"));

    let vert_shader = rustpyoung::core::shader::Shader::from_vert_source(
        &CString::new(include_str!("triangle_vert.glsl")).unwrap()
    ).unwrap();
    let frag_shader = rustpyoung::core::shader::Shader::from_frag_source(
        &CString::new(include_str!("triangle_frag.glsl")).unwrap()
    ).unwrap();
    let shader_program = rustpyoung::core::shader::Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();


    // ref: http://docs.gl/
    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    // set up vertex array object
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }
    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    while app.state == rustpyoung::core::AppState::Running {
        match app.get_event() {
            rustpyoung::core::EventApp::Pass => {}
            rustpyoung::core::EventApp::Quit => app.state = rustpyoung::core::AppState::Exiting
        }
        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3 // number of indices to be rendered
            );
        }

        //app.clear_screen();
        app.refresh();
    }
    app.stop();
}
