use crate::{utils::swap, wavefront_parser::Vertex};
use itertools::Itertools;
use num_traits::{NumOps, PrimInt, Signed, ToPrimitive, Unsigned};

#[derive(Clone, Copy, Debug)]
pub struct Point2<T: Signed + NumOps + Clone + ToPrimitive>(pub T, pub T);

#[derive(Clone, Copy, Debug)]
pub struct Point3<T: Signed + NumOps + Clone + ToPrimitive>(pub T, pub T, pub T);

#[derive(Clone, Copy, Debug)]
pub struct Point4<T: Signed + NumOps + Clone + ToPrimitive>(T, T, T);

impl<T: Signed + NumOps + Clone + ToPrimitive> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Point2(x, y)
    }

    pub fn get_pair_as_clones(&self) -> (T, T) {
        let Point2(x1, y1) = self;

        (x1.clone(), y1.clone())
    }

    pub fn unravel(&self) -> (&T, &T) {
        let Point2(p1, p2) = self;

        (p1, p2)
    }

    pub fn unravel_mut(&mut self) -> (&mut T, &mut T) {
        let Point2(p1, p2) = self;

        (p1, p2)
    }
}

impl std::ops::Mul for Point2<f64> {
    type Output = Vec<(f64, f64)>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut ret: Vec<(f64, f64)> = vec![];

        let mut v1 = vec![];
        let mut v2 = vec![];

        let mut i = self.0;
        let mut j = self.1;

        while i <= rhs.0 {
            i += 1.0;
            v1.push(i);
        }

        while j <= rhs.1 {
            j += 1.0;
            v2.push(j);
        }

        v1.into_iter()
            .cartesian_product(v2.into_iter())
            .collect_vec()
    }
}

impl std::ops::Index<usize> for Point3<f64> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        let (p1, p2, p3) = self.unravel();

        match index {
            0 => p1,
            1 => p2,
            2 => p3,
            _ => todo!(),
        }
    }
}

impl Point2<f64> {
    pub fn expand(&self) -> Point3<f64> {
        let (p1, p2) = self.unravel();

        Point3(*p1, *p2, 1f64)
    }
}

impl std::ops::Mul for Point2<i32> {
    type Output = Vec<(i32, i32)>;

    fn mul(self, rhs: Self) -> Self::Output {
        (self.0..=rhs.0)
            .cartesian_product((self.1..=rhs.1))
            .collect_vec()
    }
}

impl std::ops::Sub for Point2<i32> {
    type Output = Point2<i32>;

    fn sub(self, rhs: Self) -> Self::Output {
        let (x, y) = self.get_pair_as_clones();
        let (ox, oy) = rhs.get_pair_as_clones();

        Point2::new(x - ox, y - oy)
    }
}

impl std::ops::Index<usize> for Point2<i32> {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        let (p1, p2) = self.unravel();

        match index {
            0 => &p1,
            1 => &p2,
            _ => panic!("Index {} larger than what fits", &index),
        }
    }
}

impl std::ops::Index<usize> for Point2<f64> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        let (p1, p2) = self.unravel();
        match index {
            0 => &p1,
            1 => &p2,
            _ => panic!("Index {} larger than what fits", &index),
        }
    }
}

impl std::ops::IndexMut<usize> for Point2<f64> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let (p1, p2) = self.unravel_mut();

        match index {
            0 => p1,
            1 => p2,
            _ => panic!("Index {} larger than what fits", &index),
        }
    }
}

impl std::ops::IndexMut<usize> for Point3<f64> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let (p1, p2, p3) = self.unravel_mut();

        match index {
            0 => p1,
            1 => p2,
            2 => p3,
            _ => panic!("Index {} larger than what fits", &index),
        }
    }
}

impl<T: Signed + NumOps + Clone + ToPrimitive> Point3<T> {
    pub fn unravel(&self) -> (&T, &T, &T) {
        let Point3(p1, p2, p3) = self;

        (p1, p2, p3)
    }

    pub fn get_as_f64(&self) -> (f64, f64, f64) {
        let Point3(p1, p2, p3) = self;

        (
            p1.to_f64().unwrap(),
            p2.to_f64().unwrap(),
            p3.to_f64().unwrap(),
        )
    }

    pub fn unravel_mut(&mut self) -> (&mut T, &mut T, &mut T) {
        let Point3(p1, p2, p3) = self;

        (p1, p2, p3)
    }

    pub fn get_length(&self) -> f64 {
        let Point3(p1, p2, p3) = self;

        let x_squared = p1.to_f64().unwrap().powf(2.0);
        let y_squared = p2.to_f64().unwrap().powf(2.0);
        let z_squared = p3.to_f64().unwrap().powf(2.0);

        let sum = x_squared + y_squared + z_squared;

        sum.to_f64().unwrap().sqrt()
    }

    pub fn normalize(&self) -> Point3<f64> {
        let Point3(p1, p2, p3) = self;

        let length = self.get_length();

        let x = p1.to_f64().unwrap() / length;
        let y = p2.to_f64().unwrap() / length;
        let z = p3.to_f64().unwrap() / length;

        Point3(x, y, z)
    }
}

impl std::ops::BitXor for Point3<f64> {
    type Output = Point3<f64>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let Point3(ax, ay, az) = self;
        let Point3(bx, by, bz) = rhs;

        let i = ay * bz - az * bz;
        let j = ax * bz - az * bx;
        let k = ax * by - ay * bx;

        Point3(i, -j, k)
    }
}

impl std::ops::Index<usize> for Point3<i32> {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        let (p1, p2, p3) = &self.unravel();

        match index {
            0 => p1,
            1 => p2,
            2 => p3,
            _ => panic!("Index {} larger than what fits", &index),
        }
    }
}

impl std::ops::Mul for Point3<f64> {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        let (x, y, z) = self.unravel();
        let (xo, yo, zo) = rhs.unravel();

        (x * xo) + (y * yo) + (z * zo)
    }
}

pub fn convert_to_screen_coords<T: Signed + NumOps + Clone + ToPrimitive>(p: T) -> usize {
    let p_abs = p.abs().to_usize().unwrap();

    p_abs
}

impl Point3<f64> {
    pub fn from_world_to_screen(&self, width: usize, height: usize) -> Point3<f64> {
        let (p1, p2, p3) = self.unravel();

        let pr1 = ((*p1 + 1.0) * width as f64) as f64 / 2.0 + 0.5;
        let pr2 = ((*p2 + 1.0) * height as f64) as f64 / 2.0 + 0.5;
        let pr3 = *p3;

        let res = Point3(pr1, pr2, pr3);

        let (k1, k2, k3) = res.unravel();

        let (kk1, kk2, kk3) = (*k1, *k2, *k3);

        res
    }
}

#[derive(Copy, Debug, Clone)]
pub struct Vec3Unsigned<T: Unsigned + PrimInt>(T, T, T);

impl<T: Unsigned + PrimInt> Vec3Unsigned<T> {
    pub fn new(p1: T, p2: T, p3: T) -> Self {
        Vec3Unsigned(p1, p2, p3)
    }

    pub fn combinate(&self, num: usize) -> Vec<Vec<T>> {
        let Vec3Unsigned(p1, p2, p3) = *self;

        vec![p1, p2, p3].into_iter().combinations(num).collect_vec()
    }

    pub fn unravel(&self) -> (T, T, T) {
        let Vec3Unsigned(p1, p2, p3) = *self;

        (p1, p2, p3)
    }

    pub fn unravel_vec(&self) -> Vec<T> {
        let Vec3Unsigned(p1, p2, p3) = *self;

        vec![p1, p2, p3]
    }
}

impl std::ops::BitAnd for Point3<f64> {
    type Output = f64;

    fn bitand(self, rhs: Self) -> Self::Output {
        let (p1, p2, p3) = self.unravel();
        let (po1, po2, po3) = rhs.unravel();

        *p1 * *po1 + *p2 * *po2 + *p3 * *po3
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TriangleCoords(Point2<i32>, Point2<i32>, Point2<i32>);

impl TriangleCoords {
    pub fn new(p1: Point2<i32>, p2: Point2<i32>, p3: Point2<i32>) -> Self {
        Self(p1, p2, p3)
    }

    pub fn from_vec(v: Vec<Point2<i32>>) -> Self {
        Self(v[0], v[1], v[2])
    }

    pub fn unravel_point3(&self) -> (&Point2<i32>, &Point2<i32>, &Point2<i32>) {
        let TriangleCoords(p1, p2, p3) = self;

        (p1, p2, p3)
    }

    pub fn unraval(&self) -> (i32, i32, i32, i32, i32, i32) {
        let TriangleCoords(p1, p2, p3) = self;

        let (p11, p12) = p1.get_pair_as_clones();
        let (p21, p22) = p2.get_pair_as_clones();
        let (p31, p32) = p3.get_pair_as_clones();

        (p11, p12, p21, p22, p31, p32)
    }

    pub fn unraval_vec(&self) -> Vec<(i32, i32)> {
        let TriangleCoords(p1, p2, p3) = self;

        let (p11, p12) = p1.get_pair_as_clones();
        let (p21, p22) = p2.get_pair_as_clones();
        let (p31, p32) = p3.get_pair_as_clones();

        vec![(p11, p12), (p21, p22), (p31, p32)]
    }

    pub fn unravel_multidim_vec(&self) -> Vec<Vec<i32>> {
        let TriangleCoords(p1, p2, p3) = self;

        let (p11, p12) = p1.get_pair_as_clones();
        let (p21, p22) = p2.get_pair_as_clones();
        let (p31, p32) = p3.get_pair_as_clones();

        vec![vec![p11, p12], vec![p21, p22], vec![p31, p32]]
    }

    pub fn get_barycentric_coords(&self, respect_point: Point2<i32>) -> Point3<f64> {
        let pts = self.unravel_multidim_vec();
        let (rp1, rp2) = respect_point.get_pair_as_clones();

        let res11 = pts[2][0] - pts[0][0];
        let res12 = pts[1][0] - pts[0][0];
        let res13 = pts[0][0] - rp1;

        let res21 = pts[2][1] - pts[0][1];
        let res22 = pts[1][1] - pts[0][1];
        let res23 = pts[0][1] - rp2;

        let p3_first = Point3(res11 as f64, res12 as f64, res13 as f64);
        let p3_second = Point3(res21 as f64, res22 as f64, res23 as f64);

        let outer_product = p3_first ^ p3_second;

        let (ux, uy, uz) = outer_product.get_as_f64();

        let ret = match uz.abs() < 1f64 {
            true => Point3(-1f64, 1f64, 1f64),
            false => Point3(1f64 - (ux + uy) / uz, uy / uz, ux / uz),
        };

        ret
    }
}

impl std::ops::Index<usize> for TriangleCoords {
    type Output = Point2<i32>;

    fn index(&self, index: usize) -> &Self::Output {
        let (p1, p2, p3) = self.unravel_point3();

        match index {
            0 => p1,
            1 => p2,
            3 => p3,
            _ => panic!("Index {} larger than what fits", &index),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TriangleCoords3(Point3<f64>, Point3<f64>, Point3<f64>);

impl TriangleCoords3 {
    pub fn new(p1: Point3<f64>, p2: Point3<f64>, p3: Point3<f64>) -> Self {
        Self(p1, p2, p3)
    }

    pub fn from_vec(v: Vec<Point3<f64>>) -> Self {
        Self(v[0], v[1], v[2])
    }

    pub fn unravel_point3(&self) -> (&Point3<f64>, &Point3<f64>, &Point3<f64>) {
        let TriangleCoords3(p1, p2, p3) = self;

        (p1, p2, p3)
    }

    pub fn unraval_vec(&self) -> Vec<(f64, f64, f64)> {
        let TriangleCoords3(p1, p2, p3) = self;

        let (p11, p12, p13) = p1.get_as_f64();
        let (p21, p22, p23) = p2.get_as_f64();
        let (p31, p32, p33) = p3.get_as_f64();

        vec![(p11, p12, p13), (p21, p22, p23), (p31, p32, p33)]
    }

    pub fn unravel_multidim_vec(&self) -> Vec<Vec<f64>> {
        let TriangleCoords3(p1, p2, p3) = self;

        let (p11, p12, p13) = p1.get_as_f64();
        let (p21, p22, p23) = p2.get_as_f64();
        let (p31, p32, p33) = p3.get_as_f64();

        vec![vec![p11, p12], vec![p21, p22], vec![p31, p32]]
    }

    pub fn unravel_triplet_vec(&self) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let TriangleCoords3(p1, p2, p3) = self;

        let (p11, p12, p13) = p1.get_as_f64();
        let (p21, p22, p23) = p2.get_as_f64();
        let (p31, p32, p33) = p3.get_as_f64();

        (vec![p11, p12], vec![p21, p22], vec![p31, p32])
    }

    pub fn get_barycentric_coords(&self, p: Point3<f64>) -> Point3<f64> {
        let (a, b, c) = self.unravel_triplet_vec();

        let mut s = vec![Point3(0.0f64, 0.0f64, 0.0f64); 2];

        for i in (2usize..0usize).rev() {
            s[i][0] = a[i] - a[i];
            s[i][1] = b[i] - a[i];
            s[i][2] = a[i] - p[i];
        }

        let u = s[0] ^ s[1];

        match u.2.abs() > 1e-2 {
            true => Point3(1.0f64 - (u.0 + u.1) / u.2, u.1 / u.2, u.0 / u.2),
            fale => Point3(-1.0, 1.0, 1.0),
        }
    }

    pub fn decompose_barycentric(&self, p: Point3<f64>) -> (f64, f64, f64) {
        let ps = p.get_as_f64();

        let (a, b, c) = self.unravel_triplet_vec();

        let a = a.as_slice();
        let b = b.as_slice();
        let c = c.as_slice();

        let (px, py, pz) = p.unravel();
        let (ax, ay, az) = (a[0], a[1], 1.0);
        let (bx, by, bz) = (b[0], b[1], 1.0);
        let (cx, cy, cz) = (c[0], c[1], 1.0);

        let bary_a = (((by - cy) * (px - cx)) + ((cx - bx) * (py - cy)))
            / (((by - cy) * (ax - cx)) + ((cx - bx) * (ay - cy)));
        let bary_b = (((cy - ay) * (px - cx)) + ((ax - cx) * (py - cy)))
            / (((by - cy) * (ax - cx)) + ((cx - bx) * (ay - cy)));
        let bary_c = 1.0 - bary_a - bary_b;

        (bary_a, bary_b, bary_c)
    }
}

impl std::ops::Index<usize> for TriangleCoords3 {
    type Output = Point3<f64>;

    fn index(&self, index: usize) -> &Self::Output {
        let (p1, p2, p3) = self.unravel_point3();

        match index {
            0 => p1,
            1 => p2,
            2 => p3,
            _ => panic!("Index {} larger than what fits", &index),
        }
    }
}

pub fn calculate_normal(v: Vec<Vertex>) -> Point3<f64> {
    let p1 = v[2] - v[0];
    let p2 = v[1] - v[0];

    p1 ^ p2
}

pub fn calculate_intensity(n: Point3<f64>, light_dir: Point3<f64>) -> f64 {
    let normalized = n.normalize();

    let inner_product = normalized * light_dir;

    inner_product
}

pub fn calculate_normal_and_intensity(v: Vec<Vertex>, light_dir: Point3<f64>) -> f64 {
    let n = calculate_normal(v);

    calculate_intensity(n, light_dir)
}

#[derive(Clone, Debug)]
pub struct UVTriplet(Point2<f64>, Point2<f64>, Point2<f64>);

impl UVTriplet {
    pub fn new(p1: Point2<f64>, p2: Point2<f64>, p3: Point2<f64>) -> Self {
        Self(p1, p2, p3)
    }

    pub fn from_vec(v: Vec<Point2<f64>>) -> Self {
        Self(v[0], v[1], v[2])
    }

    pub fn get_color(&self, p: Point3<f64>, coords: TriangleCoords3, texture: &Vec<u8>) -> u8 {
        let UVTriplet(p1, p2, p3) = *self;

        let (bary_a, bary_b, bary_c) = coords.decompose_barycentric(p);

        let (p11, p12) = p1.unravel();
        let (p21, p22) = p1.unravel();
        let (p31, p32) = p1.unravel();

        let auv = texture[*p11 as usize * *p12 as usize] as f64;
        let buv = texture[*p21 as usize * *p22 as usize] as f64;
        let cuv = texture[*p31 as usize * *p32 as usize] as f64;

        ((bary_a * auv) + (bary_b * buv) + (bary_c * cuv)) as u8
    }
}
