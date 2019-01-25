use winit::{EventsLoop, Window, WindowBuilder};

use crate::CreationError;
use wayland::WaylandBackend;
use x11::X11Backend;

mod wayland;
mod x11;

pub struct YuxaWindow {
    window: Window,
    wayland: Option<WaylandBackend>,
    x11: Option<X11Backend>,
}

impl YuxaWindow {
    pub fn new(
        window_builder: WindowBuilder,
        events_loop: &EventsLoop,
    ) -> Result<Self, CreationError> {
        let window = window_builder.build(events_loop)?;

        let wayland = WaylandBackend::new(&window);
        let x11 = if wayland.is_none() {
            X11Backend::new(&window)
        } else {
            None
        };

        Ok(YuxaWindow {
            wayland,
            x11,
            window,
        })
    }

    /// Get reference to the inner winit window
    pub fn window(&self) -> &Window {
        &self.window
    }

    /// Get mutable reference to the inner winit window
    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn draw_argb8888(&mut self, buffer: &[[u8; 4]]) {
        if let Some(wayland) = &mut self.wayland {
            let dimensions: (u32, u32) =
                self.window.get_inner_size().unwrap().to_physical(1.).into();
            wayland.draw_argb8888((dimensions.0 as usize, dimensions.1 as usize), buffer);
        }
        if let Some(x11) = &mut self.x11 {
            let dimensions: (u32, u32) =
                self.window.get_inner_size().unwrap().to_physical(1.).into();
            x11.draw_argb8888((dimensions.0 as usize, dimensions.1 as usize), buffer);
        }
    }

    pub fn draw_argb8888_bytes(&mut self, buffer: &[u8]) {
        if let Some(wayland) = &mut self.wayland {
            let dimensions: (u32, u32) =
                self.window.get_inner_size().unwrap().to_physical(1.).into();
            wayland.draw_argb8888_bytes((dimensions.0 as usize, dimensions.1 as usize), buffer);
        }
        if let Some(x11) = &mut self.x11 {
            let dimensions: (u32, u32) =
                self.window.get_inner_size().unwrap().to_physical(1.).into();
            x11.draw_argb8888_bytes((dimensions.0 as usize, dimensions.1 as usize), buffer);
        }
    }

    pub fn draw_argb32(&mut self, buffer: &[u32]) {
        if let Some(wayland) = &mut self.wayland {
            let dimensions: (u32, u32) =
                self.window.get_inner_size().unwrap().to_physical(1.).into();
            wayland.draw_argb32((dimensions.0 as usize, dimensions.1 as usize), buffer);
        }
        if let Some(x11) = &mut self.x11 {
            let dimensions: (u32, u32) =
                self.window.get_inner_size().unwrap().to_physical(1.).into();
            x11.draw_argb32((dimensions.0 as usize, dimensions.1 as usize), buffer);
        }
    }
}
