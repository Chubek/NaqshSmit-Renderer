pub mod flag_shader_with_light {
    use std::borrow::BorrowMut;
    use std::ops::Deref;
    use std::path::PathBuf;

    use crate::draw_modes::draw_triangle::draw_triangle::draw_triangle_threaded;
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

    pub fn flat_shader_with_light_renderer(
        obj_path: PathBuf,
        light_dir: Point3<f64>,
        image_canvas: &Arc<Mutex<Canvas>>,
    ) {
        let model = WavefronObject::new(obj_path);

        let canv = Arc::clone(&image_canvas);

        let canv_for_size = canv.as_ref().lock().unwrap();

        let (w, h) = canv_for_size.get_size();

        let face_vertices = model.get_vert_triplets_from_face_elements();

        let mut i = 0;

        face_vertices.into_iter().for_each(|vert_index| {
            let (pt_vec, light_intensity) = {
                let mut pt_vec: Vec<Point2<i32>> = vec![];
                let mut vt_vec: Vec<Vertex> = vec![];
                for vi in vert_index.unravel_vec() {
                    let vertex = model.get_vertex_at_index(&vi).unwrap();
                    let (vx, vy, _) = vertex.xyz.unravel();

                    let p1 = ((vx + 1.0) * w as f64 / 2.0).abs() as i32;
                    let p2 = ((vy + 1.0) * h as f64 / 2.0).abs() as i32;

                    let p = Point2(p1, p2);

                    pt_vec.push(p);
                    vt_vec.push(*vertex);
                }

                (
                    TriangleCoords::from_vec(pt_vec),
                    calculate_normal_and_intensity(vt_vec, light_dir),
                )
            };

            let arc_pt = Arc::new(pt_vec);
            let arc_intensity = Arc::new(light_intensity);
            let arc_mutex_canv = Arc::clone(&image_canvas);

            thread::spawn(move || {
                let coords = arc_pt.as_ref();
                let mut canvas = arc_mutex_canv.deref();
                let intensity = arc_intensity.deref();

                if *intensity > 0.0 {
                    let color = (255.0 * *intensity).round() as i32;
                    draw_triangle_threaded(*coords, canvas, color as u8);
                }
            });

            ()
        })
    }
}
