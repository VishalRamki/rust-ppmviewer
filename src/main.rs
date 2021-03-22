#![deny(clippy::all)]
#![forbid(unsafe_code)]

extern crate args;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::env;

/// Representation of the application state
#[derive(Clone)]
struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
    frame: Option<PPM>,
}

impl World {
    fn get_width(self) -> i32 {
        self.frame.unwrap().width
    }

    fn get_height(self) -> i32 {
        self.frame.unwrap().height
    }
}

#[derive(Debug, Clone)]
struct PPM {
    ppm_type: PpmType,
    width: i32,
    height: i32,
    max_value: i32,
    values: Vec<PpmValue>,
}

#[derive(Debug, Clone)]
struct PpmValue {
    r: i32,
    g: i32,
    b: i32,
}

impl PpmValue {
    fn new(red: i32, green: i32, blue: i32) -> Self {
        PpmValue {
            r: red,
            g: green,
            b: blue,
        }
    } 
}

#[derive(Debug, Clone)]
enum PpmType {
    P3, // RGB color image in ASCII
}

impl PPM {
    fn new() -> Self {
        PPM {
            ppm_type: PpmType::P3,
            width: 0,
            height: 0,
            max_value: 0,
            values: Vec::new(),
        }
    }
}

fn read_ppm_file(path: &str) -> io::Result<PPM> {
    let file = File::open(path)?;
    let mut dat : PPM = PPM::new();
    let reader = BufReader::new(file);

    let mut skip_first_line : bool = false;

    for line in reader.lines() {
        //println!("{}", line?);
        let va = line.unwrap_or_default();
        if !skip_first_line {
            skip_first_line = true;
            continue
        }

        if dat.width == 0 && dat.height == 0 {
            let bar : Vec<i32> = va.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
            dat.width = bar[0];
            dat.height = bar[1];
            println!("This is width & Height: {:?}", bar);
            continue
        }

        if dat.max_value == 0 {
            dat.max_value = va.parse::<i32>().unwrap_or_default();
            //println!("This is width & Height: {:?}", dat.max_value);
            continue
        }

        let x : Vec<i32> = va.split(' ').map(|x| x.parse::<i32>().unwrap_or_default()).collect();
        dat.values.push(PpmValue::new(x[0], x[1], x[2]));
    }
    Ok(dat)
}

fn main() -> Result<(), Error> {

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("File Name is required.");
        std::process::exit(0);
    }

    let filename = &args[1];

    if filename.is_empty() {
        println!("File Name is required.");
        std::process::exit(0);
    }
    let mut world = World::new();
    world.frame = Some(read_ppm_file(filename).unwrap());
    let w_width = world.clone().get_width();
    let w_height = world.clone().get_height();
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(w_width as f64, w_height as f64);
        WindowBuilder::new()
            .with_title("PPMViewer - by github@VishalRamki")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(w_width as u32, w_height as u32, surface_texture)?
    };

    //let mut graphic = aci_ppm::decode(&input_f, afi::ColorChannels::Rgb).unwrap();
    //world.frame = graphic.pop();
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            // @TODO: Ensure we can resize the window;
            /*
            if let Some(size) = input.window_resized() {
                //pixels.surface_size(size.width, size.height);
            }
            */
            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
            frame: None,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let frame_instance = self.frame.as_ref().unwrap();
            let rgba = [frame_instance.values[i].r as u8, frame_instance.values[i].g as u8, frame_instance.values[i].b as u8, 0xff];
            pixel.copy_from_slice(&rgba);
        }
    }
}