pub mod draw_triangle_threaded_with_zbuffer {

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

    pub fn draw_triangle_threaded_with_zbuffer(
        coords: TriangleCoords3,
        zbuffer: &Mutex<Vec<f64>>,
        canvas_mutex: &Mutex<Canvas>,
        color: u8,
    ) {
        let mut canvas_lock = match canvas_mutex.lock() {
            Ok(p_ok) => p_ok,
            Err(p_err) => p_err.into_inner(),
        };

        let mut bbox_min = Point2(std::f64::MAX, std::f64::MAX);
        let mut bbox_max = Point2(-std::f64::MAX, -std::f64::MAX);

        let (w, h) = canvas_lock.get_size();

        let clamp = Point2((w - 1) as f64, (h - 1) as f64);

        for i in 0..3 {
            for j in 0..2 {
                bbox_min[j] = f64::max(0.0, f64::min(bbox_min[j], coords[i][j] as f64));
                bbox_max[j] = f64::min(clamp[j], f64::max(bbox_max[j], coords[i][j] as f64));
            }
        }

        let pairs = bbox_min * bbox_max;

        let mut zbf = match zbuffer.lock() {
            Ok(p_ok) => p_ok,
            Err(p_err) => p_err.into_inner(),
        };

        pairs
            .into_iter()
            .map(|(i, j)| Point3(i as f64, j as f64, 0.0f64))
            .map(|p| (p, coords.get_barycentric_coords(p)))
            .map(|(mut p, bc)| {
                for i in 0usize..3usize {
                    p.2 += coords[i][2] * bc[i];
                }

                p
            })
            .for_each(|p| {
                if zbf[p.0 as usize + p.1 as usize * w] < p.2 {
                    zbf[p.0 as usize + p.1 as usize * w] = p.2;
                    canvas_lock
                        .set_pixel(p.0 as i32, p.1 as i32, color)
                        .unwrap()
                }
            });
    }

    pub fn shade_threaded_with_zbuffer(obj_path: PathBuf, image_canvas: &Arc<Mutex<Canvas>>) {
        let model = WavefronObject::new(obj_path);

        let canv = Arc::clone(image_canvas);

        let canvas = canv.as_ref().lock().unwrap();

        let (w, h) = canvas.get_size();

        let zbuffer = vec![-f64::MAX; w * h];

        let face_vertices = model.get_vert_triplets_from_face_elements();

        let zbuffer_arc = Arc::new(Mutex::new(zbuffer));

        face_vertices.into_iter().for_each(|verts| {
            let mut v3: Vec<Point3<f64>> = vec![];

            for v in verts.unravel_vec() {
                let vertex = model.get_vertex_at_index(&v).unwrap();

                let xyz = vertex.xyz;

                let w2s = vertex.xyz.from_world_to_screen(w, h);

                v3.push(w2s);
            }

            let pts = TriangleCoords3::from_vec(v3);

            let arc_pts = Arc::new(pts);
            let arc_img_clone = Arc::clone(&image_canvas.clone());
            let zbuffer_clone = Arc::clone(&zbuffer_arc.clone());

            thread::spawn(move || {
                let pts = arc_pts.deref();
                let mutex_img = arc_img_clone.deref();
                let mutex_zbuffer = zbuffer_clone.deref();

                let mut rng = rand::thread_rng();
                let color: u8 = rng.gen();

                draw_triangle_threaded_with_zbuffer(*pts, mutex_zbuffer, mutex_img, color);
            });

            ()
        })
    }
}
