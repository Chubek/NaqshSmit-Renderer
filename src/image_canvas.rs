use crate::linear_algebra::convert_to_screen_coords;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub type CanvasType = Vec<Vec<u8>>;


#[derive(Debug)]
pub enum ImageError {
    ErrorGettingPixel,
    ErrorGettingRow,
}

pub type ResultSet = std::result::Result<(), ImageError>;

#[derive(Clone)]
pub struct Canvas(CanvasType);

impl Canvas {
    pub fn new(width: usize, height: usize, init_value: u8) -> Self {
        let ys = vec![init_value; height];
        let xs = vec![ys; width];

        Canvas(xs)
    }

    pub fn set_pixel(&mut self, xi32: i32, yi32: i32, value: u8) -> ResultSet {
        let (xi, yi) = (
            convert_to_screen_coords(xi32),
            convert_to_screen_coords(yi32),
        );

        println!("{} {} {}", xi32, yi32, value);


        let Canvas(canvas) = self;

        let y = canvas.get_mut(yi);

        match y {
            Some(xs) => {
                let x = xs.get_mut(xi);

                match x {
                    Some(pixel) => {
                        *pixel = value;

                        Ok(())
                    }
                    None => Err(ImageError::ErrorGettingPixel),
                }
            }
            None => Err(ImageError::ErrorGettingRow),
        }
    }

    pub fn get_pixel_impl(&self, x: usize, y: usize) -> u8 {
        let Canvas(canvas) = self;

        canvas[x][y]
    }

    pub fn get_size(&self) -> (usize, usize) {
        let Canvas(canvas) = self;

        let h = canvas.len();
        let w = canvas[0].len();

        (w, h)
    }

    pub fn get_map(&self) -> CanvasType {
        let Canvas(canvas) = self;

        canvas.to_vec()
    }
}
