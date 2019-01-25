use std::io::{BufWriter, Seek, SeekFrom, Write};

use sctk::utils::DoubleMemPool;
use sctk::Environment;

use sctk::reexports::client::protocol::wl_surface::RequestsTrait as SurfaceRequests;
use sctk::reexports::client::protocol::{wl_shm, wl_surface};
use sctk::reexports::client::{Display, EventQueue, Proxy};
use sctk::wayland_client::sys::client::wl_display;

use winit::os::unix::WindowExt;

use byteorder::{NativeEndian, WriteBytesExt};

pub struct WaylandBackend {
    pub display: Display,
    pub event_queue: EventQueue,
    pub env: Environment,
    pub pools: DoubleMemPool,
    pub surface: Proxy<wl_surface::WlSurface>,
}

impl WaylandBackend {
    pub fn new(window: &winit::Window) -> Option<WaylandBackend> {
        let mut wayland = None;
        if let Some(winit_display) = window.get_wayland_display() {
            if let Some(surface) = window.get_wayland_surface() {
                let (display, mut event_queue) =
                    unsafe { Display::from_external_display(winit_display as *mut wl_display) };
                let env = Environment::from_display(&*display, &mut event_queue).unwrap();
                let pools =
                    DoubleMemPool::new(&env.shm, || {}).expect("Failed to create a memory pool !");
                let surface = unsafe { Proxy::from_c_ptr(surface as *mut _) };

                wayland = Some(WaylandBackend {
                    display,
                    event_queue,
                    env,
                    pools,
                    surface,
                });
            }
        }
        wayland
    }

    pub fn draw_argb8888(&mut self, dimensions: (usize, usize), buffer: &[[u8; 4]]) {
        if let Some(mut pool) = self.pools.pool() {
            pool.resize(4 * dimensions.0 * dimensions.1)
                .expect("Failed to resize the memory pool.");
            pool.seek(SeekFrom::Start(0)).unwrap();
            if cfg!(target_endian = "little") {
                let mut writer = BufWriter::new(&mut pool);
                for pixel in buffer {
                    writer
                        .write_all(&[pixel[3], pixel[2], pixel[1], pixel[0]])
                        .unwrap()
                }
            } else {
                let mut writer = BufWriter::new(&mut pool);
                for pixel in buffer {
                    writer.write_all(pixel).unwrap()
                }
            }

            pool.flush().unwrap();
            let new_buffer = pool.buffer(
                0,
                dimensions.0 as i32,
                dimensions.1 as i32,
                4 * dimensions.0 as i32,
                wl_shm::Format::Argb8888,
            );
            self.surface.attach(Some(&new_buffer), 0, 0);
            self.surface.commit();
            self.surface
                .damage(0, 0, dimensions.0 as i32, dimensions.1 as i32);

            self.display.flush().unwrap();
            self.event_queue.dispatch_pending().unwrap();
            self.event_queue.sync_roundtrip().unwrap();
        }
    }

    pub fn draw_argb8888_bytes(&mut self, dimensions: (usize, usize), buffer: &[u8]) {
        if let Some(mut pool) = self.pools.pool() {
            pool.resize(4 * dimensions.0 * dimensions.1)
                .expect("Failed to resize the memory pool.");
            pool.seek(SeekFrom::Start(0)).unwrap();
            if cfg!(target_endian = "little") {
                let mut writer = BufWriter::new(&mut pool);
                for i in (0..buffer.len()).step_by(4) {
                    writer
                        .write_all(&[buffer[i + 3], buffer[i + 2], buffer[i + 1], buffer[i]])
                        .unwrap()
                }
            } else {
                pool.write_all(buffer).unwrap();
            }

            pool.flush().unwrap();
            let new_buffer = pool.buffer(
                0,
                dimensions.0 as i32,
                dimensions.1 as i32,
                4 * dimensions.0 as i32,
                wl_shm::Format::Argb8888,
            );
            self.surface.attach(Some(&new_buffer), 0, 0);
            self.surface.commit();
            self.surface
                .damage(0, 0, dimensions.0 as i32, dimensions.1 as i32);

            self.display.flush().unwrap();
            self.event_queue.dispatch_pending().unwrap();
            self.event_queue.sync_roundtrip().unwrap();
        }
    }

    pub fn draw_argb32(&mut self, dimensions: (usize, usize), buffer: &[u32]) {
        if let Some(mut pool) = self.pools.pool() {
            pool.resize(4 * dimensions.0 * dimensions.1)
                .expect("Failed to resize the memory pool.");
            pool.seek(SeekFrom::Start(0)).unwrap();
            {
                let mut writer = BufWriter::new(&mut pool);
                for pixel in buffer {
                    writer.write_u32::<NativeEndian>(*pixel).unwrap();
                }
            }
            pool.flush().unwrap();
            let new_buffer = pool.buffer(
                0,
                dimensions.0 as i32,
                dimensions.1 as i32,
                4 * dimensions.0 as i32,
                wl_shm::Format::Argb8888,
            );
            self.surface.attach(Some(&new_buffer), 0, 0);
            self.surface.commit();
            self.surface
                .damage(0, 0, dimensions.0 as i32, dimensions.1 as i32);

            self.display.flush().unwrap();
            self.event_queue.dispatch_pending().unwrap();
            self.event_queue.sync_roundtrip().unwrap();
        }
    }
}
