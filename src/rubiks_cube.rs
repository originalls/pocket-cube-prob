use crate::enums::*;
use crate::gui::{MeshArr, MeshVec};
use std::convert::TryInto;
use three_d::*;

#[derive(Clone)]
pub struct PocketTile {
    tl: Vec3, // top left corner (0, 0)
    tr: Vec3, // top right corner (1, 0)
    bl: Vec3, // bottom left corner (0, 1)
    br: Vec3, // bottom right corner (1, 1)
    rside: CubeRealSide,
}

pub struct PocketSide {
    tl: Vec3, // top left corner (0, 0)
    tr: Vec3, // top right corner (1, 0)
    bl: Vec3, // bottom left corner (0, 1)
    // bottom right corner would be abundant
    side: CubeSide,
    facelets: [CubeRealSide; 2 * 2],
}

pub struct PocketCube {
    facelets: [CubeRealSide; 6 * 2 * 2],
}

impl CubeCorner {
    // todo: check values
    pub fn to_vertex(&self) -> Vec3 {
        match self {
            CubeCorner::FUR => vec3(1., 1., 1.),
            CubeCorner::FUL => vec3(-1., 1., 1.),
            CubeCorner::FDR => vec3(1., -1., 1.),
            CubeCorner::FDL => vec3(-1., -1., 1.),
            CubeCorner::BUR => vec3(1., 1., -1.),
            CubeCorner::BUL => vec3(-1., 1., -1.),
            CubeCorner::BDR => vec3(1., -1., -1.),
            CubeCorner::BDL => vec3(-1., -1., -1.),
        }
    }
}

impl PocketCube {
    pub fn new_default() -> Self {
        Self {
            facelets: DEFAULT_FACELETS,
        }
    }

    pub fn new(facelets: [CubeRealSide; 6 * 2 * 2]) -> Self {
        Self { facelets }
    }

    pub fn set_facelets(&mut self, facelets: [CubeRealSide; 6 * 2 * 2]) {
        self.facelets = facelets;
    }

    pub fn get_pocket_side(&self, side: CubeSide) -> PocketSide {
        let facelets = self.get_side_facelets(side);
        let (tl_corner, tr_corner, bl_corner) = match side {
            CubeSide::PosY => (CubeCorner::BUL, CubeCorner::BUR, CubeCorner::FUL),
            CubeSide::NegX => (CubeCorner::FUL, CubeCorner::FUR, CubeCorner::FDL),
            CubeSide::PosZ => (CubeCorner::FUR, CubeCorner::BUR, CubeCorner::FDR),
            CubeSide::PosX => (CubeCorner::BUR, CubeCorner::BUL, CubeCorner::BDR),
            CubeSide::NegZ => (CubeCorner::BUL, CubeCorner::FUL, CubeCorner::BDL),
            CubeSide::NegY => (CubeCorner::FDL, CubeCorner::FDR, CubeCorner::BDL),
        };

        let (tl, tr, bl) = (
            tl_corner.to_vertex(),
            tr_corner.to_vertex(),
            bl_corner.to_vertex(),
        );

        PocketSide {
            tl,
            tr,
            bl,
            side,
            facelets,
        }
    }

    fn get_side_facelets(&self, side: CubeSide) -> [CubeRealSide; 4] {
        let i = match side {
            CubeSide::PosY => 0, // U
            CubeSide::NegX => 1, // L
            CubeSide::PosZ => 2, // F
            CubeSide::PosX => 3, // R
            CubeSide::NegZ => 4, // B
            CubeSide::NegY => 5, // D
        };
        let slice = (i * 4)..(i * 4 + 4);
        self.facelets[slice]
            .try_into()
            .expect("Invalid facelets slice length")
    }

    // no need for [PocketTile; 24]
    pub fn get_all_tiles(&self) -> Vec<PocketTile> {
        let pocket_sides: Vec<CubeSide> = vec![
            CubeSide::PosX,
            CubeSide::NegX,
            CubeSide::PosY,
            CubeSide::NegY,
            CubeSide::PosZ,
            CubeSide::NegZ,
        ];
        let pocket_sides_tiles: Vec<Vec<PocketTile>> = pocket_sides
            .iter()
            .map(|x| self.get_pocket_side(*x).get_pocket_tiles().to_vec().clone())
            .collect();

        let mut pocket_tiles: Vec<PocketTile> = Vec::new();
        for i in pocket_sides_tiles.iter() {
            pocket_tiles.extend(i.clone());
        }
        pocket_tiles
    }

    pub fn get_mesh_vec(&self) -> MeshVec {
        let pocket_tiles = self.get_all_tiles();
        let mut mesh_vecs: Vec<MeshVec> = Vec::new();
        let mut index = 0;
        for i in pocket_tiles.iter() {
            let mesh_vec = i.to_mesh_arr().to_mesh_vec();
            mesh_vecs.push(mesh_vec.indexed(index));
            index += mesh_vec.vertices_count();
        }

        let mut mesh_vec: MeshVec = MeshVec::new();
        for i in mesh_vecs.iter() {
            mesh_vec = mesh_vec.concat(i);
        }

        mesh_vec
    }
}

impl PocketSide {
    fn new(
        &self,
        tl: Vec3,
        tr: Vec3,
        bl: Vec3,
        side: CubeSide,
        facelets: [CubeRealSide; 2 * 2],
    ) -> Self {
        PocketSide {
            tl,
            tr,
            bl,
            side,
            facelets,
        }
    }

    fn get_delta_x_y(&self) -> (Vec3, Vec3) {
        let delta_x = self.tr - self.tl;
        let delta_y = self.bl - self.tl;
        (delta_x, delta_y)
    }

    fn get_pocket_tiles(&self) -> [PocketTile; 4] {
        let (delta_x, delta_y) = self.get_delta_x_y();
        let mut tiles: Vec<PocketTile> = Vec::new();
        // let mut tiles: [PocketTile; 4] = Default::default();

        let border = 0.02;
        let tile_size = 0.5 - border * 2.;
        let tile_delta_x = delta_x * tile_size;
        let tile_pad_x = delta_x * border;
        let tile_delta_y = delta_y * tile_size;
        let tile_pad_y = delta_y * border;

        for j in 0..2 {
            for i in 0..2 {
                let index = i + j * 2;
                let tl = self.tl
                    + tile_pad_x
                    + tile_pad_y
                    + (tile_delta_x + tile_pad_x * 2.) * (i as f32)
                    + (tile_delta_y + tile_pad_y * 2.) * (j as f32);
                let tr = tl + tile_delta_x;
                let bl = tl + tile_delta_y;
                let br = tl + tile_delta_x + tile_delta_y;
                let rside = self.facelets[index];

                tiles.push(PocketTile {
                    tl,
                    tr,
                    bl,
                    br,
                    rside,
                })
            }
        }

        tiles.try_into().unwrap_or_else(|v: Vec<PocketTile>| {
            panic!("Expected a tiles Vec of length 4 but it was {}", v.len())
        })
    }
}

impl PocketTile {
    pub fn to_mesh_arr(&self) -> MeshArr<4> {
        let vertices = [self.tl, self.tr, self.bl, self.br];
        let colors = [self.rside.to_srgba(); 4];
        let indices = vec![0, 1, 2, 1, 3, 2];

        MeshArr::<4>::new(vertices, indices, colors)
    }
}

// pub const RubiksCubeDefault = RubiksCube

const DEFAULT_FACELETS: [CubeRealSide; 24] = [
    CubeRealSide::U,
    CubeRealSide::U,
    CubeRealSide::U,
    CubeRealSide::U,
    CubeRealSide::L,
    CubeRealSide::L,
    CubeRealSide::L,
    CubeRealSide::L,
    CubeRealSide::F,
    CubeRealSide::F,
    CubeRealSide::F,
    CubeRealSide::F,
    CubeRealSide::R,
    CubeRealSide::R,
    CubeRealSide::R,
    CubeRealSide::R,
    CubeRealSide::B,
    CubeRealSide::B,
    CubeRealSide::B,
    CubeRealSide::B,
    CubeRealSide::D,
    CubeRealSide::D,
    CubeRealSide::D,
    CubeRealSide::D,
];
