use std::fs::File;
use std::io::{Read, BufRead};
use std::io::BufReader;
use crate::tinypmm_error::TinyppmError;

// binary ppm header is always "P6"
const PPM_BINARY_HEADER : &str = "P6";

/// PPMImage struct is returned when the file is read
pub struct PPMImage {
    height: usize,
    width: usize,
    pixels: Vec<u32>,
}

/// public methods (accessors) for PPIMage
impl PPMImage {
    /// Returns image height
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns image width
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns image data in ARGB format (8bpp per channel)
    pub fn pixels(&self) -> &Vec<u32> {
        &self.pixels
    }
}

/// Reads specified .ppm file
/// Returns PPMImage struct containing:
/// - width: usize
/// - height: usize
/// - pixels: Vec<u32>
///
/// 24bpp bitmaps are converted to 32 bit, so it is easy to push them to framebuffer.
///
/// # Example:
/// ```rust, no_run
/// extern crate tinyppm;
///
/// let ppm_image_result = tinyppm::ppm_loader::read_image_data("my_ppm_image.ppm");
/// let ppm_image = match ppm_image_result {
/// Ok(image) => image,
/// _ => panic!("unable to read specified image file!"),
/// };
///
/// ```
pub fn read_image_data(image_name: &str) -> Result<PPMImage, TinyppmError> {
    let file = File::open(image_name)?;

    let mut reader = std::io::BufReader::new(file);
    let (width, height) = read_image_info(&mut reader)?;

    let mut rgb_buffer: Vec<u8> = Vec::with_capacity(width * height * 3);
    reader.read_to_end(rgb_buffer.as_mut())?;

    let buffer = convert_rgb_to_argb(width, height, &rgb_buffer[..]);
    Ok(PPMImage{
        width,
        height,
        pixels: buffer
    })
}

/// converts 24bpp (8 bpp per channel) into 32bpp (ARGB) image data
fn convert_rgb_to_argb(width: usize, height: usize, rgb_buffer: &[u8]) -> Vec<u32> {
    let mut buffer: Vec<u32> = vec![0; width * height];
    for index in 0..width * height {
        let pixel = index * 3;
        let r = rgb_buffer[pixel] as u32;
        let g = rgb_buffer[pixel + 1] as u32;
        let b = rgb_buffer[pixel + 2] as u32;
        let color = b + (g << 8) + (r << 16);
        buffer[index] = color;
    }
    buffer
}

/// Reads image info (header) and returns tuple with (with, height)
fn read_image_info(reader: &mut BufReader<File>) -> Result<(usize, usize), TinyppmError> {
    let mut string_buffer = String::new();
    for _i in 0..3 {
        reader.read_line(&mut string_buffer).unwrap();
    }

    let ppm_id = string_buffer.lines().nth(0usize).unwrap();
    if ppm_id != PPM_BINARY_HEADER {
        return Err(TinyppmError::new(TinyppmError::InvalidHeader));
    }

    let image_size = string_buffer.lines().nth(1usize).unwrap().to_string().clone();
    let (width, height) = extract_image_size(image_size);

    let color_depth = string_buffer.lines().nth(2usize).unwrap().to_string().clone();

    if ! is_image_24bpp(color_depth) {
        return Err(TinyppmError::new(TinyppmError::UnsupportedBPP));
    }

    Ok((width, height))
}

/// checks if image is 8bit per channel (24bpp).
fn is_image_24bpp(bpp_str: String) -> bool {
    let bpp = bpp_str.parse::<usize>().expect("image bit depth should be a number");
    bpp == 255usize
}

/// return tuple containing:
/// - image height
/// - image width
fn extract_image_size(size: String) -> (usize, usize) {
    let image_size: Vec<String> = size.split_whitespace().into_iter().map(|w| w.to_string()).collect();
    let width = image_size.first().unwrap()
        .parse::<usize>()
        .expect("image width should be a number");
    let height = image_size.last().unwrap().
        parse::<usize>()
        .expect("image height should be a number");
    (width, height)
}
