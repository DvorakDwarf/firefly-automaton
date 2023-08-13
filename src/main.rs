use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, self};
use winit_input_helper::WinitInputHelper;

mod grid;
mod cell;
mod consts;

use crate::consts::*;
use crate::grid::Grid;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let mut grid = Grid::new();

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
        }
    })
}
    
// use pixels::{Error, Pixels, SurfaceTexture};
// use winit::dpi::LogicalSize;
// use winit::event::{Event, VirtualKeyCode};
// use winit::event_loop::{ControlFlow, EventLoop};
// use winit::window::WindowBuilder;
// use winit_input_helper::WinitInputHelper;

// mod grid;
// mod cell;

// use crate::grid::Grid;

// const WIDTH: u32 = 320;
// const HEIGHT: u32 = 240;
// const BOX_SIZE: i16 = 64;

// fn main() -> Result<(), Error> {
//     let event_loop = EventLoop::new();
//     let mut input = WinitInputHelper::new();
//     let window = {
//         let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
//         WindowBuilder::new()
//             .with_title("Fireflies")
//             .with_inner_size(size)
//             .with_min_inner_size(size)
//             .build(&event_loop)
//             .unwrap()
//     };

//     let mut pixels = {
//         let window_size = window.inner_size();
//         let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
//         Pixels::new(WIDTH, HEIGHT, surface_texture)?
//     };

//     let mut grid = Grid::new();

//     event_loop.run(move |event, _, control_flow| {
//         // Draw the current frame
//         if let Event::RedrawRequested(_) = event {
//             grid.draw(pixels.frame_mut());
//             pixels.render().unwrap();
//         }

//         // Handle input events
//         if input.update(&event) {
//             // Close events
//             if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
//                 *control_flow = ControlFlow::Exit;
//                 return;
//             }


//             // Update internal state and request a redraw
//             window.request_redraw();
//         }
//     });
// }
