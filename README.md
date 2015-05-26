# glium_sdl2

An SDL2 backend for [Glium](https://github.com/tomaka/glium) - a high-level
OpenGL wrapper for the Rust language.

This library, along with `glium` and [`rust-sdl2`](https://github.com/AngryLawyer/rust-sdl2),
are in heavy development and are subject to change.

## Example usage

Using glium with SDL2 is very similar to using glium with glutin.
Generally speaking, Glium documentation and tutorials should be fairly trivial
to adapt to SDL2.

```rust
#[macro_use]
extern crate glium;

extern crate glium_sdl2;
extern crate sdl2;

let mut sdl_context = sdl2::init().video().unwrap();

let display = sdl_context.window("My window", 800, 600)
    .resizable()
    .build_glium()
    .unwrap();

let mut running = true;

while running {
    let mut target = display.draw();
    // do drawing here...
    target.finish();

    // Event loop: includes all windows

    for event in sdl_context.event_pump().poll_iter() {
        use sdl2::event::Event;

        match event {
            Event::Quit { .. } => {
                running = false;
            },
            _ => ()
        }
    }
}
```
