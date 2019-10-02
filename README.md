tinyppm
=======

Simple .ppm loader written in Rust.

This is more of a toy project, I have written, to be able to easy reuse it for some of my experiments with 2d graphics in Rust.

Usage
-----

**v.0.20.0 Note:** This version introduces proper error handling. Earlier `tinyppm` was a bit of pain as it exited when something unexpected happened (unrecognized header, unsupported color depth, non-existing/unreadable file specified...). 
 Currently `tinyppm` returns an error in case some problem occurs. It is responsibility of the consumer to decide if this is critical or not, and take appropriate action.
 
 **Note:** the above change will require some slight modification of code that relies on `tinyppm 0.1.x`.

1. Add `tinyppm` to your `Cargo.toml`
2. Call `read_image_data`:

```rust
extern crate tinyppm;

fn my_function(filename: &String) {
    let ppm_image_result = tinyppm::ppm_loader::read_image_data(filename);
    let ppm_image = match ppm_image_result {
        Ok(image) => image,
        _ => panic!("unable to read specified image file!"),
    };
    // `ppm_image` is now a struct containing image with, height and pixels 
}

```

The structure returned is defined as follows:

```rust
pub struct PPMImage {
    height: usize,
    width: usize,
    pixels: Vec<u32>,
}
```
and  it exposes 3 public methods:

```rust
    pub fn height() -> usize {
        // returns image height
    }

    pub fn width() -> usize {
        // returns image width
    }

    pub fn pixels() -> &Vec<u32> {
        // returns reference to buffer containing pixels
    } 
```

Details:
--------

- only 'raw ppm' format is supported (the most popular format of ppm. More details: [ppm format specification][ppm] ).

- `tinyppm` supports only True Color images (i.e. 24bits per pixel - 3 color channels & 8 bits per channel). After the image is read it is converted to RGB+A format (32bpp) so that it is ready to be pushed directly to framebuffer.

[ppm]: http://netpbm.sourceforge.net/doc/ppm.html

Additional examples:
--------------------

If you want to check how it works, please have a look at `ppm_viewer` which is simple image viewer written in Rust: https://github.com/MaciekTalaska/2d_effects/tree/master/ppm_viewer

License
-------
This code is released under the MIT license.
