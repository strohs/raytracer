# Rust Raytracer
A Rust implementation of the ray-tracer from the first two books of the [Ray Tracing in One Weekend](https://raytracing.github.io/)

In addition to the functionality from the book, this implementation adds multi-threading to the ray-tracer and a 
simple command line parser that will allow you to specify the image width, aspect-ratio, samples per pixel, and a 
pre-made scene number to render.

## Sample Scenes

### Building
build the `raytracer` executable with `cargo build --release`

### Running
from the command line run
> raytracer

this will generate a Cornell Box scene with a width of 1024 pixels, a 16:9 aspect ratio, and 500 samples per pixel 

The following command line options are also supported:
```
raytracer [-w WIDTH] [-p SAMPLES_PER_PIXEL] [-a ASPECT_RATIO] [-s SCENE_NUMBER]

WIDTH = width of the rendered image, defaults to 1024
SAMPLES_PER_PIXEL = number of multisamples to take for each pixel. Defaults to 500. Setting this to higher values
                    will improve image quality and increases render time.
ASPECT_RATIO = should be a floating point number >= 1.0. Defaults to 1.77  Some examples:
               1.77 = a 16:9 aspect ratio
               1.6  = a 16:10 aspect ratio
               1.33 = a 4:3 apect ratio
               1.43 = IMAX film format
               1.85 = U.S. widescreen cinema format
SCENE_NUMBER = integer between 1 - 6. let's you pick a pre-made scene from the book to render. Defaults to 6
               1 = Random Spheres
               2 = Two Perlin Spheres
               3 = Texture mapped Earth
               4 = Cornell Box
               5 = Cornell Box with smoky primitives
               6 = Final Scene (random boxes, spheres, lit by a single light)
```