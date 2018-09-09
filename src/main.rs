#[macro_use]
extern crate gfx;
extern crate alga;
extern crate gfx_core;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate imgui;
extern crate imgui_gfx_renderer;
extern crate imgui_sys;
extern crate nalgebra as na;

use gfx::traits::FactoryExt;
use gfx::Device;
use gfx::texture::Mipmap;
use gfx_window_glutin as gfx_glutin;
use glutin::dpi::{LogicalPosition, LogicalSize};
use glutin::Api::OpenGl;
use glutin::{GlContext, GlRequest};
use imgui::*;
use imgui_gfx_renderer::{Renderer, Shaders};
use na::{Matrix4, Vector3};

use std::time::Instant;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

const CLEAR_COLOR: [f32; 4] = [0.25, 0.25, 0.5, 1.0];

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (i32, i32),
    pressed: (bool, bool, bool),
    wheel: f32,
}

gfx_defines!{
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
    }

    constant Transform {
        transform: [[f32; 4];4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

fn gfx_load_texture<F, R>(
    factory: &mut F,
) -> gfx::handle::ShaderResourceView<R, [f32; 4]>
where
    F: gfx::Factory<R>,
    R: gfx::Resources,
{
    use gfx::format::Rgba8;
    let img = image::open("resources/twitter_avatar.png")
        .unwrap()
        .to_rgba();
    let (width, height) = img.dimensions();
    let kind = gfx::texture::Kind::D2(
        width as u16,
        height as u16,
        gfx::texture::AaMode::Single,
    );
    let (_, view) = factory
        .create_texture_immutable_u8::<Rgba8>(kind, Mipmap::Allocated, &[&img])
        .unwrap();
    view
}

pub fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let windowbuilder = glutin::WindowBuilder::new()
        .with_title("Triangle Example".to_string())
        .with_dimensions(LogicalSize::from((512, 512)));
    let contextbuilder = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(OpenGl, (3, 2)))
        .with_vsync(true);
    let (window, mut device, mut factory, mut color_view, mut depth_view) =
        gfx_glutin::init::<ColorFormat, DepthFormat>(
            windowbuilder,
            contextbuilder,
            &events_loop,
        );

    let pso = factory
        .create_pipeline_simple(
            include_bytes!("shader/myshader_150.glslv"),
            include_bytes!("shader/myshader_150.glslf"),
            pipe::new(),
        ).unwrap();

    let mut encoder: gfx::Encoder<_, _> =
        factory.create_command_buffer().into();
    const TRIANGLE: [Vertex; 6] = [
        Vertex {
            pos: [-0.5, -0.5, 0.0, 1.0],
            uv: [0.0, 1.0],
        },
        Vertex {
            pos: [0.5, -0.5, 0.0, 1.0],
            uv: [1.0, 1.0],
        },
        Vertex {
            pos: [0.0, 0.5, 0.0, 1.0],
            uv: [0.5, 0.0],
        },
        Vertex {
            pos: [-0.5 + 1.0, -0.5, 0.0, 1.0],
            uv: [0.0, 1.0],
        },
        Vertex {
            pos: [0.5 + 1.0, -0.5, 0.0, 1.0],
            uv: [1.0, 1.0],
        },
        Vertex {
            pos: [0.0 + 1.0, 0.5, 0.0, 1.0],
            uv: [0.5, 0.0],
        },
    ];

    let sampler = factory.create_sampler_linear();
    let texture = gfx_load_texture(&mut factory);

    let (vertex_buffer, slice) =
        factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let transform_buffer = factory.create_constant_buffer(1);

    let data = pipe::Data {
        vbuf: vertex_buffer,
        tex: (texture, sampler),
        transform: transform_buffer,
        out: color_view.clone(),
    };

    let imgui_shaders = {
        let version = device.get_info().shading_language;
        if version.is_embedded {
            if version.major >= 3 {
                Shaders::GlSlEs300
            } else {
                Shaders::GlSlEs100
            }
        } else if version.major >= 4 {
            Shaders::GlSl400
        } else if version.major >= 3 {
            Shaders::GlSl130
        } else {
            Shaders::GlSl110
        }
    };

    let mut imgui = ImGui::init();
    imgui.set_ini_filename(None);
    let mut renderer = Renderer::init(
        &mut imgui,
        &mut factory,
        imgui_shaders,
        color_view.clone(),
    ).expect("Failed to initialize renderer");

    let mut running = true;
    let mut last_frame = Instant::now();
    let mut mouse_state = MouseState::default();

    let mut pos = [0.0, 0.0];
    while running {
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::Resized(_) => {
                        gfx_window_glutin::update_views(&window, &mut color_view, &mut depth_view);
                        renderer.update_render_target(color_view.clone());
                    }
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::CursorMoved {
                        position: LogicalPosition { x, y },
                        ..
                    } => mouse_state.pos = (x as i32, y as i32),
                    glutin::WindowEvent::MouseInput { state, button, .. } => match button {
                        glutin::MouseButton::Left => {
                            mouse_state.pressed.0 = state == glutin::ElementState::Pressed
                        }
                        glutin::MouseButton::Right => {
                            mouse_state.pressed.1 = state == glutin::ElementState::Pressed
                        }
                        glutin::MouseButton::Middle => {
                            mouse_state.pressed.2 = state == glutin::ElementState::Pressed
                        }
                        _ => {}
                    },
                    glutin::WindowEvent::MouseWheel {
                        delta: glutin::MouseScrollDelta::LineDelta(_, y),
                        phase: glutin::TouchPhase::Moved,
                        ..
                    } => {
                        mouse_state.wheel = y;
                    }
                    glutin::WindowEvent::MouseWheel {
                        delta: glutin::MouseScrollDelta::PixelDelta(LogicalPosition { y, .. }),
                        phase: glutin::TouchPhase::Moved,
                        ..
                    } => {
                        mouse_state.wheel = y as f32;
                    }
                    glutin::WindowEvent::ReceivedCharacter(c) => imgui.add_input_character(c),
                    _ => {}
                }
            }
        });

        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32
            + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;

        update_mouse(&mut imgui, &mut mouse_state);

        let size_pixels = window.get_inner_size().unwrap();
        let hidpi = window.get_hidpi_factor() as f32;

        let mat: Matrix4<_> = Matrix4::identity()
            .append_translation(&Vector3::new(pos[0], pos[1], 0.0));
        let mat = mat.as_slice();
        let obj_transform: Transform = Transform {
            transform: [
                [mat[0], mat[4], mat[8], mat[12]],
                [mat[1], mat[5], mat[9], mat[13]],
                [mat[2], mat[6], mat[10], mat[14]],
                [mat[3], mat[7], mat[11], mat[15]],
            ],
        };

        let frame_size = FrameSize {
            logical_size: size_pixels.into(),
            hidpi_factor: hidpi.into(),
        };
        let ui = imgui.frame(frame_size, delta_s);
        ui.window(im_str!("Hello world"))
            .size((300.0, 250.0), ImGuiCond::FirstUseEver)
            .build(|| {
                ui.slider_float(im_str!("X"), &mut pos[0], -1.5, 1.5)
                    .build();
                ui.slider_float(im_str!("Y"), &mut pos[1], -1.5, 1.5)
                    .build();
                ui.separator();

                for y in 0..4 {
                    for x in 0..4 {
                        if x != 0 {
                            ui.same_line(0.0);
                        }
                        ui.text(im_str!(
                            "{:.1}",
                            obj_transform.transform[y][x]
                        ));
                    }
                }
            });

        encoder.clear(&color_view, CLEAR_COLOR);
        encoder
            .update_buffer(&data.transform, &[obj_transform], 0)
            .unwrap();
        encoder.draw(&slice, &pso, &data);

        renderer
            .render(ui, &mut factory, &mut encoder)
            .expect("Rendering failed");

        encoder.flush(&mut device);

        window.swap_buffers().unwrap();
        device.cleanup();
    }
}

fn update_mouse(imgui: &mut ImGui, mouse_state: &mut MouseState) {
    let scale = imgui.display_framebuffer_scale();
    imgui.set_mouse_pos(
        mouse_state.pos.0 as f32 / scale.0,
        mouse_state.pos.1 as f32 / scale.1,
    );
    imgui.set_mouse_down([
        mouse_state.pressed.0,
        mouse_state.pressed.1,
        mouse_state.pressed.2,
        false,
        false,
    ]);
    imgui.set_mouse_wheel(mouse_state.wheel / scale.1);
    mouse_state.wheel = 0.0;
}
