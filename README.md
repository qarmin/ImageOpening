## Image opener
Simple tool to measure time of opening of image files.  

### How to use?
Compile app
```
cargo build
cargo build --release
```
and get binary from target folder:
```
target/debug/image_opening /home/rafal/photos
target/release/image_opening /home/rafal/photos2
```

### Opening speed of images
Current(0.23) image crate open images very slowly(Just opening with `image::open()` without any computations)

|Photo| Size | Dimension |  Debug opening time | Release opening time|
|:---:|:---:|:---:|:---:|:---:|
|PNG|4,3MB|3840x2592|9461ms|191ms|
|PNG|191,5kB|493x333|174ms|4ms|
|JPG|810,2kB|3840x2592|6633ms|160ms|

### License
MIT
