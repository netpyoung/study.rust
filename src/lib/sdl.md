# SDL
https://wiki.libsdl.org/FrontPage
Simple DirectMedia Layer is a cross-platform development library designed to provide low level access to audio, keyboard, mouse, joystick, and graphics hardware via OpenGL and Direct3D.


https://github.com/Rust-SDL2/rust-sdl2

Cargo.toml

``` toml
[dependencies.sdl2]
version = "0.31.0"
features = ["bundled", "static-link"]
```


"Bundled" Feature
Since 0.31, this crate supports a feature named "bundled" which downloads SDL2 from source, compiles it and links it automatically. While this should work for any architecture, you will need a C compiler (like gcc, clang, or MS's own compiler) to use this feature properly.

``` rust
extern crate sdl2;

fn main() {
    let sdl = sdl2::init().unwrap();
}
```

``` rust
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let mut builder_window = video_subsystem.window("Game", 300, 300);
    let builder_sdl = builder_window
        .position_centered()
        .resizable();
    let window = builder_sdl.build().unwrap();

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }
    }
```
