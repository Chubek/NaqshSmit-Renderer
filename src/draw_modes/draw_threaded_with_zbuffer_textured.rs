pub mod draw_triangle_threaded_with_zbuffer_texture {

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

    pub fn draw_triangle_threaded_with_zbuffer_with_texture(
        coords: TriangleCoords3,
        zbuffer: &Mutex<Vec<f64>>,
        canvas_mutex: &Mutex<Canvas>,
        uv_points: &UVTriplet,
        texture: &Vec<u8>,
    ) {
        let mut canvas_lock = canvas_mutex.lock().unwrap();

        let ss = canvas_lock.get_map();

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

        let mut zbf = zbuffer.lock().unwrap();

        pairs
            .into_iter()
            .map(|(i, j)| Point3(i as f64, j as f64, 0.0f64))
            .map(|p| (p, coords.get_barycentric_coords(p)))
            .map(|(mut p, bc)| {
                for i in 0usize..3usize {
                    p.2 += coords[i][2] * bc[i];
                }
                let color = uv_points.get_color(p, coords, texture);
                (p, color)
            })
            .for_each(
                |(p, color)| match zbf[p.0 as usize + p.1 as usize * w] < p.2 {
                    true => {
                        zbf[p.0 as usize + p.1 as usize * w] = p.2;

                        let ss = canvas_lock.get_map();

                        println!("{}", ss[p.0 as usize][p.1 as usize]);

                        canvas_lock
                            .set_pixel(p.0 as i32, p.1 as i32, color)
                            .unwrap();

                        let ss = canvas_lock.get_map();

                        println!("{}", ss[p.0 as usize][p.1 as usize]);
                    }
                    false => (),
                },
            );
    }

    pub fn shade_threaded_with_zbuffer_with_texture(
        obj_path: PathBuf,
        texture_path: PathBuf,
        image_canvas: &Arc<Mutex<Canvas>>,
    ) {
        let model = WavefronObject::new(obj_path);

        let canv = Arc::clone(image_canvas);

        let canvas = canv.as_ref().lock().unwrap();

        let (w, h) = canvas.get_size();

        let zbuffer = vec![-f64::MAX; w * h];

        let face_vertices = model.get_vert_triplets_from_face_elements();

        let zbuffer_arc = Arc::new(Mutex::new(zbuffer));

        let face_textures = model.get_texture_triplets_from_elements();

        let texture = read_tga(texture_path);

        face_vertices
            .into_iter()
            .zip(face_textures.into_iter())
            .for_each(|(verts, textures)| {
                let mut v3: Vec<Point3<f64>> = vec![];

                for v in verts.unravel_vec() {
                    let vertex = model.get_vertex_at_index(&v).unwrap();

                    let xyz = vertex.xyz;

                    let w2s = vertex.xyz.from_world_to_screen(w, h);

                    v3.push(w2s);
                }

                let mut points_uv: Vec<Point2<f64>> = vec![];
                for i in textures.unravel_vec() {
                    let uv = model.get_texture_at_index(&i).unwrap();

                    let (u, v) = uv.unravel_uv_impl();

                    points_uv.push(Point2(u as f64, v as f64));
                }

                let uv_triplet = UVTriplet::from_vec(points_uv);

                let pts = TriangleCoords3::from_vec(v3);
                let texture_arc = Arc::new(texture.clone());
                let arc_pts = Arc::new(pts);
                let arc_img_clone = Arc::clone(&image_canvas.clone());
                let zbuffer_clone = Arc::clone(&zbuffer_arc.clone());
                let uv_arc = Arc::new(uv_triplet);

                thread::spawn(move || {
                    let coords = arc_pts.deref();
                    let canvas_mutex = arc_img_clone.deref();
                    let mutex_zbuffer = zbuffer_clone.deref();
                    let texture = texture_arc.deref();
                    let uv_points = uv_arc.deref();
                    let zbuffer = zbuffer_clone.deref();

                    draw_triangle_threaded_with_zbuffer_with_texture(
                        *coords,
                        zbuffer,
                        canvas_mutex,
                        uv_points,
                        texture,
                    );
                });

                ()
            })
    }
}
