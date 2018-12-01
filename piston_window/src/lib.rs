//#![deny(missing_docs)]

extern crate piston;
//extern crate window;
//extern crate input;

extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate graphics;
extern crate shader_version;
pub extern crate texture;

pub use shader_version::OpenGL;
pub use graphics::*;

//pub use window::*;
//pub use input::*;
pub use piston::window::*;
pub use piston::input::*;
pub use piston::event_loop::*;
pub use gfx_graphics::{ Texture, TextureSettings, Filter, Flip };

use gfx_graphics::{ Gfx2d, GfxGraphics };
use std::time::Duration;

/// Actual factory used by Gfx backend.
pub type GfxFactory = gfx_device_gl::Factory;
/// Actual gfx::Stream implementation carried by the window.
pub type GfxEncoder = gfx::Encoder<gfx_device_gl::Resources,
    gfx_device_gl::CommandBuffer>;
/// Glyph cache.
pub type Glyphs = gfx_graphics::GlyphCache<'static, gfx_device_gl::Factory,
    gfx_device_gl::Resources>;
/// 2D graphics.
pub type G2d<'a> = GfxGraphics<'a,
    gfx_device_gl::Resources,
    gfx_device_gl::CommandBuffer>;
/// Texture type compatible with `G2d`.
pub type G2dTexture = Texture<gfx_device_gl::Resources>;


#[cfg(feature="sdl2")]
extern crate sdl2_window;
#[cfg(feature="sdl2")]
use sdl2_window::Sdl2Window;
/// Contains everything required for controlling window, graphics, event loop.
#[cfg(feature="sdl2")]
pub struct PistonWindow<W: Window = Sdl2Window> {
    /// The window.
    pub window: W,
    /// GFX encoder.
    pub encoder: GfxEncoder,
    /// GFX device.
    pub device: gfx_device_gl::Device,
    /// Output frame buffer.
    pub output_color: gfx::handle::RenderTargetView<
        gfx_device_gl::Resources, gfx::format::Rgba8>,
    /// Output stencil buffer.
    pub output_stencil: gfx::handle::DepthStencilView<
        gfx_device_gl::Resources, gfx::format::DepthStencil>,
    /// Gfx2d.
    //pub g2d: Gfx2d<gfx_device_gl::Resources>,
    /// Event loop state.
    pub events: Events,
    /// The factory that was created along with the device.
    pub factory: gfx_device_gl::Factory,
}

#[cfg(feature="glutin")]
extern crate glutin_window;
#[cfg(feature="glutin")]
use glutin_window::GlutinWindow;
/// Contains everything required for controlling window, graphics, event loop.
#[cfg(feature="glutin")]
pub struct PistonWindow<W: Window = GlutinWindow> {
    /// The window.
    pub window: W,
    /// GFX encoder.
    pub encoder: GfxEncoder,
    /// GFX device.
    pub device: gfx_device_gl::Device,
    /// Output frame buffer.
    pub output_color: gfx::handle::RenderTargetView<
        gfx_device_gl::Resources, gfx::format::Rgba8>,
    /// Output stencil buffer.
    pub output_stencil: gfx::handle::DepthStencilView<
        gfx_device_gl::Resources, gfx::format::DepthStencil>,
    /// Gfx2d.
    //pub g2d: Gfx2d<gfx_device_gl::Resources>,
    /// Event loop state.
    pub events: Events,
    /// The factory that was created along with the device.
    pub factory: gfx_device_gl::Factory,
}

impl<W> BuildFromWindowSettings for PistonWindow<W>
    where W: Window + OpenGLWindow + BuildFromWindowSettings
{
    fn build_from_window_settings(settings: &WindowSettings) -> Result<PistonWindow<W>, String> {
        // Turn on sRGB.
        let settings = settings.clone().srgb(false);

        // Use OpenGL 3.2 by default, because this is what window backends
        // usually do.
        let opengl = settings.get_maybe_opengl().unwrap_or(OpenGL::V3_2);
        let samples = settings.get_samples();

        Ok(PistonWindow::new(opengl, samples, try!(settings.build())))
    }
}

fn create_main_targets(dim: gfx::texture::Dimensions) ->
(gfx::handle::RenderTargetView<
    gfx_device_gl::Resources, gfx::format::Rgba8>,
 gfx::handle::DepthStencilView<
    gfx_device_gl::Resources, gfx::format::DepthStencil>) {
    use gfx::format::{DepthStencil, Format, Formatted, Rgba8};
    use gfx::memory::Typed;

    let color_format: Format = <Rgba8 as Formatted>::get_format();
    let depth_format: Format = <DepthStencil as Formatted>::get_format();
    let (output_color, output_stencil) =
        gfx_device_gl::create_main_targets_raw(dim,
                                               color_format.0,
                                               depth_format.0);
    let output_color = Typed::new(output_color);
    let output_stencil = Typed::new(output_stencil);
    (output_color, output_stencil)
}

impl<W> PistonWindow<W>
    where W: Window
{
    /// Creates a new piston window.
    pub fn new(opengl: OpenGL, samples: u8, mut window: W) -> Self
        where W: OpenGLWindow
    {
        let (device, mut factory) =
            gfx_device_gl::create(|s|
                window.get_proc_address(s) as *const _);

        let (output_color, output_stencil) = {
            let aa = samples as gfx::texture::NumSamples;
            let draw_size = window.draw_size();
            let dim = (draw_size.width as u16, draw_size.height as u16,
                       1, aa.into());
            create_main_targets(dim)
        };

        //let g2d = Gfx2d::new(opengl, &mut factory);
        let encoder = factory.create_command_buffer().into();
        let events = Events::new(EventSettings::new());
        PistonWindow {
            window: window,
            encoder: encoder,
            device: device,
            output_color: output_color,
            output_stencil: output_stencil,
            //g2d: g2d,
            events: events,
            factory: factory,
        }
    }

    /// Renders 2D graphics.
    ///
    /// Calls the closure on render events.
    /// There is no need to filter events manually, and there is no overhead.
    pub fn draw_2d<E, F, U>(&mut self, e: &E, f: F) -> Option<U> where
        W: OpenGLWindow,
        E: GenericEvent,
        F: FnOnce(Context, &mut G2d) -> U
    {
        /*if let Some(args) = e.render_args() {
            self.window.make_current();
            let res = self.g2d.draw(
                &mut self.encoder,
                &self.output_color,
                &self.output_stencil,
                args.viewport(),
                f
            );
            self.encoder.flush(&mut self.device);
            Some(res)
        } else {*/
            None
        //}
    }

    /// Renders 3D graphics.
    ///
    /// Calls the closure on render events.
    /// There is no need to filter events manually, and there is no overhead.
    pub fn draw_3d<E, F, U>(&mut self, e: &E, f: F) -> Option<U> where
        W: OpenGLWindow,
        E: GenericEvent,
        F: FnOnce(&mut Self) -> U
    {
        if let Some(_) = e.render_args() {
            self.window.make_current();
            let res = f(self);
            self.encoder.flush(&mut self.device);
            Some(res)
        } else {
            None
        }
    }

    /// Returns next event.
    /// Cleans up after rendering and resizes frame buffers.
    pub fn next(&mut self) -> Option<Event> {
        if let Some(e) = self.events.next(&mut self.window) {
            self.event(&e);
            Some(e)
        } else {
            None
        }
    }

    /// Let window handle new event.
    /// Cleans up after rendering and resizes frame buffers.
    pub fn event<E: GenericEvent>(&mut self, event: &E) {
        use gfx::Device;
        use gfx::memory::Typed;

        if let Some(_) = event.after_render_args() {
            // After swapping buffers.
            self.device.cleanup();
        }

        // Check whether window has resized and update the output.
        let dim = self.output_color.raw().get_dimensions();
        let (w, h) = (dim.0, dim.1);
        let draw_size = self.window.draw_size();
        if w != draw_size.width as u16 || h != draw_size.height as u16 {
            let dim = (draw_size.width as u16,
                       draw_size.height as u16,
                       dim.2, dim.3);
            let (output_color, output_stencil) = create_main_targets(dim);
            self.output_color = output_color;
            self.output_stencil = output_stencil;
        }
    }
}

impl<W> Window for PistonWindow<W>
    where W: Window
{
    fn should_close(&self) -> bool { self.window.should_close() }
    fn set_should_close(&mut self, value: bool) {
        self.window.set_should_close(value)
    }
    fn size(&self) -> Size { self.window.size() }
    fn draw_size(&self) -> Size { self.window.draw_size() }
    fn swap_buffers(&mut self) { self.window.swap_buffers() }
    fn wait_event(&mut self) -> Input {
        Window::wait_event(&mut self.window)
    }
    fn wait_event_timeout(&mut self, timeout: Duration) -> Option<Input> {
        Window::wait_event_timeout(&mut self.window, timeout)
    }
    fn poll_event(&mut self) -> Option<Input> {
        Window::poll_event(&mut self.window)
    }
}

impl<W> AdvancedWindow for PistonWindow<W>
    where W: AdvancedWindow
{
    fn get_title(&self) -> String { self.window.get_title() }
    fn set_title(&mut self, title: String) {
        self.window.set_title(title)
    }
    fn get_exit_on_esc(&self) -> bool { self.window.get_exit_on_esc() }
    fn set_exit_on_esc(&mut self, value: bool) {
        self.window.set_exit_on_esc(value)
    }
    fn set_capture_cursor(&mut self, value: bool) {
        self.window.set_capture_cursor(value)
    }
    fn show(&mut self) { self.window.show() }
    fn hide(&mut self) { self.window.hide() }
    fn get_position(&self) -> Option<Position> {
        self.window.get_position()
    }
    fn set_position<P: Into<Position>>(&mut self, pos: P) {
        self.window.set_position(pos)
    }
    fn set_size<S: Into<Size>>(&mut self, size: S) {
        self.window.set_size(size)
    }
}

impl<W> EventLoop for PistonWindow<W>
    where W: Window
{
    fn get_event_settings(&self) -> EventSettings {
        self.events.get_event_settings()
    }

    fn set_event_settings(&mut self, settings: EventSettings) {
        self.events.set_event_settings(settings);
    }
}