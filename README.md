# Imgui / GFX Example

A Rust demo application combining [Dear ImGui](https://github.com/Gekkio/imgui-rs) with the [gfx](https://github.com/gfx-rs/gfx) graphics library. It renders texture-mapped triangles using OpenGL and overlays an ImGui window with sliders to interactively control the triangle position. The project was born from mashing together [the LearningGfx tutorial](https://wiki.alopex.li/LearningGfx) and [the `support_gfx` example in imgui-rs](https://github.com/Gekkio/imgui-rs/blob/f7ffac7c8d4abb14896dbae4c02f44205a465ff8/imgui-examples/examples/support_gfx/mod.rs).

![Screenshot](https://github.com/WimbledonLabs/imgui_gfx_example/raw/master/resources/example.png)

## Building and Running

Requires a working [Rust](https://www.rust-lang.org/tools/install) toolchain.

```sh
cargo build
cargo run
```

## Features

- Renders textured triangles via `gfx` with custom GLSL 150 vertex and fragment shaders
- Loads a PNG texture at runtime using the `image` crate
- Overlays a Dear ImGui UI with X/Y sliders to translate the triangles and a live view of the transform matrix
- Handles mouse input, window resizing, and HiDPI scaling via `glutin`
- Automatic GLSL shader version selection for the ImGui renderer based on the available OpenGL version

## Limitations

- This is a standalone example application, not a reusable library
- Uses older dependency versions (`gfx` 0.17, `imgui` 0.0.20, `glutin` 0.18) that may not compile with the latest Rust toolchain without updates
- Minimal error handling (`unwrap()` throughout)
- Texture path (`resources/twitter_avatar.png`) is hardcoded
- Requires OpenGL 3.2+ (GLSL 150 shaders)

## History

Development took place on 2018-09-08:

- 2018-09-08 — Initial commit with full application: gfx-rs rendering pipeline, Dear ImGui overlay with position sliders, custom GLSL 150 shaders, texture loading, mouse/window event handling, and HiDPI support
- 2018-09-08 — Added example screenshot for the README
