# raytracing

Software raytracing, based on the book series [_Ray Tracing in One
Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
by Peter Shirley, Trevor David Black, and Steve Hollasch.

## Features

- Multithreaded rendering with Rayon
- GUI interface for scene and rendering configuration

## Key Differences

The original book uses C++ and Object-Oriented design patterns, which are not directly transferable to Rust. The
following are some key differences in the Rust implementation:

- `Hit` and `Material` traits instead of abstract base classes.
    - The `enum_dispatch` crate is used to transform dynamic dispatch into static dispatch for trait objects.
- Owned materials and references are used instead of smart pointers. Rust's ownership model ensures that those are
  handled safely.
    - This and the previous point provide significant performance improvements over a more literal translation to Rust.
- GUI interface with the `egui` crate for easy configuration.
- PNG image output with the `image` crate.
- No support for Bounding Volume Hierarchies (BVH). It would require using smart pointers, which carry a performance
  penalty greater than the performance gain from using BVHs. It was added and subsequently removed after benchmarking.
