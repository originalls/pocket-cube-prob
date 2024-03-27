use crate::enums::*;
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
    pub positions: Positions,
    pub indices: Indices,
}

#[derive(Clone)]
pub struct ColoredCube {
    sides: Vec<ColoredCubeSide>,
    border: f32,
}

pub struct PocketCube {
    colors_flat: [Srgba; 24],
    colored_cubes: Vec<ColoredCube>,
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

fn cube_side_to_normal(side: &CubeSide) -> Vec3 {
    match side {
        CubeSide::PosX => Vec3::unit_x(),
        CubeSide::NegX => -Vec3::unit_x(),
        CubeSide::PosY => Vec3::unit_y(),
        CubeSide::NegY => -Vec3::unit_y(),
        CubeSide::PosZ => Vec3::unit_z(),
        CubeSide::NegZ => -Vec3::unit_z(),
    }
}

fn cube_face_to_vertices_border(side: &CubeSide, border: f32) -> [Vec3; 4] {
    let normal = cube_side_to_normal(side);
    let factor = 1. - border;

    let (cart1, cart2) = match side.to_abs() {
        CubeDir::X => (Vec3::unit_y(), Vec3::unit_z()),
        CubeDir::Y => (Vec3::unit_x(), Vec3::unit_z()),
        CubeDir::Z => (Vec3::unit_x(), Vec3::unit_y()),
    };

    let mut vertices: Vec<Vec3> = Vec::new();

    let twos: [f32; 2] = [-1., 1.];
    for i in twos {
        for j in twos {
            let vertex = normal + (i * cart1 + j * cart2) * factor;
            vertices.push(vertex);
        }
    }

    let mut iter = vertices.into_iter();

    [
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ]
}

fn cube_face_to_vertices(side: &CubeSide) -> [Vec3; 4] {
    cube_face_to_vertices_border(side, 0.)
}

fn cube_corner_to_vertex(corner: &CornerPos) -> Vec3 {
    match corner {
        CornerPos::FUR => vec3(1., 1., 1.),
        CornerPos::FUL => vec3(-1., 1., 1.),
        CornerPos::FDR => vec3(1., -1., 1.),
        CornerPos::FDL => vec3(-1., -1., 1.),
        CornerPos::BUR => vec3(1., 1., -1.),
        CornerPos::BUL => vec3(-1., 1., -1.),
        CornerPos::BDR => vec3(1., -1., -1.),
        CornerPos::BDL => vec3(-1., -1., -1.),
    }
}

fn cube_corner_vertex_to_faces(vertex: Vec3) -> Vec<CubeSide> {
    let mut sides: Vec<CubeSide> = Vec::new();
    if vertex.x == 1. {
        sides.push(CubeSide::PosX);
    } else if vertex.x == -1. {
        sides.push(CubeSide::NegX);
    }
    if vertex.y == 1. {
        sides.push(CubeSide::PosY);
    } else if vertex.y == -1. {
        sides.push(CubeSide::NegY);
    }
    if vertex.z == 1. {
        sides.push(CubeSide::PosZ);
    } else if vertex.z == -1. {
        sides.push(CubeSide::NegZ);
    }
    sides
}

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

        Self {
            positions: Positions::F32(positions),
            indices: Indices::U8(indices),
        }
    }
}

impl ColoredCube {
    pub fn new(sides: &Vec<ColoredCubeSide>, border: f32) -> Self {
        Self {
            sides: sides.clone(),
            border,
        }
    }

    pub fn get_vertices(&self) -> Vec<Vec3> {
        let mut vertices: Vec<Vec3> = Vec::new();
        for colored_side in self.sides.iter() {
            let side = colored_side.side;
            let face_vertices = cube_face_to_vertices(&side);
            vertices.extend(face_vertices.to_vec());
        }

        vertices
    }

    pub fn get_indices(&self) -> Vec<u8> {
        let mut indices: Vec<u8> = Vec::new();
        for (i, &_) in self.sides.iter().enumerate() {
            let mut def_indices: Vec<u8> = vec![0, 1, 2, 1, 2, 3];
            for j in def_indices.iter_mut() {
                *j += (i as u8) * 4;
            }

            indices.extend(def_indices);
        }

        indices
    }

    pub fn get_colors(&self) -> Vec<Srgba> {
        let mut colors: Vec<Srgba> = Vec::new();
        for colored_side in self.sides.iter() {
            let color = colored_side.color;
            for _ in 0..4 {
                colors.push(color);
            }
        }

        colors
    }
}

impl CubeSide {
    pub fn to_abs(&self) -> CubeDir {
        match self {
            CubeSide::PosX | CubeSide::NegX => CubeDir::X,
            CubeSide::PosY | CubeSide::NegY => CubeDir::Y,
            CubeSide::PosZ | CubeSide::NegZ => CubeDir::Z,
        }
    }
}

impl PocketCube {
    pub fn new(colors_flat: &[Srgba; 24]) -> Self {
        let mut flat_indices: [(CornerPos, [u8; 3]); 8] = [
            (CornerPos::FUR, [3, 9, 12]),
            (CornerPos::FUL, [2, 5, 8]),
            (CornerPos::FDR, [11, 14, 21]),
            (CornerPos::FDL, [7, 10, 20]),
            (CornerPos::BUR, [1, 16, 13]),
            (CornerPos::BUL, [0, 4, 17]),
            (CornerPos::BDR, [15, 18, 23]),
            (CornerPos::BDL, [6, 22, 19]),
        ];
        let mut colored_cubes: Vec<ColoredCube> = Vec::new();
        for i in 0..8 {
            let x = flat_indices[i];
            let vertex = cube_corner_to_vertex(&x.0);
            let faces = cube_corner_vertex_to_faces(vertex);
            let colored_cube_sides: Vec<ColoredCubeSide> = faces
                .iter()
                .enumerate()
                .map(|(index, face)| {
                    let color_index = x.1[index];
                    let color = colors_flat[color_index as usize];
                    ColoredCubeSide { color, side: *face }
                })
                .collect();
            let colored_cube = ColoredCube::new(&colored_cube_sides, 0.1);
            colored_cubes.push(colored_cube);
        }
        Self {
            colors_flat: *colors_flat,
            colored_cubes,
        }
    }

    pub fn get_mesh_data(self: &Self, color_set: Vec<Srgba>) -> (Vec<u8>, Vec<Vec3>, Vec<Srgba>) {
        // color_set will be used for optimization
        let index = 0;
        let mut indices: Vec<u8> = Vec::new();
        let mut vertices: Vec<Vec3> = Vec::new();
        let mut colors: Vec<Srgba> = Vec::new();
        for i in 1..8 {
            if let Some(ccube) = self.colored_cubes.get(i) {
                let cube_indices: Vec<u8> = ccube.get_indices().iter().map(|x| index + x).collect();
                let cube_vertices = ccube.get_vertices();
                let cube_colors: Vec<Srgba> = ccube.get_colors();

                indices.extend(cube_indices);
                vertices.extend(cube_vertices);
                colors.extend(cube_colors);
            }
        }

        (indices, vertices, colors)
    }
}
