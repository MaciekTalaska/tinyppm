use std::fs::File;
use std::io::{Read, BufRead};
use std::io::BufReader;
use crate::tinypmm_error::TinyppmError;

const PPM_BINARY_HEADER : &str = "P6";              // binary ppm header is always "P6"

pub struct PPMImage {
    height: usize,
    width: usize,
    pixels: Vec<u32>,
}

// TODO: change the in-code documentation (this docstring)
// TODO: if all is Ok, return struct instead of tuple
/// Reads specified .ppm file
/// Returns (width: usize, height: usize, pixels: Vec<u32>)
///
/// 24bpp bitmaps are converted to 32 bit, so it is easy to push them to framebuffer.
///
/// # Examples
/// ```rust, no_run
/// extern crate tinyppm;
///
/// // some code here...
///
/// fn my_function(filename: &str) {
///     let (width, height, image) = tinyppm:ppm_loader::read_image_data(filename);
///     // `image` contains 32bit image data
/// }
///
/// ```
pub fn read_image_data(image_name: &str) -> Result<(usize, usize, Vec<u32>), TinyppmError> {
    let file = File::open(image_name)?;

    let mut reader = std::io::BufReader::new(file);
    let (width, height) = read_image_info(&mut reader)?;

    let mut rgb_buffer: Vec<u8> = Vec::with_capacity(width * height * 3);
    let read_bytes = reader.read_to_end(rgb_buffer.as_mut())?;

    if read_bytes != width * height * 3 {
        return Err(TinyppmError::new(TinyppmError::FileSizeMismatch));
    }

    let buffer = convert_rgb_to_argb(width, height, &mut rgb_buffer);
    Ok((width, height, buffer))
}

/// converts 24bpp (8 bpp per channel) into 32bpp (ARGB) image data
fn convert_rgb_to_argb(width: usize, height: usize, rgb_buffer: &mut Vec<u8>) -> Vec<u32> {
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
