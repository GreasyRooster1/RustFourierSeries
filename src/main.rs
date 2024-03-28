use std::f32::consts::PI;
use beryllium::{
    events::Event,
    init::InitFlags,
    video::{CreateWinArgs, RendererFlags},
    Sdl,
};
use beryllium::*;
use beryllium::video::RendererWindow;
use pixel_formats::r8g8b8a8_Srgb;
use FourierSeries::{cos, DrawRadial, sin};

fn main() {
    // Initializes SDL2
    let sdl = Sdl::init(InitFlags::EVERYTHING);

    for info in sdl.get_renderer_driver_infos().unwrap() {
        println!("RendererDriver: {info:?}");
    }

    // Makes the window with an associated SDL Renderer.
    let win = sdl
        .create_renderer_window(
            CreateWinArgs {
                title: "Example Renderer Window",
                width:500,
                height:500,
                ..Default::default()
            },
            RendererFlags::ACCELERATED_VSYNC,
        )
        .unwrap();
    println!("Created The Renderer Window!");
    println!("Selected Renderer Info: {:?}", win.get_renderer_info());

    //todo: create a fourier series data structure for this
    let frequencies:Vec<f32>=vec![1.0,2.0,-3.0,4.0];
    let amplitudes:Vec<f32>=vec![4.0,3.0,2.0,1.0];
    let mut angle = 0.0;
    let scale = 20.0;

    'main_loop: loop {
        #[allow(clippy::never_loop)]
        #[allow(clippy::single_match)]
        while let Some((event, _timestamp)) = sdl.poll_events() {
            match event {
                Event::Quit => break 'main_loop,
                _ => (),
            }
        }

        win.set_draw_color(0,0,0,255).unwrap();
        win.clear().unwrap();

        render_circles(&win,&frequencies,&amplitudes,angle,scale);
        render_parametric(&win,&frequencies,&amplitudes,scale);

        angle+=0.01;
        win.present();
    }

    // All the cleanup is handled by the various drop impls.
}

fn render_parametric(win:&RendererWindow,frequencies:&Vec<f32>,amplitudes:&Vec<f32>,scale:f32) {
    let steps = 100;
    let mut parametric_points:Vec<[i32;2]> = vec![];
    win.set_draw_color(255,255,255,255).unwrap();
    for s in 0..steps+1 {
        let mut prev_pos = vec![0.0,0.0];
        let angle = (2.0*PI/steps as f32)*s as f32;
        for i in 1..frequencies.len() + 1 {
            let prev_amp = amplitudes[i - 1];
            let  freq = frequencies[i - 1];
            let x = prev_pos[0] + cos(angle * freq) * prev_amp;
            let y = prev_pos[1] + sin(angle * freq) * prev_amp;
            prev_pos=vec![x,y];
        }
        let sx = (prev_pos[0]*scale) as i32+250;
        let sy = (prev_pos[1]*scale) as i32+250;
        parametric_points.append(&mut vec![[sx, sy]]);
    }
    win.draw_lines(parametric_points.as_slice()).unwrap()
}

fn render_circles(win:&RendererWindow,frequencies:&Vec<f32>,amplitudes:&Vec<f32>,angle:f32,scale:f32){
    let mut prev_pos = vec![0.0,0.0];
    for i in 1..frequencies.len() {
        let amp = amplitudes[i];
        let prev_amp = amplitudes[i - 1];
        let freq = frequencies[i - 1];
        let x = prev_pos[0] + cos(angle * freq) * prev_amp;
        let y = prev_pos[1] + sin(angle * freq) * prev_amp;
        win.set_draw_color(127, 127, 127, 255).unwrap();
        win.draw_circles(&[[250 + (x * scale) as i32, 250 + (y * scale) as i32, (amp * scale) as i32, 40]]);
        win.set_draw_color(255, 0, 0, u8::MAX).unwrap();
        win.draw_circles(&[[250 + (x * 20.0) as i32, 250 + (y * 20.0) as i32, 5, 20]]);
        win.set_draw_color(0, 0, 255, u8::MAX).unwrap();
        win.draw_lines(&[[250 + (prev_pos[0] * 20.0) as i32, 250 + (prev_pos[1] * 20.0) as i32],[250 + (x * 20.0) as i32, 250 + (y * 20.0) as i32]]).unwrap();
        prev_pos = vec![x, y];
        if i == frequencies.len()-1 {
            let prev_amp = amp;
            let freq = frequencies[frequencies.len()-1];
            let x = prev_pos[0] + cos(angle * freq) * prev_amp;
            let y = prev_pos[1] + sin(angle * freq) * prev_amp;
            win.draw_lines(&[[250 + (prev_pos[0] * 20.0) as i32, 250 + (prev_pos[1] * 20.0) as i32],[250 + (x * 20.0) as i32, 250 + (y * 20.0) as i32]]).unwrap();
            win.set_draw_color(255, 255, 0, u8::MAX).unwrap();
            win.draw_circles(&[[250 + (x * 20.0) as i32, 250 + (y * 20.0) as i32, 5, 20]]);
        }
    }
}