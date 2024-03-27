use crate::enums::*;

pub type PermId = u32; // between 0 and 3674159
pub const PERMID_MIN: PermId = 0;
pub const PERMID_MAX: PermId = 3674159;
pub const PERMID_POSCOUNT: PermId = 5040; // 7!

// ================ CUBE STATES ================

pub struct CubeState {
    bdl: CornerPos,
    positions: [CornerPos; 7],
    rotations: [CornerTwist; 6],
}

pub struct Cubelet {
    pos: CornerPos,
    rot: CornerTwist,
}

// Unused for now
pub struct FullCubeState {
    positions: [CornerPos; 8],
    rotations: [CornerTwist; 8],
}

pub struct FlattenedCube {
    sides: [CubeSide; 24],
}

// ================ MOVES ================

pub struct FullMove {
    pos_perm: [CornerPos; 8],
    rot_perm: [CornerTwist; 8],
}
