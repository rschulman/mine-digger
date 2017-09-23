use super::Vertex;

pub fn cube(pos_x: f32, pos_y: f32, pos_z: f32, edge_length: f32) -> (Vec<Vertex>, Vec<u16>) {
    // (Vertexes, Indices)
    let edge_half = edge_length * 0.5;
    let vertices: Vec<Vertex> =
        vec![// Top
             Vertex {
                 pos: [pos_x + edge_half * -1.0, pos_y + edge_half * 1.0, pos_z + edge_half * 1.0],
                 uv: [0.0, 1.0],
             },
             Vertex {
                 pos: [pos_x + edge_half * -1.0, pos_y + edge_half * -1.0, pos_z + edge_half * 1.0],
                 uv: [0.0, 0.0],
             },
             Vertex {
                 pos: [pos_x + edge_half * 1.0, pos_y + edge_half * -1.0, pos_z + edge_half * 1.0],
                 uv: [1.0, 0.0],
             },
             Vertex {
                 pos: [pos_x + edge_half * 1.0, pos_y + edge_half * 1.0, pos_z + edge_half * 1.0],
                 uv: [1.0, 1.0],
             },

             // Bottom
             Vertex {
                 pos: [pos_x + edge_half * -1.0, pos_y + edge_half * 1.0, pos_z + edge_half * -1.0],
                 uv: [0.0, 1.0],
             },
             Vertex {
                 pos: [pos_x + edge_half * -1.0,
                       pos_y + edge_half * -1.0,
                       pos_z + edge_half * -1.0],
                 uv: [0.0, 0.0],
             },

             Vertex {
                 pos: [pos_x + edge_half * 1.0, pos_y + edge_half * -1.0, pos_z + edge_half * -1.0],
                 uv: [1.0, 0.0],
             },

             Vertex {
                 pos: [pos_x + edge_half * 1.0, pos_y + edge_half * 1.0, pos_z + edge_half * -1.0],
                 uv: [1.0, 1.0],
             },

             // Front
             Vertex {
                 pos: [pos_x + edge_half * 1.0, pos_y + edge_half * -1.0, pos_z + edge_half * -1.0],
                 uv: [1.0, 0.0],
             },
             Vertex {
                 pos: [pos_x + edge_half * -1.0,
                       pos_y + edge_half * -1.0,
                       pos_z + edge_half * -1.0],
                 uv: [0.0, 0.0],
             },
             Vertex {
                 pos: [pos_x + edge_half * 1.0, pos_y + edge_half * -1.0, pos_z + edge_half * 1.0],
                 uv: [1.0, 1.0],
             },
             Vertex {
                 pos: [pos_x + edge_half * -1.0, pos_y + edge_half * -1.0, pos_z + edge_half * 1.0],
                 uv: [0.0, 1.0],
             },

             // Back
             Vertex {
                 pos: [pos_x + edge_half * 1.0, pos_y + edge_half * 1.0, pos_z + edge_half * -1.0],
                 uv: [1.0, 0.0],
             },
             Vertex {
                 pos: [pos_x + edge_half * -1.0, pos_y + edge_half * 1.0, pos_z + edge_half * -1.0],
                 uv: [0.0, 0.0],
             },
             Vertex {
                 pos: [pos_x + edge_half * 1.0, pos_y + edge_half * 1.0, pos_z + edge_half * 1.0],
                 uv: [1.0, 1.0],
             },
             Vertex {
                 pos: [pos_x + edge_half * -1.0, pos_y + edge_half * 1.0, pos_z + edge_half * 1.0],
                 uv: [0.0, 1.0],
             },

             // Right
             Vertex {
                 pos: [pos_x + edge_half * 1.0, pos_y + edge_half * 1.0, pos_z + edge_half * -1.0],
                 uv: [1.0, 0.0],
             },
             Vertex {
                 pos: [pos_x + edge_half * 1.0, pos_y + edge_half * -1.0, pos_z + edge_half * -1.0],
                 uv: [0.0, 0.0],
             },
             Vertex {
                 pos: [pos_x + edge_half * 1.0, pos_y + edge_half * 1.0, pos_z + edge_half * 1.0],
                 uv: [1.0, 1.0],
             },
             Vertex {
                 pos: [pos_x + edge_half * 1.0, pos_y + edge_half * -1.0, pos_z + edge_half * 1.0],
                 uv: [0.0, 1.0],
             },

             // Left
             Vertex {
                 pos: [pos_x + edge_half * -1.0, pos_y + edge_half * 1.0, pos_z + edge_half * -1.0],
                 uv: [1.0, 0.0],
             },
             Vertex {
                 pos: [pos_x + edge_half * -1.0,
                       pos_y + edge_half * -1.0,
                       pos_z + edge_half * -1.0],
                 uv: [0.0, 0.0],
             },
             Vertex {
                 pos: [pos_x + edge_half * -1.0, pos_y + edge_half * 1.0, pos_z + edge_half * 1.0],
                 uv: [1.0, 1.0],
             },
             Vertex {
                 pos: [pos_x + edge_half * -1.0, pos_y + edge_half * -1.0, pos_z + edge_half * 1.0],
                 uv: [0.0, 1.0],
             }];

    let indices: Vec<u16> = vec![0, 1, 2, 2, 1, 3, 4, 5, 6, 6, 5, 7, 8, 9, 10, 10, 9, 11, 12, 13,
                                 14, 14, 13, 15, 16, 17, 18, 18, 17, 19, 20, 21, 22, 22, 21, 23];

    (vertices, indices)
}
