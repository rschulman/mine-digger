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

use gfx::traits::FactoryExt;
use gfx::Device;
use glutin::GlContext;
use cgmath::prelude::*;
use cgmath::{Deg, Vector3, Point3, Matrix4};

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 3] = "a_Pos",
        color: [f32; 3] = "a_Color",
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
        out: gfx::RenderTarget<ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<DepthFormat> =
            gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

fn cube(pos_x: f32, pos_y: f32, pos_z: f32, edge_length: f32) -> (Vec<Vertex>, Vec<u16>) {
    // (Vertexes, Indices)
    let edge_half = edge_length * 0.5;
    let vertices: Vec<Vertex> =
        vec![Vertex {
                 pos: [pos_x + edge_half * -1.0, pos_y + edge_half * 1.0, pos_z + edge_half * 1.0],
                 color: [1.0, 0.0, 0.0],
             },
             Vertex {
                 pos: [pos_x + edge_half * -1.0, pos_y + edge_half * -1.0, pos_z + edge_half * 1.0],
                 color: [1.0, 0.0, 0.0],
             },

             Vertex {
                 pos: [pos_x + edge_half * 1.0, pos_y + edge_half * 1.0, pos_z + edge_half * 1.0],
                 color: [1.0, 0.0, 0.0],
             },

             Vertex {
                 pos: [pos_x + edge_half * 1.0, pos_y + edge_half * -1.0, pos_z + edge_half * 1.0],
                 color: [1.0, 0.0, 0.0],
             },
             Vertex {
                 pos: [pos_x + edge_half * -1.0, pos_y + edge_half * 1.0, pos_z + edge_half * -1.0],
                 color: [1.0, 0.0, 0.0],
             },
             Vertex {
                 pos: [pos_x + edge_half * -1.0,
                       pos_y + edge_half * -1.0,
                       pos_z + edge_half * -1.0],
                 color: [1.0, 0.0, 0.0],
             },

             Vertex {
                 pos: [pos_x + edge_half * 1.0, pos_y + edge_half * 1.0, pos_z + edge_half * -1.0],
                 color: [1.0, 0.0, 0.0],
             },

             Vertex {
                 pos: [pos_x + edge_half * 1.0, pos_y + edge_half * -1.0, pos_z + edge_half * -1.0],
                 color: [1.0, 0.0, 0.0],
             }];

    let indices: Vec<u16> = vec![
        0, 1, 2, 2, 1, 3,  // top
        4, 5, 6, 6, 5, 7,  // bottom
        2, 3, 6, 6, 3, 7,  // right
        0, 1, 4, 4, 1, 5,  // left
        0, 2, 4, 4, 2, 6,  // front
        1, 3, 5, 5, 3, 7   // back
    ];

    (vertices, indices)
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
    let model_mat = Matrix4::identity().into();
    let view_mat = Matrix4::look_at(Point3::new(6.0, 6.0, 6.0),
                                    Point3::new(0.0, 0.0, 0.0),
                                    Vector3::unit_z())
        .into();
    let proj_mat = cgmath::perspective(Deg(60.0f32), 1.3, 0.1, 1000.0).into();

    let locals_buffer = factory.create_constant_buffer(1);

    let (vertices, indices) = cube(0.0, 0.0, 0.0, 2.0);
    let (vertex_buffer, slice) =
        factory.create_vertex_buffer_with_slice(&vertices, &indices as &[u16]);
    let mut data = pipe::Data {
        vbuf: vertex_buffer,
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
