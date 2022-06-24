#![allow(unused)]

use std::{path::PathBuf, sync::{Arc, Mutex}};

use linear_algebra::{TriangleCoords, Point2, Point3};
use wavefront_parser::WavefronObject;

mod context;
mod image_canvas;
mod linear_algebra;
mod utils;
mod wavefront_parser;
mod read_tga;
mod draw_modes;


#[test]
fn test_wave_front() {
    let model = WavefronObject::new(PathBuf::from("resources/african_head.obj"));

    let n_faces = model.get_n_faces();
    let n_vertices = model.get_n_vertices();

    assert_eq!(n_faces, 2492);

    assert_eq!(n_vertices, 1258);
    
}



fn main() {
    let mut image_canvas = crate::image_canvas::Canvas::new(800, 800, 20);


   let arc_mutex_canv = Arc::new(Mutex::new(image_canvas));


   crate::context::display_threaded_image_on_screen(arc_mutex_canv);
}
