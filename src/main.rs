// Copyright 2015 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate cgmath;
extern crate image;

use gfx::traits::FactoryExt;
//use gfx::traits::FactoryExt::create_sample_linear;
use gfx::{Device, Factory};
use gfx::format::Rgba8;
use glutin::GlContext;
use cgmath::prelude::*;
use cgmath::{Deg, Vector3, Point3, Matrix4};

mod cube;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 3] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
    }

    constant Locals {
        model: [[f32; 4]; 4] = "u_Model",
        view: [[f32; 4]; 4] = "u_View",
        proj: [[f32; 4]; 4] = "u_Proj",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        model: gfx::Global<[[f32; 4]; 4]> = "u_Model",
        view: gfx::Global<[[f32; 4]; 4]> = "u_View",
        proj: gfx::Global<[[f32; 4]; 4]> = "u_Proj",
        tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<DepthFormat> =
            gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

pub fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window_config = glutin::WindowBuilder::new()
        .with_title("Mine Digger".to_string())
        .with_dimensions(1024, 768);
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_config, context, &events_loop);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let pso = factory.create_pipeline_simple(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"),
    "/src/shader/rect_150.glslv")),
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"),
    "/src/shader/rect_150.glslf")),
    pipe::new())
        .unwrap();

    // Load up model, view, and projection transform matrices
    let model_mat = Matrix4::identity().into();
    let view_mat = Matrix4::look_at(Point3::new(6.0, 6.0, 6.0),
    Point3::new(0.0, 0.0, 0.0),
    Vector3::unit_z())
        .into();
    let proj_mat = cgmath::perspective(Deg(60.0f32), 1.3, 0.1, 1000.0).into();

    // Load a texture
    let img = image::open("textures/dirt.png").unwrap().to_rgba();
    let (img_width, img_height) = img.dimensions();
    let tex_type = gfx::texture::Kind::D2(img_width as u16,
                                          img_height as u16,
                                          gfx::texture::AaMode::Single);
    let (_, view) = factory.create_texture_immutable_u8::<Rgba8>(tex_type, &[&img]).unwrap();
    let sampler = factory.create_sampler_linear();


    let locals_buffer = factory.create_constant_buffer(1);

    let (vertices, indices) = cube::cube(0.0, 0.0, 0.0, 2.0);
    let (vertex_buffer, slice) =
        factory.create_vertex_buffer_with_slice(&vertices, &indices as &[u16]);
    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        tex: (view, sampler),
        locals: locals_buffer,
        model: model_mat,
        view: view_mat,
        proj: proj_mat,
        out: main_color,
        out_depth: main_depth.clone(),
    };


    let mut running = true;
    while running {
        // fetch events
        events_loop.poll_events(|event| if let glutin::Event::WindowEvent { event, .. } = event {
            match event {
                glutin::WindowEvent::KeyboardInput {
                    input: glutin::KeyboardInput {
                        virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                        .. },
                        ..
                } | glutin::WindowEvent::Closed => running = false,
                glutin::WindowEvent::Resized(width, height) => {
                    window.resize(width, height);
                    gfx_window_glutin::update_views(&window, &mut data.out, &mut main_depth);
                }
                _ => (),
            }
        });

        // draw a frame
        let locals = Locals {
            model: model_mat,
            view: view_mat,
            proj: proj_mat,
        };
        encoder.update_buffer(&data.locals, &[locals], 0).unwrap();
        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.clear_depth(&data.out_depth, 1.0);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
