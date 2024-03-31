use crate::{group::PocketCube, types::PERMID_COUNT};
use std::cmp::min;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;

type MiniTableRow = [u32; 4];
type MiniTable = Vec<MiniTableRow>;

pub fn generate_mini_table() -> MiniTable {
    let mut table: MiniTable = vec![[0, 0, 0, 1]; PERMID_COUNT];
    println!("Initialized MiniTable vector");
    let mut pc = PocketCube::new();

    const BATCH_SIZE: usize = 100000;

    for x in 0..((table.len() + BATCH_SIZE - 1) / BATCH_SIZE) {
        let start = x * BATCH_SIZE;
        let end = min(PERMID_COUNT, (x + 1) * BATCH_SIZE);
        for i in start..end {
            pc.apply_id((i as u32).into());
            let branches = pc.get_branches();
            for j in 0..3 {
                table[i][j] = branches[j].get_id();
            }
            table[i][3] = 0;
        }
        println!(
            "Batch {} completed. ({}%)",
            x,
            ((end as f32) / (PERMID_COUNT as f32) * 10000.0).round() / 100.0
        );
    }
    table
}

// impl Default for MiniTableRow {
//     fn default() -> Self {
//         [0, 0, 0, 0]
//     }
// }

pub fn write_to_file(data: &MiniTable, fname: &str) {
    let mut f = File::create(fname).expect("Unable to create file");
    for i in data {
        f.write_all(&convert(i)).expect("Unable to write data");
    }
}

pub fn load_mini_table(fname: &str) -> MiniTable {
    let mut f = File::open(fname).expect("No table file found");
    // let mut buffer = vec![[0, 0, 0, 0] as MiniTableRow; PERMID_COUNT];
    let mut table: MiniTable = Vec::new();
    let mut buffer = vec![0 as u8; PERMID_COUNT * 4 * 4];
    f.read(&mut buffer)
        .expect("Buffer overflow while reading table file");
    for _ in 0..PERMID_COUNT {
        let num1 = pop_u32(&mut buffer);
        let num2 = pop_u32(&mut buffer);
        let num3 = pop_u32(&mut buffer);
        let num4 = pop_u32(&mut buffer);
        table.push([num4, num3, num2, num1]);
    }

    table.reverse();
    table
}

fn pop_u32(arr: &mut Vec<u8>) -> u32 {
    let big = (arr.pop().unwrap() as u32)
        + ((arr.pop().unwrap() as u32) << 8)
        + ((arr.pop().unwrap() as u32) << 16)
        + ((arr.pop().unwrap() as u32) << 24);
    u32::to_be(big)
}

// From: https://stackoverflow.com/a/72631195
fn convert(data: &[u32; 4]) -> [u8; 16] {
    let mut res = [0; 16];
    for i in 0..4 {
        res[4 * i..][..4].copy_from_slice(&data[i].to_le_bytes());
    }
    res
}

pub struct MtableIterator {
    mtable: MiniTable,
    probs: Vec<f64>,
    epoch: usize,
}

impl MtableIterator {
    pub fn new() -> Self {
        Self {
            mtable: Vec::new(),
            probs: vec![0.0; PERMID_COUNT],
            epoch: 0,
        }
    }

    pub fn load_mtable(&mut self, mtable: MiniTable) {
        self.mtable = mtable;
    }

    pub fn set_zero(&mut self) {
        self.probs[0] = 1.0;
    }

    // if disperse is true,
    pub fn iterate(&mut self, disperse: bool) {
        let mut new_probs: Vec<f64> = vec![0.0; PERMID_COUNT];
        if !disperse {
            new_probs[0] = self.probs[0];
        }
        for i in (if disperse { 0 } else { 1 })..PERMID_COUNT {
            let prob = &self.probs[i];
            let row = &self.mtable[i];
            let prob_each = prob / 3.0;
            for j in 0..3 {
                new_probs[row[j] as usize] += prob_each;
            }
        }
        self.probs = new_probs;
        self.epoch += 1;
    }

    pub fn get_prob(&self, id: u32) -> f64 {
        if id >= (PERMID_COUNT as u32) {
            panic!("Invalid id to get_prob")
        }
        return self.probs[id as usize];
    }
}
