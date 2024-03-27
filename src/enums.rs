// Types and structs for describing a 2x2x2 cube's state

use std::fmt;

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

#[derive(Copy, Clone)]
pub enum CubeRealSide {
    R,
    L,
    U,
    D,
    F,
    B,
}

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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub enum CornerTwist {
    Rot0, // 0 1 2 (no permutation = identity)
    Rot1, // 1 2 0
    Rot2, // 2 0 1
}
