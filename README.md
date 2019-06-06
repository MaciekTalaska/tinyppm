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

License
-------
This code is released under the MIT license.
