use crate::enums::CornerPos::*;
use crate::enums::CornerTwist::*;
use crate::enums::{CornerPos, CornerTwist};

pub struct CubicPerm {
    pub pos: CornerPos,
    pub rot: CornerTwist,
}

// Identity move
pub fn move_id(pos: CornerPos) -> CubicPerm {
    CubicPerm::new(pos, Rot0)
}

// Up face CW = Down face CW (clockwise)
pub fn move_up(pos: CornerPos) -> CubicPerm {
    // FUR -> BUR
    // BUR -> BUL
    // BUL -> FUL
    // FUL -> FUR
    match pos {
        FUR => CubicPerm::new(FUL, Rot0),
        FUL => CubicPerm::new(BUL, Rot0),
        BUL => CubicPerm::new(BUR, Rot0),
        BUR => CubicPerm::new(FUR, Rot0),
        _ => move_id(pos),
    }
}

// Front Face CW = Back face CW
pub fn move_front(pos: CornerPos) -> CubicPerm {
    match pos {
        FUR => CubicPerm::new(FDR, Rot1),
        FDR => CubicPerm::new(FDL, Rot2),
        FDL => CubicPerm::new(FUL, Rot1),
        FUL => CubicPerm::new(FUR, Rot2),
        _ => move_id(pos),
    }
}

// Right Face CW = Left face CW
pub fn move_right(pos: CornerPos) -> CubicPerm {
    match pos {
        FUR => CubicPerm::new(BUR, Rot2),
        BUR => CubicPerm::new(BDR, Rot1),
        BDR => CubicPerm::new(FDR, Rot2),
        FDR => CubicPerm::new(FUR, Rot1),
        _ => move_id(pos),
    }
}

pub fn move_inv(move_func: impl Fn(CornerPos) -> CubicPerm) -> impl Fn(CornerPos) -> CubicPerm {
    move |pos: CornerPos| {
        let perm_1 = move_func(pos);
        let perm_2 = move_func(perm_1.pos);
        let perm_3 = move_func(perm_2.pos);
        CubicPerm::new(
            perm_3.pos,
            perm_1.rot.permutate(&perm_2.rot).permutate(&perm_3.rot),
        )
    }
}

pub type MoveFunc = fn(CornerPos) -> CubicPerm;

// pub fn move_opt_up(pos: CornerPos) -> Option<CubicPerm> {
//     Some(move_up(pos))
// }
// pub fn move_opt_front(pos: CornerPos) -> Option<CubicPerm> {
//     Some(move_front(pos))
// }
// pub fn move_opt_right(pos: CornerPos) -> Option<CubicPerm> {
//     Some(move_right(pos))
// }
// pub fn move_opt_inv(
//     move_func: impl Fn(CornerPos) -> CubicPerm,
//     pos: CornerPos,
// ) -> Option<CubicPerm> {
//     Some(move_inv(move_func)(pos))
// }

impl CubicPerm {
    pub fn new(pos: CornerPos, rot: CornerTwist) -> Self {
        Self { pos, rot }
    }
}
