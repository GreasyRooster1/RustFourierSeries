use std::f32::consts::PI;
use std::ffi::c_int;
use beryllium::video::RendererWindow;
use round::round;

pub trait DrawRadial {
    fn draw_ellipse(&self,points: &[[c_int; 4]]){}
    fn draw_circles(&self, circles: &[[c_int; 4]]){}
}
impl DrawRadial for RendererWindow{
    fn draw_circles(&self, circles: &[[c_int; 4]]) {
        for c in circles {
            let mut points: Vec<[i32; 2]> = vec![];
            let point_spacing = PI*2f32 / c[3] as f32;
            for i in 0.. c[3]+1 {
                points.append(&mut vec![[
                    (c[0]as f32+f32::cos(i as f32 * point_spacing)*c[2]as f32) as i32,
                    (c[1]as f32+f32::sin(i as f32 * point_spacing)*c[2]as f32) as i32
                ]]);
            }
            self.draw_lines(points.as_slice()).unwrap()
        }
    }
}

pub fn cos(x:f32) -> f32 {
    return x.cos();
}
pub fn sin(x:f32) -> f32 {
    return x.sin();
}