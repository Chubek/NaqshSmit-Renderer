pub mod draw_wireframe {
    use std::borrow::BorrowMut;
    use std::ops::Deref;
    use std::path::PathBuf;

    use crate::image_canvas::{self, Canvas};
    use crate::linear_algebra::{
        calculate_intensity, calculate_normal_and_intensity, Point2, Point3, TriangleCoords,
        TriangleCoords3, UVTriplet,
    };
    use crate::read_tga::read_tga;
    use crate::utils::swap;
    use crate::wavefront_parser::{Vertex, WavefronObject};
    use embedded_graphics::text;
    use rand::Rng;
    use std::sync::{Arc, Mutex, MutexGuard};
    use std::{thread, vec};
    pub fn draw_line(from: Point2<i32>, to: Point2<i32>, image_canvas: &mut Canvas, color: u8) {
        let mut steep = false;

        let (mut x1, mut y1) = from.get_pair_as_clones();
        let (mut x2, mut y2) = to.get_pair_as_clones();

        if (x1 - x2).abs() < (y1 - y2).abs() {
            (x1, y1, x2, y2) = (y1, x1, y2, x2);
            steep = true;
        } else if x1 > x2 {
            (x1, x2, y1, y2) = (x2, x1, y2, y1);
        }

        let mut dx = x2 - x1;
        let mut dy = y2 - y1;

        let derr = (dy).abs() * 2;
        let mut err = 0;

        let mut y = y1.clone();

        for x in x1..x2 {
            if steep {
                image_canvas.set_pixel(y, x, color);
            } else {
                image_canvas.set_pixel(x, y, color);
            }

            err += derr;

            if err > dx {
                y += if y2 > y1 { 1 } else { -1 };
                err -= dx * 2;
            }
        }
    }

    pub fn wireframe_renderer(obj_path: PathBuf, image_canvas: &mut Canvas, color: u8) {
        let model = WavefronObject::new(obj_path);
        let (h, w) = image_canvas.get_size();

        let face_vertices = model.get_vert_triplets_from_face_elements();

        face_vertices.into_iter().for_each(|veretex_indices| {
            let combination = veretex_indices.combinate(2);

            combination.into_iter().for_each(|pair| {
                let (vi, vii) = (pair[0], pair[1]);

                let first_vertex = model.get_vertex_at_index(&vi).unwrap();
                let second_vertex = model.get_vertex_at_index(&vii).unwrap();

                let (xv1, yv1, _) = first_vertex.xyz.unravel();
                let (xv2, yv2, _) = second_vertex.xyz.unravel();

                let x1 = (xv1 + 1.0) * (w as f64) / 2.0;
                let y1 = (yv1 + 1.0) * (h as f64) / 2.0;
                let x2 = (xv2 + 1.0) * (w as f64) / 2.0;
                let y2 = (yv2 + 1.0) * (h as f64) / 2.0;

                let from = Point2::new(x1.round() as i32, y1.round() as i32);
                let to = Point2::new(x2.round() as i32, y2.round() as i32);

                draw_line(from, to, image_canvas, color);
            })
        })
    }
}
