pub mod flat_shader_with_zbuffer {
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

    pub fn flat_shader_renderer_threaded_with_zbuffer(
        obj_path: PathBuf,
        image_canvas: &Arc<Mutex<Canvas>>,
    ) {
        let model = WavefronObject::new(obj_path);

        let canv = Arc::clone(&image_canvas);

        let canv_for_size = canv.as_ref().lock().unwrap();

        let (w, h) = canv_for_size.get_size();

        let face_vertices = model.get_vert_triplets_from_face_elements();

        face_vertices.into_iter().for_each(|vert_index| {
            let pt_vec = {
                let mut pt_vec: Vec<Point2<i32>> = vec![];

                for vi in vert_index.unravel_vec() {
                    let vertex = model.get_vertex_at_index(&vi).unwrap();

                    let (vx, vy, _) = vertex.xyz.unravel();

                    let p1 = ((*vx + 1.0) * w as f64 / 2.0).round() as i32;
                    let p2 = ((*vy + 1.0) * h as f64 / 2.0).round() as i32;

                    let p = Point2(p1, p2);

                    pt_vec.push(p);
                }
                TriangleCoords::from_vec(pt_vec)
            };

            let arc_pt = Arc::new(pt_vec);
            let arc_mutex_canv = Arc::clone(&image_canvas);

            thread::spawn(move || {
                let coords = arc_pt.as_ref();
                let mut canvas = arc_mutex_canv.deref();
                let mut rng = rand::thread_rng();
                let color: u8 = rng.gen();
                draw_triangle_threaded(*coords, canvas, color);
            });

            ()
        })
    }
}
