use winit::{EventsLoop, Window, WindowBuilder};

use winapi::shared::windef::HDC;
use winapi::um::{wingdi, winuser};
use winit::os::windows::WindowExt;

use crate::CreationError;

pub struct YuxaWindow {
    window: Window,
    hdc: HDC,
}

impl YuxaWindow {
    pub fn new(
        window_builder: WindowBuilder,
        events_loop: &EventsLoop,
    ) -> Result<Self, CreationError> {
        let window = window_builder.build(events_loop)?;
        let hdc = unsafe { winuser::GetDC(window.get_hwnd() as *mut _) };

        Ok(YuxaWindow { window, hdc })
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
        let dimensions: (u32, u32) = self.window.get_inner_size().unwrap().to_physical(1.).into();
        let dimensions = (dimensions.0 as i32, dimensions.1 as i32);

        let mut new_buffer = Vec::with_capacity(dimensions.0 as usize * dimensions.1 as usize);
        for i in 0..buffer.len() {
            let pixel = unsafe { std::mem::transmute::<[u8; 4], u32>(buffer[i]).to_be() };
            new_buffer.push(pixel);
        }
        let new_buffer: &[u32] = &new_buffer;

        unsafe {
            let map = wingdi::CreateBitmap(
                dimensions.0,
                dimensions.1,
                1,
                32,
                (new_buffer as *const [u32]) as *const std::ffi::c_void,
            );

            let src = wingdi::CreateCompatibleDC(self.hdc);
            wingdi::SelectObject(src, map as *mut std::ffi::c_void);

            wingdi::BitBlt(
                self.hdc,
                0,
                0,
                dimensions.0,
                dimensions.1,
                src,
                0,
                0,
                wingdi::SRCCOPY,
            );
        }
    }

    pub fn draw_argb8888_bytes(&mut self, buffer: &[u8]) {
        let dimensions: (u32, u32) = self.window.get_inner_size().unwrap().to_physical(1.).into();
        let dimensions = (dimensions.0 as i32, dimensions.1 as i32);

        let mut new_buffer = Vec::with_capacity(dimensions.0 as usize * dimensions.1 as usize);
        for i in (0..buffer.len()).step_by(4) {
            let pixel = unsafe {
                std::mem::transmute::<[u8; 4], u32>([
                    buffer[i],
                    buffer[i + 1],
                    buffer[i + 2],
                    buffer[i + 3],
                ])
                .to_be()
            };
            new_buffer.push(pixel);
        }
        let new_buffer: &[u32] = &new_buffer;

        unsafe {
            let map = wingdi::CreateBitmap(
                dimensions.0,
                dimensions.1,
                1,
                32,
                (new_buffer as *const [u32]) as *const std::ffi::c_void,
            );

            let src = wingdi::CreateCompatibleDC(self.hdc);
            wingdi::SelectObject(src, map as *mut std::ffi::c_void);

            wingdi::BitBlt(
                self.hdc,
                0,
                0,
                dimensions.0,
                dimensions.1,
                src,
                0,
                0,
                wingdi::SRCCOPY,
            );
            wingdi::DeleteDC(src);
        }
    }

    pub fn draw_argb32(&mut self, buffer: &[u32]) {
        let dimensions: (u32, u32) = self.window.get_inner_size().unwrap().to_physical(1.).into();
        let dimensions = (dimensions.0 as i32, dimensions.1 as i32);
        unsafe {
            let map = wingdi::CreateBitmap(
                dimensions.0,
                dimensions.1,
                1,
                32,
                (buffer as *const [u32]) as *const std::ffi::c_void,
            );

            let src = wingdi::CreateCompatibleDC(self.hdc);
            wingdi::SelectObject(src, map as *mut std::ffi::c_void);

            wingdi::BitBlt(
                self.hdc,
                0,
                0,
                dimensions.0,
                dimensions.1,
                src,
                0,
                0,
                wingdi::SRCCOPY,
            );
        }
    }
}
