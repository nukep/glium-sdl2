// Copyright (c) 2016 glium_sdl2 developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

//! An SDL2 backend for [Glium](https://github.com/tomaka/glium) - a high-level
//! OpenGL wrapper for the Rust language.
//!
//! # Example
//! ```no_run
//! # #[macro_use] extern crate glium;
//! # extern crate glium_sdl2;
//! # extern crate sdl2;
//! # fn main() {
//! use glium_sdl2::DisplayBuild;
//!
//! let sdl_context = sdl2::init().unwrap();
//! let video_subsystem = sdl_context.video().unwrap();
//!
//! let display = video_subsystem.window("My window", 800, 600)
//!     .resizable()
//!     .build_glium()
//!     .unwrap();
//!
//! let mut running = true;
//! let mut event_pump = sdl_context.event_pump().unwrap();
//!
//! while running {
//!     let mut target = display.draw();
//!     // do drawing here...
//!     target.finish().unwrap();
//!
//!     // Event loop: includes all windows
//!
//!     for event in event_pump.poll_iter() {
//!         use sdl2::event::Event;
//!
//!         match event {
//!             Event::Quit { .. } => {
//!                 running = false;
//!             },
//!             _ => ()
//!         }
//!     }
//! }
//! # }
//! ```

extern crate glium;
extern crate sdl2;

use std::cell::UnsafeCell;
use std::ops::Deref;
use std::rc::Rc;
use std::os::raw::c_void;
use std::error::Error;
use std::fmt;

use glium::debug;
use glium::backend::{Backend, Context, Facade};
use sdl2::video as sdl;

pub type Display = SDL2Facade;

use self::GliumSdl2Error::*;

#[derive(Debug)]
pub enum GliumSdl2Error {
    WindowBuildError(sdl::WindowBuildError),
    GlContextCreationError(String),
    GliumContextCreationError(glium::IncompatibleOpenGl),
}

impl From<glium::IncompatibleOpenGl> for GliumSdl2Error {
    fn from(err: glium::IncompatibleOpenGl) -> Self {
        GliumContextCreationError(err)
    }
}

impl From<sdl::WindowBuildError> for GliumSdl2Error {
    fn from(err: sdl::WindowBuildError) -> Self {
        WindowBuildError(err)
    }
}

impl From<String> for GliumSdl2Error {
    fn from(s: String) -> Self {
        GlContextCreationError(s)
    }
}

impl Error for GliumSdl2Error {
    fn description(&self) -> &str {
        match *self {
            WindowBuildError(ref err) => err.description(),
            GlContextCreationError(ref s) => s.as_str(),
            GliumContextCreationError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            WindowBuildError(ref err) => err.cause(),
            GlContextCreationError(_) => None,
            GliumContextCreationError(ref err) => err.cause(),
        }
    }
}

impl fmt::Display for GliumSdl2Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

/// Facade implementation for an SDL2 window.
#[derive(Clone)]
pub struct SDL2Facade {
    // contains everything related to the current context and its state
    context: Rc<Context>,

    backend: Rc<SDL2WindowBackend>,
}

impl Facade for SDL2Facade {
    fn get_context(&self) -> &Rc<Context> { &self.context }
}

impl Deref for SDL2Facade {
    type Target = Context;

    fn deref(&self) -> &Context { &self.context }
}

impl SDL2Facade {
    pub fn window(&self) -> &sdl::Window {
        self.backend.window()
    }

    pub fn window_mut(&mut self) -> &mut sdl::Window {
        self.backend.window_mut()
    }

    /// Start drawing on the backbuffer.
    ///
    /// This function returns a `Frame`, which can be used to draw on it.
    /// When the `Frame` is destroyed, the buffers are swapped.
    ///
    /// Note that destroying a `Frame` is immediate, even if vsync is enabled.
    pub fn draw(&self) -> glium::Frame {
        glium::Frame::new(self.context.clone(), self.backend.get_framebuffer_dimensions())
    }
}

/// An object that can build a facade object.
///
/// This trait is different from `glium::DisplayBuild` because Rust doesn't allow trait
/// implementations on types from external crates, unless the trait is in the same crate as the impl.
/// To clarify, both `glium::DisplayBuild` and `sdl2::video::WindowBuilder` are in different crates
/// than `glium_sdl2`.
pub trait DisplayBuild {
    /// The object that this `DisplayBuild` builds.
    type Facade: glium::backend::Facade;

    /// The type of error that initialization can return.
    type Err;

    /// Build a context and a facade to draw on it.
    ///
    /// Performs a compatibility check to make sure that all core elements of glium
    /// are supported by the implementation.
    fn build_glium(self) -> Result<Self::Facade, Self::Err> where Self: Sized {
        self.build_glium_debug(Default::default())
    }

    /// Build a context and a facade to draw on it.
    ///
    /// Performs a compatibility check to make sure that all core elements of glium
    /// are supported by the implementation.
    fn build_glium_debug(self, debug::DebugCallbackBehavior) -> Result<Self::Facade, Self::Err>;

    /// Build a context and a facade to draw on it
    ///
    /// This function does the same as `build_glium`, except that the resulting context
    /// will assume that the current OpenGL context will never change.
    unsafe fn build_glium_unchecked(self) -> Result<Self::Facade, Self::Err> where Self: Sized {
        self.build_glium_unchecked_debug(Default::default())
    }

    /// Build a context and a facade to draw on it
    ///
    /// This function does the same as `build_glium`, except that the resulting context
    /// will assume that the current OpenGL context will never change.
    unsafe fn build_glium_unchecked_debug(self, debug::DebugCallbackBehavior)
                                          -> Result<Self::Facade, Self::Err>;

    // TODO
    // Changes the settings of an existing facade.
    // fn rebuild_glium(self, &Self::Facade) -> Result<(), Self::Err>;
}

impl<'a> DisplayBuild for &'a mut sdl2::video::WindowBuilder {
    type Facade = SDL2Facade;
    type Err = GliumSdl2Error;

    fn build_glium_debug(self, debug: debug::DebugCallbackBehavior) -> Result<SDL2Facade, GliumSdl2Error> {
        let backend = Rc::new(SDL2WindowBackend::new(self)?);
        let context = unsafe { Context::new(backend.clone(), true, debug)? };

        Ok(SDL2Facade {
            context: context,
            backend: backend
        })
    }

    unsafe fn build_glium_unchecked_debug(self, debug: debug::DebugCallbackBehavior) -> Result<SDL2Facade, GliumSdl2Error> {
        let backend = Rc::new(SDL2WindowBackend::new(self)?);
        let context = Context::new(backend.clone(), false, debug)?;

        Ok(SDL2Facade {
            context: context,
            backend: backend
        })
    }
}

pub struct SDL2WindowBackend {
    window: UnsafeCell<sdl::Window>,
    context: sdl::GLContext
}

impl SDL2WindowBackend {
    fn window(&self) -> &sdl::Window {
        /*let ptr = self.window.get();
        let window: &sdl::Window = unsafe { mem::transmute(ptr) };*/
        unsafe { &*self.window.get() }
    }

    fn window_mut(&self) -> &mut sdl::Window {
        /*let ptr = self.window.get();
        let window: &mut sdl::Window = unsafe { mem::transmute(ptr) };*/
        unsafe { &mut *self.window.get() }
    }

    pub fn new(window_builder: &mut sdl::WindowBuilder) -> Result<SDL2WindowBackend, GliumSdl2Error> {
        let window = window_builder.opengl().build()?;
        let context = window.gl_create_context()?;

        Ok(SDL2WindowBackend {
            window: UnsafeCell::new(window),
            context: context,
        })
    }
}

unsafe impl Backend for SDL2WindowBackend {
    fn swap_buffers(&self) -> Result<(), glium::SwapBuffersError> {
        self.window().gl_swap_window();

        // AFAIK, SDL or `SDL_GL_SwapWindow` doesn't have any way to detect context loss.
        // TODO: Find out if context loss is an issue in SDL2 (especially for the Android port).

        Ok(())
    }

    unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
        // Assumes the appropriate context for the window has been set before this call.

        self.window().subsystem().gl_get_proc_address(symbol) as *const c_void
    }

    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        let (width, height) = self.window().drawable_size();
        (width as u32, height as u32)
    }

    fn is_current(&self) -> bool {
        self.context.is_current()
    }

    unsafe fn make_current(&self) {
        self.window().gl_make_current(&self.context).unwrap()
    }
}
