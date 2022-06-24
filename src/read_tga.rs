use embedded_graphics::pixelcolor::Rgb888;
use std::{fs, path::PathBuf};
use tinytga::Tga;

pub fn read_tga(path: PathBuf) -> Vec<u8> {
    let data = fs::read(path).unwrap();

    let tga: Tga<Rgb888> = Tga::from_slice(data.as_slice()).unwrap();

    let map = tga.as_raw().image_data().to_vec();

    map
}
