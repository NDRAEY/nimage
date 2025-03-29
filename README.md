# NImage

NImage is a crate that provides basic image processing functions.

Inspired by [image](https://crates.io/crates/image) crate.

It's no_std compatible, but still depends on `alloc`.

# Features

- [x] TGA image import.
- [x] Drawing and getting pixels on image
- [x] Resizing images (bilinear algorithm)
- [x] Flipping images vertically and horizontally
- [x] Rotating images (by 90 deg, left and right)
- [x] Cutting (cropping) images
