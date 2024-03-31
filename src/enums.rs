// Types and structs for describing a 2x2x2 cube's state

use std::fmt;
use three_d::{vec3, Vec3};

// ================ ENUMS ================

#[derive(Copy, Clone)]
pub enum CubeDir {
    X,
    Y,
    Z,
}

#[derive(Copy, Clone)]
pub enum CubeSide {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CubeRealSide {
    R,
    L,
    U,
    D,
    F,
    B,
}

// The corner of the cube
#[derive(Copy, Clone)]
pub enum CubeCorner {
    FUR,
    FUL,
    FDR,
    FDL,
    BUR,
    BUL,
    BDR,
    BDL,
}

// Possible (corner) cubic positions
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CornerPos {
    FUR,
    FUL,
    FDR,
    FDL,
    BUR,
    BUL,
    BDR,
    BDL,
}

impl fmt::Display for CornerPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant_name = match self {
            CornerPos::FUR => "Corner_FUR",
            CornerPos::FUL => "Corner_FUL",
            CornerPos::FDR => "Corner_FDR",
            CornerPos::FDL => "Corner_FDL",
            CornerPos::BUR => "Corner_BUR",
            CornerPos::BUL => "Corner_BUL",
            CornerPos::BDR => "Corner_BDR",
            CornerPos::BDL => "Corner_BDL",
        };
        write!(f, "{}", variant_name)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CornerTwist {
    Rot0, // 0 1 2 (no permutation = identity)
    Rot1, // 1 2 0 / 2 0 1
    Rot2, // 2 0 1 / 1 2 0
}

// impl CubeSide {
//     pub fn cube_side_to_normal(&self) -> Vec3 {
//         match self {
//             CubeSide::PosX => Vec3::unit_x(),
//             CubeSide::NegX => -Vec3::unit_x(),
//             CubeSide::PosY => Vec3::unit_y(),
//             CubeSide::NegY => -Vec3::unit_y(),
//             CubeSide::PosZ => Vec3::unit_z(),
//             CubeSide::NegZ => -Vec3::unit_z(),
//         }
//     }
// }

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

// todo: remove this
impl CornerPos {
    pub fn to_vertex(&self) -> Vec3 {
        match self {
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
}

impl CornerTwist {
    pub fn permutate(&self, rot: &Self) -> Self {
        match rot {
            Self::Rot0 => *self,
            Self::Rot1 => match self {
                Self::Rot0 => Self::Rot1,
                Self::Rot1 => Self::Rot2,
                Self::Rot2 => Self::Rot0,
            },
            Self::Rot2 => match self {
                Self::Rot0 => Self::Rot2,
                Self::Rot1 => Self::Rot0,
                Self::Rot2 => Self::Rot1,
            },
        }
    }
}

impl From<u32> for CornerPos {
    fn from(value: u32) -> Self {
        match value {
            0 => CornerPos::FUR,
            1 => CornerPos::FUL,
            2 => CornerPos::FDR,
            3 => CornerPos::FDL,
            4 => CornerPos::BUR,
            5 => CornerPos::BUL,
            6 => CornerPos::BDR,
            7 => CornerPos::BDL,
            _ => unimplemented!("Invalid order, cannot be converted to CornerPos"),
        }
    }
}

impl From<CornerPos> for u32 {
    fn from(pos: CornerPos) -> Self {
        match pos {
            CornerPos::FUR => 0,
            CornerPos::FUL => 1,
            CornerPos::FDR => 2,
            CornerPos::FDL => 3,
            CornerPos::BUR => 4,
            CornerPos::BUL => 5,
            CornerPos::BDR => 6,
            CornerPos::BDL => 7,
        }
    }
}

impl From<CornerTwist> for u32 {
    fn from(rot: CornerTwist) -> Self {
        match rot {
            CornerTwist::Rot0 => 0,
            CornerTwist::Rot1 => 1,
            CornerTwist::Rot2 => 2,
        }
    }
}

impl From<u32> for CornerTwist {
    fn from(value: u32) -> Self {
        match value {
            0 => CornerTwist::Rot0,
            1 => CornerTwist::Rot1,
            2 => CornerTwist::Rot2,
            _ => unimplemented!("Invalid order, cannot be converted to CornerTwist"),
        }
    }
}
