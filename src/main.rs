use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, self};
use winit_input_helper::WinitInputHelper;
use std::time::{Duration, Instant};
use std::{thread, time};

mod grid;
mod cell;
mod consts;
mod light;
mod firefly;

use crate::consts::*;
use crate::grid::Grid;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let mut grid = Grid::new();
    const FRAME_DURATION: Duration = time::Duration::from_millis((1000. / FPS) as u64);

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        let timer = Instant::now();

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            grid.draw(pixels.frame_mut());
            pixels.render().unwrap();
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            grid.update();
            window.request_redraw();

            //Account for time spent computing
            if timer.elapsed() < FRAME_DURATION {
                let leftover_frame = FRAME_DURATION - timer.elapsed();
                thread::sleep(leftover_frame);
            }
        }
    })
}