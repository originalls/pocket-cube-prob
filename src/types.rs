pub const PERMID_POSCOUNT: u32 = 5040; // 7!
pub const PERMID_ROTCOUNT: u32 = 729; // 3^6
pub const PERMID_COUNT: usize = (PERMID_POSCOUNT * PERMID_ROTCOUNT) as usize;

#[derive(Debug, Copy, Clone)]
pub struct RotId {
    rid: u32,
}
#[derive(Debug, Copy, Clone)]
pub struct PosId {
    pid: u32,
}
#[derive(Debug, Copy, Clone)]
pub struct PermId {
    id: u32,
    rid: RotId,
    pid: PosId,
}

// ================ IDs ================

impl RotId {
    pub fn new(rid: u32) -> Self {
        // todo: add min/max checks
        Self { rid }
    }
    pub fn get_id(&self) -> u32 {
        self.rid
    }
}

impl PosId {
    pub fn new(pid: u32) -> Self {
        // todo: add min/max checks
        Self { pid }
    }
    pub fn get_id(&self) -> u32 {
        self.pid
    }
}

impl PermId {
    pub fn new(pid: &PosId, rid: &RotId) -> Self {
        let id = PERMID_POSCOUNT * rid.get_id() + pid.get_id();
        Self {
            id,
            pid: *pid,
            rid: *rid,
        }
    }

    pub fn get_rot_id(&self) -> &RotId {
        &self.rid
    }
    pub fn get_pos_id(&self) -> &PosId {
        &self.pid
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }
}

impl From<u32> for PermId {
    fn from(id: u32) -> Self {
        let p_id = id % PERMID_POSCOUNT;
        let r_id = id / PERMID_POSCOUNT;

        Self {
            id,
            pid: PosId::new(p_id),
            rid: RotId::new(r_id),
        }
    }
}
