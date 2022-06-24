pub mod draw_triangle {
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

    pub fn draw_triangle_single(coords: TriangleCoords, image_canvas: &mut Canvas, color: u8) {
        let (w, h) = image_canvas.get_size();

        let mut bbox_min = Point2::new(w as i32 - 1, h as i32 - 1);
        let mut bbox_max = Point2::new(0, 0);
        let clamp = bbox_min.clone();

        for (tx, ty) in coords.unraval_vec() {
            bbox_min.0 = i32::max(0, i32::min(bbox_min.0, tx));
            bbox_min.1 = i32::max(0, i32::min(bbox_min.1, ty));

            bbox_max.0 = i32::min(clamp.0, i32::max(bbox_max.0, tx));
            bbox_max.1 = i32::min(clamp.1, i32::max(bbox_max.1, ty));
        }

        let pairs = bbox_min * bbox_max;

        pairs
            .into_iter()
            .map(|(i, j)| Point2::new(i, j))
            .map(|p| (p, coords.get_barycentric_coords(p)))
            .filter(|(_, bc)| bc.0 > 0.0 && bc.1 > 0.0 && bc.2 > 0.0)
            .for_each(|(p, _)| image_canvas.set_pixel(p.0, p.1, color).unwrap());
    }

    pub fn draw_triangle_threaded(coords: TriangleCoords, canvas_mutex: &Mutex<Canvas>, color: u8) {
        let mut image_canvas = canvas_mutex.lock().unwrap();
        let (w, h) = image_canvas.get_size();
        let mut bbox_min = Point2::new(w as i32 - 1, h as i32 - 1);
        let mut bbox_max = Point2::new(0, 0);
        let clamp = bbox_min.clone();

        for (tx, ty) in coords.unraval_vec() {
            bbox_min.0 = i32::max(0, i32::min(bbox_min.0, tx));
            bbox_min.1 = i32::max(0, i32::min(bbox_min.1, ty));

            bbox_max.0 = i32::min(clamp.0, i32::max(bbox_max.0, tx));
            bbox_max.1 = i32::min(clamp.1, i32::max(bbox_max.1, ty));
        }

        let pairs = bbox_min * bbox_max;

        pairs
            .into_iter()
            .map(|(i, j)| Point2::new(i, j))
            .map(|p| (p, coords.get_barycentric_coords(p)))
            .for_each(|(p, _)| image_canvas.set_pixel(p.0, p.1, color).unwrap());
    }
}
