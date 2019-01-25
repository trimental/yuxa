use winit::os::unix::WindowExt;
use x11_dl::xlib::{CopyFromParent, Display, Visual, Window, XImage, Xlib, ZPixmap, _XGC};

pub struct X11Backend {
    display: *mut Display,
    window: *mut Window,
    gc: *mut _XGC,
    xlib: Xlib,
    image: *mut XImage,
}

impl X11Backend {
    pub fn new(window: &winit::Window) -> Option<X11Backend> {
        let mut x11 = None;
        let xlib = Xlib::open().unwrap();
        let dimensions: (u32, u32) = window.get_inner_size().unwrap().to_physical(1.).into();

        if let Some(display) = window.get_xlib_display() {
            let display = display as *mut Display;
            if let Some(window) = window.get_xlib_window() {
                let window = window as *mut Window;
                let gc = unsafe { (xlib.XDefaultGC)(display, (xlib.XDefaultScreen)(display)) };

                let image = unsafe {
                    let depth =
                        (xlib.XDefaultDepth)(display, (xlib.XDefaultScreen)(display)) as u32;
                    (xlib.XCreateImage)(
                        display,
                        CopyFromParent as *mut Visual,
                        depth,
                        ZPixmap,
                        0,
                        std::ptr::null_mut(),
                        dimensions.0 as u32,
                        dimensions.1 as u32,
                        32,
                        dimensions.0 as i32 * 4,
                    )
                };

                x11 = Some(X11Backend {
                    display,
                    window,
                    gc,
                    xlib,
                    image,
                })
            }
        }

        x11
    }

    pub fn draw_argb8888(&mut self, dimensions: (usize, usize), buffer: &[[u8; 4]]) {
        unsafe {
            let mut new_buffer = Vec::new();

            for pixel in buffer {
                new_buffer.push(
                    u32::from(pixel[0]) << 24
                        | u32::from(pixel[1]) << 16
                        | u32::from(pixel[2]) << 8
                        | u32::from(pixel[3]),
                )
            }

            (*self.image).data = new_buffer.as_slice().as_ptr() as *mut libc::c_char;
            (*self.image).width = dimensions.0 as i32;
            (*self.image).bytes_per_line = dimensions.0 as i32 * 4;
            (*self.image).height = dimensions.1 as i32;

            (self.xlib.XPutImage)(
                self.display,
                self.window as u64,
                self.gc,
                self.image,
                0,
                0,
                0,
                0,
                dimensions.0 as u32,
                dimensions.1 as u32,
            );
            (self.xlib.XSync)(self.display, 0);
        }
    }

    pub fn draw_argb8888_bytes(&mut self, dimensions: (usize, usize), buffer: &[u8]) {
        unsafe {
            let mut new_buffer = Vec::new();

            for i in (0..buffer.len()).step_by(4) {
                new_buffer.push(
                    u32::from(buffer[i]) << 24
                        | u32::from(buffer[i + 1]) << 16
                        | u32::from(buffer[i + 2]) << 8
                        | u32::from(buffer[i + 3]),
                )
            }

            (*self.image).data = new_buffer.as_slice().as_ptr() as *mut libc::c_char;
            (*self.image).width = dimensions.0 as i32;
            (*self.image).bytes_per_line = dimensions.0 as i32 * 4;
            (*self.image).height = dimensions.1 as i32;

            (self.xlib.XPutImage)(
                self.display,
                self.window as u64,
                self.gc,
                self.image,
                0,
                0,
                0,
                0,
                dimensions.0 as u32,
                dimensions.1 as u32,
            );
            (self.xlib.XSync)(self.display, 0);
        }
    }

    pub fn draw_argb32(&mut self, dimensions: (usize, usize), buffer: &[u32]) {
        unsafe {
            (*self.image).data = buffer.as_ptr() as *mut libc::c_char;
            (*self.image).width = dimensions.0 as i32;
            (*self.image).bytes_per_line = dimensions.0 as i32 * 4;
            (*self.image).height = dimensions.1 as i32;

            (self.xlib.XPutImage)(
                self.display,
                self.window as u64,
                self.gc,
                self.image,
                0,
                0,
                0,
                0,
                dimensions.0 as u32,
                dimensions.1 as u32,
            );
            (self.xlib.XSync)(self.display, 0);
        }
    }
}
