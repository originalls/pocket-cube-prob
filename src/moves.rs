use crate::types::CornerPos::*;
use crate::types::*;

pub const Identity: FullMove = FullMove {
    posPerm: [FUR, FUL, FDR, FDL, BUR, BUL, BDR, BDL],
    rotPerm: [Rot0, Rot0, Rot0, Rot0, Rot0, Rot0, Rot0, Rot0],
};

pub const Up: FullMove = FullMove {
    // FUR -> BUR
    // BUR -> BUL
    // BUL -> FUL
    // FUL -> FUR
    posPerm: [BUR, FUR, FDR, FDL, BUL, FUL, BDR, BDL],
    rotPerm: [Rot0, Rot0, Rot0, Rot0, Rot0, Rot0, Rot0, Rot0],
};

pub const Identity: FullMove = FullMove {
    posPerm: [FUR, FUL, FDR, FDL, BUR, BUL, BDR, BDL],
    rotPerm: [Rot0, Rot0, Rot0, Rot0, Rot0, Rot0, Rot0, Rot0],
};
