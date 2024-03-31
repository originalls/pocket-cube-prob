use crate::enums::CornerPos::*;
use crate::enums::CubeRealSide::*;
use crate::enums::{CornerPos, CornerTwist, CubeRealSide};
use crate::moves::CubicPerm;
use crate::moves::{move_front, move_right, move_up};
use crate::perm::id_to_index_arr;
use crate::perm::index_arr_to_id;
use crate::perm::index_arr_to_permutation;
use crate::perm::permutation_to_index_arr;
use crate::perm::smushed_to_array;
use crate::types::PermId;
use crate::types::PosId;
use crate::types::RotId;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Cubic {
    sides: [CubeRealSide; 3],
    orig_pos: CornerPos,
    curr_pos: CornerPos,
    rot: CornerTwist,
}

impl Cubic {
    pub fn new(pos: CornerPos, sides: [CubeRealSide; 3]) -> Self {
        Self {
            sides,
            orig_pos: pos,
            curr_pos: pos,
            rot: CornerTwist::Rot0,
        }
    }

    pub fn rotate(&mut self, rot: CornerTwist) {
        self.rot = self.rot.permutate(&rot);
    }

    pub fn set_pos(&mut self, pos: CornerPos) {
        self.curr_pos = pos;
    }
    pub fn get_pos(&self) -> CornerPos {
        self.curr_pos
    }
    pub fn set_rot(&mut self, rot: CornerTwist) {
        self.rot = rot;
    }
    pub fn get_rot(&self) -> CornerTwist {
        self.rot
    }
    pub fn get_sides_rotated(&self) -> [CubeRealSide; 3] {
        match self.rot {
            CornerTwist::Rot0 => [self.sides[0], self.sides[1], self.sides[2]],
            CornerTwist::Rot1 => [self.sides[2], self.sides[0], self.sides[1]],
            CornerTwist::Rot2 => [self.sides[1], self.sides[2], self.sides[0]],
        }
    }
}

#[derive(Clone)]
pub struct PocketCube {
    pub cubics: [Cubic; 8],
    default_cubics: [Cubic; 8],
}

impl PocketCube {
    pub fn new() -> Self {
        let default_cubics = [
            Cubic::new(CornerPos::FUR, [U, F, R]),
            Cubic::new(CornerPos::FUL, [U, L, F]),
            Cubic::new(CornerPos::FDR, [D, R, F]),
            Cubic::new(CornerPos::FDL, [D, F, L]),
            Cubic::new(CornerPos::BUR, [U, R, B]),
            Cubic::new(CornerPos::BUL, [U, B, L]),
            Cubic::new(CornerPos::BDR, [D, B, R]),
            Cubic::new(CornerPos::BDL, [D, L, B]),
        ];
        Self {
            cubics: default_cubics.clone(),
            default_cubics,
        }
    }

    pub fn apply_id(&mut self, id: PermId) {
        let pos_indexes = id_to_index_arr::<u32>(&id.get_pos_id().get_id(), &7);
        let pos_perm = index_arr_to_permutation(&self.get_default_cubics().to_vec(), &pos_indexes);

        let rot_ids = smushed_to_array::<u32, 6>(id.get_rot_id().get_id(), |x: u32| x);
        let mut rot_sum = 0;
        for i in &rot_ids {
            rot_sum += i;
        }
        let last_rot: CornerTwist = (((rot_sum % 3) + 3) % 3).into();

        for i in 0..7 {
            self.cubics[i].set_pos(pos_perm[i].get_pos());
        }

        for i in 0..6 {
            self.cubics[i].set_rot(rot_ids[i].into());
        }
        self.cubics[6].set_rot(last_rot);
    }

    fn reduce_cubics_6(&self, arr: &[Cubic; 8]) -> [Cubic; 6] {
        [
            arr[0].clone(),
            arr[1].clone(),
            arr[2].clone(),
            arr[3].clone(),
            arr[4].clone(),
            arr[5].clone(),
        ]
    }
    fn reduce_cubics_7(&self, arr: &[Cubic; 8]) -> [Cubic; 7] {
        [
            arr[0].clone(),
            arr[1].clone(),
            arr[2].clone(),
            arr[3].clone(),
            arr[4].clone(),
            arr[5].clone(),
            arr[6].clone(),
        ]
    }

    // 3 ^     12 11 10 09 08 07 06 05 04 03 02 01 00
    // permId: P0 P1 P2 P3 P4 P5 R0 R1 R2 R3 R4 R5 R6
    pub fn get_pos_id(&self) -> PosId {
        let id_func = |x: &Cubic| -> u32 { x.get_pos().into() };

        let index_arr = permutation_to_index_arr(
            &self.get_cubics().to_vec(),
            &self.get_default_cubics().to_vec(),
            id_func,
        );
        let id = index_arr_to_id::<u32>(&index_arr);

        // let arr = id_to_index_arr::<u32>(&id, &7);
        // let id_again = index_arr_to_id(&arr);
        // let perm_again =
        //     index_arr_to_permutation(&self.get_default_cubics().to_vec(), &index_arr, id_func);

        // println!("ID {}  Again  {}", id, id_again);
        // println!("Mod 3 {}", sum % 3);
        PosId::new(id)
    }

    pub fn get_rot_id(&self) -> RotId {
        let mut sum = 0;
        for i in self.reduce_cubics_6(&self.cubics) {
            let rot: u32 = i.get_rot().into();
            sum *= 3;
            sum += rot;
        }
        RotId::new(sum)
    }

    pub fn get_perm_id(&self) -> PermId {
        PermId::new(&self.get_pos_id(), &self.get_rot_id())
    }

    pub fn get_branches(&self) -> [PermId; 3] {
        let (mut cube1, mut cube2, mut cube3) = (self.clone(), self.clone(), self.clone());

        cube1.do_move(move_right);
        cube2.do_move(move_up);
        cube3.do_move(move_front);

        return [
            cube1.get_perm_id(),
            cube2.get_perm_id(),
            cube3.get_perm_id(),
        ];
    }

    pub fn get_cubics(&self) -> [Cubic; 7] {
        self.reduce_cubics_7(&self.cubics)
    }

    pub fn get_default_cubics(&self) -> [Cubic; 7] {
        self.reduce_cubics_7(&self.default_cubics)
    }

    pub fn reset_cubics(&mut self) {
        self.cubics = [
            Cubic::new(CornerPos::FUR, [U, F, R]),
            Cubic::new(CornerPos::FUL, [U, L, F]),
            Cubic::new(CornerPos::FDR, [D, R, F]),
            Cubic::new(CornerPos::FDL, [D, F, L]),
            Cubic::new(CornerPos::BUR, [U, R, B]),
            Cubic::new(CornerPos::BUL, [U, B, L]),
            Cubic::new(CornerPos::BDR, [D, B, R]),
            Cubic::new(CornerPos::BDL, [D, L, B]),
        ];
    }

    pub fn get_sides(&self, pos: CornerPos) -> [CubeRealSide; 3] {
        for i in &self.cubics {
            if i.get_pos() == pos {
                return i.get_sides_rotated();
            }
        }
        panic!(
            "Tried to get sides of the non-existent cubic with position {:?}",
            pos
        );
    }

    pub fn to_facelets(&self) -> [CubeRealSide; 6 * 2 * 2] {
        let (fur, ful, fdr, fdl) = (
            self.get_sides(FUR),
            self.get_sides(FUL),
            self.get_sides(FDR),
            self.get_sides(FDL),
        );
        let (bur, bul, bdr, bdl) = (
            self.get_sides(BUR),
            self.get_sides(BUL),
            self.get_sides(BDR),
            self.get_sides(BDL),
        );

        return [
            bul[0], bur[0], ful[0], fur[0], /* Up Face */
            bul[2], ful[1], bdl[1], fdl[2], /* Left Face */
            ful[2], fur[1], fdl[1], fdr[2], /* Front Face */
            fur[2], bur[1], fdr[1], bdr[2], /* Right Face */
            bur[2], bul[1], bdr[1], bdl[2], /* Back Face */
            fdl[0], fdr[0], bdl[0], bdr[0], /* Down Face */
        ];
    }

    pub fn do_move(&mut self, perm_func: impl Fn(CornerPos) -> CubicPerm) {
        for i in &mut self.cubics {
            let perm = perm_func(i.get_pos());
            i.set_pos(perm.pos);
            i.rotate(perm.rot);
        }
    }
}

// Implement Debug for Cubic
impl fmt::Debug for Cubic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write custom debug representation using `write!` macro or methods of `Formatter`
        write!(
            f,
            "Cubic {{ orig_pos: {}, curr_pos: {} }}",
            self.orig_pos, self.curr_pos
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_index_to_
}
