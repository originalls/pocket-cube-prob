use crate::{enums::*, gui::MeshVec};
use three_d::*;

/*

a ––– b
|     |
d ––– c

*/

#[derive(Copy, Clone)]
pub struct Cube {
    pub center: Vec3,
    pub size: f32,
}

pub struct CubeMesh {
    pub positions: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub colors: Vec<Srgba>,
}

#[derive(Copy, Clone)]
pub struct ColoredCubeSide {
    pub color: Srgba,
    pub side: CubeSide,
}

/*

======== SIDES ========
     0: +x  1: -x
     2: +y  3: -y
     4: +z  5: -z
=======================

*/

impl CubeMesh {
    pub fn new(cube: Cube) -> Self {
        let mut positions = vec![
            vec3(1., 1., 1.),    // +x +y +z
            vec3(1., 1., -1.),   // +x +y -z
            vec3(1., -1., 1.),   // +x -y +z
            vec3(1., -1., -1.),  // +x -y -z
            vec3(-1., 1., 1.),   // -x +y +z
            vec3(-1., 1., -1.),  // -x +y -z
            vec3(-1., -1., 1.),  // -x -y +z
            vec3(-1., -1., -1.), // -x -y -z
        ];
        for vertex in positions.iter_mut() {
            *vertex *= cube.size;
        }
        let indices = vec![
            0, 1, 2, 2, 1, 3, // Front face
            0, 2, 4, 4, 2, 6, // Right face
            4, 6, 5, 5, 6, 7, // Back face
            5, 7, 1, 1, 7, 3, // Left face
            0, 4, 1, 1, 4, 5, // Top face
            2, 3, 6, 6, 3, 7, // Bottom face
        ];

        let colors = vec![Srgba::BLACK; 8];

        Self {
            positions,
            indices,
            colors,
        }
    }

    pub fn to_vec_mesh(&self) -> MeshVec {
        MeshVec {
            vertices: self.positions.clone(),
            indices: self.indices.clone(),
            colors: self.colors.clone(),
        }
    }
}

impl From<CubeSide> for CubeDir {
    fn from(side: CubeSide) -> CubeDir {
        match side {
            CubeSide::PosX | CubeSide::NegX => CubeDir::X,
            CubeSide::PosY | CubeSide::NegY => CubeDir::Y,
            CubeSide::PosZ | CubeSide::NegZ => CubeDir::Z,
        }
    }
}
