# Safe Rust bindings to the [Skia Graphics Library](https://skia.org/).

[![Build Status](https://dev.azure.com/pragmatrix-github/rust-skia/_apis/build/status/rust-skia.rust-skia?branchName=master)](https://dev.azure.com/pragmatrix-github/rust-skia/_build/latest?definitionId=2&branchName=master)

Skia Submodule Status: chrome/m75 ([pending changes][skiapending]).

[skiapending]: https://github.com/google/skia/compare/847d55be4e6273fc3cd9c0b30c7bfc8a2d6575b7...chrome/m75

## Goals

This project attempts to provide _up to date_ safe bindings that bridge idiomatic Rust with Skia's C++ API on all major desktop, mobile, and [WebAssembly](https://en.wikipedia.org/wiki/WebAssembly) platforms, including GPU rendering support for [Vulkan](https://en.wikipedia.org/wiki/Vulkan_(API)), [Metal](https://en.wikipedia.org/wiki/Metal_(API)), and [OpenGL](https://en.wikipedia.org/wiki/OpenGL).

## Building

Note that the information in this section is preliminary. Please open an issue for any build problem.

This project requires LLVM, python, and git to build.

To test if LLVM is installed with the correct version, use `clang --version`. Currently, version 7.0.1 is required, or - on macOS X - Apple LLVM Version 10 should do, too.

For python, at least version 2.7 should be available. Use `python --version` to see what's there.

### macOS X

- Install the XCode command line tools with `xcode-select --install`.
- **macOS X 10.14 (Mojave)**: install the SDK headers: `/Library/Developer/CommandLineTools/Packages/macOS_SDK_headers_for_macOS_10.14.pkg`, otherwise the binding generation will fail with `'TargetConditionals.h' file not found`.
- Alternatively, install LLVM 7.0.1 via `brew install llvm@7` and then set `PATH`, `CPPFLAGS`, and `LDFLAGS` like instructed.

### Windows

- Be sure the `git` command line tool is installed.
- Install the [official LLVM 7.0.1](http://releases.llvm.org/download.html) distribution.
- msys:
  - Install one of the Python2 packages, for example `mingw-w64-x86_64-python2`.
  - LLVM is _always_ picked up from `C:/Program Files/LLVM`, so be sure it's available from there.
- without msys:
  - Download and install Python version 2 from [python.org](https://www.python.org/downloads/release/python-2716/).

### Linux

- LLVM should be installed out of the box, if not, install version 7.0.1.

Then use:

`cargo build -vv`

Under Linux, OpenGL libraries _may_ be missing, if that is the case, install OpenGL drivers for you graphics card, or install a mesa OpenGL package like `libgl1-mesa-dev`.

Please share your build experience so that we can try to automate the build and get to the point where `cargo build` _is_ sufficient to build the bindings _including_ Skia, and if that is not possible, clearly prompts to what's missing.

To simplify and speed up the build, we also plan to provide prebuilt binaries for some of the major platforms ([#49](https://github.com/rust-skia/rust-skia/issues/49)).

### Feature `vulkan`

Vulkan support can be enabled by setting the Cargo feature `default = ["vulkan"]` in `skia-safe/Cargo.toml`, which will cause a rebuild of Skia. To render the examples with Vulkan use `cargo run --example skia-org -- [OUTPUT_DIR] --driver vulkan`.

Note that Vulkan drivers need to be available. On Windows, they are most likely available already, on Linux [this article on linuxconfig.org](<https://linuxconfig.org/install-and-test-vulkan-on-linux>) might get you started, and on macOS with Metal support, [install the Vulkan SDK](<https://vulkan.lunarg.com/sdk/home>) for Mac and configure MoltenVK by setting the `DYLD_LIBRARY_PATH`, `VK_LAYER_PATH`, and `VK_ICD_FILENAMES` environment variables as described in `Documentation/getting_started_macos.html`.

## Examples

The examples are taken from [Skia's website](https://skia.org/) and [ported to the Rust API](skia-safe/examples/skia-org).

If you were able to build the project, run

`cargo run --example skia-org -- [OUTPUT_DIR]` 

to generate some Skia drawn PNG images in the directory `OUTPUT_DIR`. To render with OpenGL, use

`cargo run --example skia-org -- [OUTPUT_DIR] --driver opengl`

And `cargo run --example skia-org -- --help` shows the drivers that are currently supported.

## Status

### Crate

An official crate is not yet available. We've created [a Milestone](https://github.com/rust-skia/rust-skia/milestone/1) on Github's issue tracker to track the progress.

### Platforms

- [x] Windows
- [x] Linux Ubuntu 16 (18 should work, too).
- [x] macOS X
- [ ] WebAssembly: [#42](https://github.com/rust-skia/rust-skia/pull/42) (help wanted).
- [ ] Android
- [ ] iOS

### Bindings

Skia is a large library. While we strive to bind all of the C++ APIs, it's nowhere complete yet. 

We do support most of the SkCanvas, SkPaint, and SkPath and related APIs and are trying to make the examples from the [skia.org](https://skia.org/) website work.

### Features

- [x] Vector Graphics: Matrix, Rect, Point, Size, etc.
- [x] Basic Drawing: Surface, Canvas, Paint, Path.
- [x] Basic Effects and Shaders.
- [x] PDF
- [x] SVG
- [ ] Animation
- [x] Vulkan
- [x] OpenGL
- [ ] Metal

## This project needs contributions!

If you'd like to help with the bindings, take a look at the [Wiki](https://github.com/rust-skia/rust-skia/wiki) to get started and create an issue to avoid duplicate work. For smaller tasks, grep for "TODO" in the source code. And for heroic work, check out the label [help wanted](https://github.com/rust-skia/rust-skia/labels/help%20wanted). And if you like to help making the Rust API nicer to use, look out for open issues with the label [api ergonomics](https://github.com/rust-skia/rust-skia/issues?q=is%3Aissue+is%3Aopen+label%3A%22api+ergonomics%22).

## Maintainers

- LongYinan (@Brooooooklyn)
- Armin (@pragmatrix)

## License

MIT

  
