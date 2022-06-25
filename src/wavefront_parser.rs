use crate::linear_algebra::{Point3, Vec3Unsigned};
use std::borrow::BorrowMut;
use std::fs;
use std::path::PathBuf;
use std::str::SplitWhitespace;


#[derive(Debug)]
pub struct TextureUV {
    u: f64,
    v: Option<f64>,
    w: Option<f64>,
}

impl TextureUV {
    pub fn new(u: f64, v: Option<f64>, w: Option<f64>) -> Self {
        Self { u, v, w }
    }

    pub fn unravel_uv_impl(&self) -> (f64, f64) {
        (self.u, self.v.unwrap())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub xyz: Point3<f64>,
    pub w: Option<f64>,
}

impl Vertex {
    pub fn new(triplet: (f64, f64, f64), w: Option<f64>) -> Self {
        let (x, y, z) = triplet;

        Self {
            xyz: Point3(x, y, z),
            w,
        }
    }
}

impl std::ops::Sub for Vertex {
    type Output = Point3<f64>;

    fn sub(self, rhs: Self) -> Self::Output {
        let (x, y, z) = self.xyz.unravel();
        let (xo, yo, zo) = rhs.xyz.unravel();

        Point3(x - xo, y - yo, z - zo)
    }
}

pub struct SpaceVertex {
    u: f64,
    v: Option<f64>,
    w: Option<f64>,
}

impl SpaceVertex {
    pub fn new(u: f64, v: Option<f64>, w: Option<f64>) -> Self {
        Self { u, v, w }
    }
}

pub struct FaceElement {
    vertex_triplet: FaceTriplet,
    texture_triplet: FaceTriplet,
    normal_triplet: FaceTriplet,
}

type FaceTriplet = (Option<usize>, Option<usize>, Option<usize>);

impl FaceElement {
    pub fn new(
        vertex_triplet: FaceTriplet,
        texture_triplet: FaceTriplet,
        normal_triplet: FaceTriplet,
    ) -> Self {
        Self {
            vertex_triplet,
            texture_triplet,
            normal_triplet,
        }
    }
}

pub struct WavefronObject {
    v: Vec<Vertex>,
    vt: Vec<TextureUV>,
    vn: Vec<Point3<f64>>,
    vp: Vec<SpaceVertex>,
    f: Vec<FaceElement>,
    l: Vec<Vec<usize>>,
}

impl WavefronObject {
    pub fn new(path: PathBuf) -> Self {
        let read_str = Self::read_to_string(path.clone());
        let mut v: Vec<Vertex> = vec![];
        let mut vt: Vec<TextureUV> = vec![];
        let mut vn: Vec<Point3<f64>> = vec![];
        let mut vp: Vec<SpaceVertex> = vec![];
        let mut f: Vec<FaceElement> = vec![];
        let mut l: Vec<Vec<usize>> = vec![];

        let mut lines = read_str.lines();

        lines.for_each(|x| {
            Self::do_one_round(
                &mut x.split_whitespace(),
                &mut v,
                &mut vt,
                &mut vn,
                &mut vp,
                &mut f,
                &mut l,
            )
        });

        println!("Wavefront object {} loaded", path.display());
        Self {
            v,
            vt,
            vn,
            vp,
            f,
            l,
        }
    }

    fn do_one_round(
        split_ws: &mut SplitWhitespace,
        v: &mut Vec<Vertex>,
        vt: &mut Vec<TextureUV>,
        vn: &mut Vec<Point3<f64>>,
        vp: &mut Vec<SpaceVertex>,
        f: &mut Vec<FaceElement>,
        l: &mut Vec<Vec<usize>>,
    ) {
        Self::parse_vertex(split_ws.clone().borrow_mut(), v);
        Self::parse_uvs(split_ws.clone().borrow_mut(), vt);
        Self::parse_normals(split_ws.clone().borrow_mut(), vn);
        Self::parse_space_vertices(split_ws.clone().borrow_mut(), vp);
        Self::parse_lines(split_ws.clone().borrow_mut(), l);
        Self::parse_face_elements(split_ws.clone().borrow_mut(), f)
    }

    fn read_to_string(path: PathBuf) -> String {
        fs::read_to_string(path).unwrap()
    }

    fn parse_vertex(split_ws: &mut SplitWhitespace, v: &mut Vec<Vertex>) {
        match split_ws.next() {
            Some(first) => {
                if first == "v" {
                    let x_str = split_ws.next().unwrap();
                    let y_str = split_ws.next().unwrap();
                    let z_str = split_ws.next().unwrap();
                    let w_option = split_ws.next();

                    let triplet = (
                        x_str.parse::<f64>().unwrap(),
                        y_str.parse::<f64>().unwrap(),
                        z_str.parse::<f64>().unwrap(),
                    );

                    let w = if let Some(w_str) = w_option {
                        Some(w_str.parse::<f64>().unwrap())
                    } else {
                        None
                    };

                    let vertex = Vertex::new(triplet, w);

                    v.push(vertex)
                }
            }
            None => (),
        }
    }

    fn parse_uvs(split_ws: &mut SplitWhitespace, vt: &mut Vec<TextureUV>) {
        match split_ws.next() {
            Some(first) => {
                if first == "vt" {
                    let u_str = split_ws.next().unwrap();
                    let v_option = split_ws.next();
                    let w_option = split_ws.next();

                    let u = u_str.parse::<f64>().unwrap();

                    let v = if let Some(v_str) = v_option {
                        Some(v_str.parse::<f64>().unwrap())
                    } else {
                        None
                    };

                    let w = if let Some(w_str) = w_option {
                        Some(w_str.parse::<f64>().unwrap())
                    } else {
                        None
                    };

                    let texture_uv = TextureUV::new(u, v, w);

                    vt.push(texture_uv)
                }
            }
            None => (),
        }
    }

    fn parse_normals(split_ws: &mut SplitWhitespace, vn: &mut Vec<Point3<f64>>) {
        match split_ws.next() {
            Some(first) => {
                if first == "vn" {
                    let x_str = split_ws.next().unwrap();
                    let y_str = split_ws.next().unwrap();
                    let z_str = split_ws.next().unwrap();

                    let (x, y, z) = (
                        x_str.parse::<f64>().unwrap(),
                        y_str.parse::<f64>().unwrap(),
                        z_str.parse::<f64>().unwrap(),
                    );

                    let normal = Point3(x, y, z);

                    vn.push(normal)
                }
            }
            None => (),
        }
    }

    fn parse_space_vertices(split_ws: &mut SplitWhitespace, vp: &mut Vec<SpaceVertex>) {
        match split_ws.next() {
            Some(first) => {
                if first == "vp" {
                    let u_str = split_ws.next().unwrap();
                    let v_option = split_ws.next();
                    let w_option = split_ws.next();

                    let u = u_str.parse::<f64>().unwrap();

                    let v = if let Some(v_str) = v_option {
                        Some(v_str.parse::<f64>().unwrap())
                    } else {
                        None
                    };

                    let w = if let Some(w_str) = w_option {
                        Some(w_str.parse::<f64>().unwrap())
                    } else {
                        None
                    };

                    let space_vertex = SpaceVertex::new(u, v, w);

                    vp.push(space_vertex)
                }
            }
            None => (),
        }
    }

    fn parse_face_elements(split_ws: &mut SplitWhitespace, f: &mut Vec<FaceElement>) {
        fn parse_single_triplet(slash_seperated: &str) -> FaceTriplet {
            let mut split_slash = slash_seperated.split("/");

            let vi_option = split_slash.next();
            let ti_option = split_slash.next();
            let ni_option = split_slash.next();

            let vertex_index = if let Some(vi_str) = vi_option {
                Some(vi_str.parse::<usize>().unwrap())
            } else {
                None
            };

            let texture_index = if let Some(ti_str) = ti_option {
                Some(ti_str.parse::<usize>().unwrap())
            } else {
                None
            };

            let normal_index = if let Some(ni_str) = ni_option {
                Some(ni_str.parse::<usize>().unwrap())
            } else {
                None
            };

            (vertex_index, texture_index, normal_index)
        }

        if let Some(first_line) = split_ws.next() {
            if first_line != "f" {
                return ();
            }
        }

        let (vertex_triplet, texture_triplet, normal_triplet) = {
            let p1 = split_ws.next();
            let p2 = split_ws.next();
            let p3 = split_ws.next();

            if p1.is_none() || p2.is_none() || p3.is_none() {
                return ();
            }

            let (sl1, sl2, sl3) = (p1.unwrap(), p2.unwrap(), p3.unwrap());

            let trip1 = parse_single_triplet(sl1);
            let trip2 = parse_single_triplet(sl2);
            let trip3 = parse_single_triplet(sl3);

            let vertex_triplet = (trip1.0, trip2.0, trip3.0);
            let texture_triplet = (trip1.1, trip2.1, trip3.1);
            let normal_triplet = (trip1.2, trip2.2, trip3.2);

            (vertex_triplet, texture_triplet, normal_triplet)
        };

        let face_element = FaceElement::new(vertex_triplet, texture_triplet, normal_triplet);

        f.push(face_element)
    }

    fn parse_lines(split_ws: &mut SplitWhitespace, l: &mut Vec<Vec<usize>>) {
        match split_ws.next() {
            Some(first) => {
                if first == "l" {
                    let lines = split_ws
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();

                    l.push(lines)
                }
            }
            None => (),
        }
    }

    pub fn get_vertex_at_index(&self, u: &usize) -> Option<&Vertex> {
        self.v.get(*u - 1)
    }

    pub fn get_texture_at_index(&self, u: &usize) -> Option<&TextureUV> {
        self.vt.get(*u - 1)
    }

    pub fn get_vertex_impl(&self, u: usize) -> Vertex {
        self.v[u]
    }

    pub fn get_vert_triplets_from_face_elements(&self) -> Vec<Vec3Unsigned<usize>> {

        self.f
            .iter()
            .map(|x| x.vertex_triplet)
            .filter(|x| x.0.is_some() && x.1.is_some() && x.2.is_some())
            .map(|x| Vec3Unsigned::new(x.0.unwrap(), x.1.unwrap(), x.2.unwrap()))
            .collect::<Vec<Vec3Unsigned<usize>>>()
    }

    pub fn get_texture_triplets_from_elements(&self) -> Vec<Vec3Unsigned<usize>> {
        self.f
            .iter()
            .map(|x| x.texture_triplet)
            .filter(|x| x.0.is_some() && x.1.is_some() && x.2.is_some())
            .map(|x| Vec3Unsigned::new(x.0.unwrap(), x.1.unwrap(), x.2.unwrap()))
            .collect::<Vec<Vec3Unsigned<usize>>>()
    }

    pub fn get_len_vertices(&self) -> usize {
        self.v.len()
    }

    pub fn get_n_faces(&self) -> usize {
        let f = &self.f;

        f.len()
    }

    pub fn get_n_vertices(&self) -> usize {
        let v = &self.v;

        v.len()
    }
}
