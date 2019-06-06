tinyppm
=======

Simple .ppm loader written in Rust.

This is more of a toy project, I have written, to be able to easy reuse it for some of my experiments with 2d graphhics in Rust.

Usage
-----

1. Add `tinyppm` to your `Cargo.toml`
2. Call `get_image_data`:

```rust
extern crate tinyppm;

fn my_function(filename: &String) {
    let (width, height, image) = tinyppm:ppm_loader::read_image_data(filename);
    // `image` contains 32bit image data
}

```

Details:
--------

`tinyppm` supports only 'raw ppm' format (the most popular format of ppm. More details: [ppm format specification][ppm] ).

Another restriction that is important is that at them moment `tinyppm` supports reading only truecolor (24bpp - 3 color channels, 8b per channel) images. After the image is read it is converted to RGB+A format (32bpp) so that it is ready to be pushed directly to framebuffer.

[ppm]: http://netpbm.sourceforge.net/doc/ppm.html

Additional examples:
--------------------

If you want to check how it works, please have a look at `ppm_viewer` which is simple image viewer written in Rust: https://github.com/MaciekTalaska/2d_effects/tree/master/ppm_viewer

License
-------
This code is released under the MIT license.
