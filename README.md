# GBRT
This is a project which aims to build an efficient photorealistic software renderer in Rust.  
The project aims to provide an efficient implementation of a raytracing algorithm, with a focus on code readability and maintainability.  
Here, I took a load of inspiration from ["Ray Tracing in One Weekend"](https://raytracing.github.io/books/RayTracingInOneWeekend.html) and the other books of the trilogy and as such, if you're interested in the topic of raytracing, I really suggest you to read them.

<p float="left">
    <img width="49%" src="https://raw.githubusercontent.com/giulianbiolo/gbrt/master/outputs/spheres_in_sphere.png">
    <img width="49%" src="https://raw.githubusercontent.com/giulianbiolo/gbrt/master/outputs/spheres_jet.png">
    <img width="49%" src="https://raw.githubusercontent.com/giulianbiolo/gbrt/master/outputs/skybox_winter.png">
    <img width="49%" src="https://raw.githubusercontent.com/giulianbiolo/gbrt/master/outputs/stormtroopers.png">
</p>

Features
========

General
---------

* Written in Rust
* Developed for any OS
* Highly optimized using SSE and AVX intrinsics
* Parsing scene description from a YAML file
  
Geometry
--------

* Bounding Volume Hierarchy (BVH) used for scene and mesh traversal
* Supported shape types: triangle meshes, sphere, box, rectangle

Lighting
--------

* Supported light types: diffuse lights of any supported geometry

Materials
---------

* Default materials supported: Metal, Lambertian, Dielectric, Plastic
* Transparency and refraction
* Normal mapping support

Textures
--------

* Some very basic procedural textures
* 2D bitmap textures
* Supported file formats: all formats supported by the image-rs crate

Getting Started
===============

To get started, clone the repository and run the following command in the root directory:

    $ cargo run --release

This will build and run the project, and output a PNG image of the rendered basic scene.
To render a custom scene defined by a yaml config file run the following:

    $ cargo run --release -- configs/your_config.yaml

To build a more optimized version of the code you can also specify:

    $ cargo run --release --target x86_64-pc-windows-msvc -- configs/your_config.yaml   # If you're running on Windows
    $ cargo run --release --target x86_64-unknown-linux-gnu -- configs/your_config.yaml # If you're running on Linux

TODO list
=========

* Working now on implementing support of light transport algorithms:
  * Naive Path Tracing (sampling only BSDF)
  * Path Tracing with multiple importance sampling (sampling both lights and material BSDF)
  * Bidirectional Path Tracing (with MIS)
* Working now on implementing support for:
  * Physically based BSDFs: diffuse, metal, dielectric, plastic
  * Cook-Torrance BSDF for specular reflection with GGX normal distribution
* Better material model and multilayer materials (e.g. introduce Disney-like "principled" material)
* Volumetric rendering
* Optimize traversal and shading with SSE/AVX [ Would probably mean a complete rewrite and as such is a very long term goal ]

Contributing
------------

If you're interested in contributing to this project, please feel free to open a pull request or an issue regarding your ideas.

License
-------

This project is licensed under the MIT License.
