# glium_sdl2

[![Build Status](https://travis-ci.org/nukep/glium-sdl2.svg)](https://travis-ci.org/nukep/glium-sdl2)
[![Latest Version](https://img.shields.io/crates/v/glium_sdl2.svg)](https://crates.io/crates/glium_sdl2)

An SDL2 backend for [Glium](https://github.com/tomaka/glium) - a high-level
OpenGL wrapper for the Rust language.

This library, along with `glium` and [`rust-sdl2`](https://github.com/AngryLawyer/rust-sdl2),
are in heavy development and are subject to change.

```toml
[dependencies]
glium_sdl2 = "0.12"
sdl2 = "0.18"
glium = "0.14"

features = []
default-features = false
```

glium_sdl2 doesn't reexport the `glium` or `sdl2` crates, so you must declare
them _with the versions listed above_ in your `Cargo.toml` file.

glium_sdl2 will be bumped to 0.13, 0.14, etc. once this library, `glium` or `sdl2`
make breaking changes.

## [Documentation](http://nukep.github.io/glium-sdl2/)

## Example usage

Using glium with SDL2 is very similar to using glium with glutin.
Generally speaking, Glium documentation and tutorials should be fairly trivial
to adapt to SDL2.

```rust
#[macro_use]
extern crate glium;

extern crate glium_sdl2;
extern crate sdl2;

fn main() {
    use glium_sdl2::DisplayBuild;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let display = video_subsystem.window("My window", 800, 600)
        .resizable()
        .build_glium()
        .unwrap();

    let mut running = true;
    let mut event_pump = sdl_context.event_pump().unwrap();

    while running {
        let mut target = display.draw();
        // do drawing here...
        target.finish().unwrap();

        // Event loop: includes all windows

        for event in event_pump.poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit { .. } => {
                    running = false;
                },
                _ => ()
            }
        }
    }
}
```

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
