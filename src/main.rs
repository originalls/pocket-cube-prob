// #![allow(warnings)]

use std::env;

use calc::MtableIterator;
use csv::Writer;

mod calc;
mod enums;
mod geom;
mod group;
mod gui;
mod moves;
mod perm;
mod rubiks_cube;
mod types;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 && args[1] == "gen" {
        let mtable = calc::generate_mini_table();
        calc::write_to_file(&mtable, "out/output.tbl");
    } else if args.len() >= 2 && (args[1] == "calc" || args[1] == "calcdis") {
        let mtable = calc::load_mini_table("out/output.tbl");
        let mut iterator = MtableIterator::new();
        println!("{:?}", mtable[13]);
        println!("Loading mtable...");
        iterator.load_mtable(mtable);
        iterator.set_zero();
        println!("Loaded mtable");

        let mut wtr = Writer::from_path("out/output.csv").unwrap();
        wtr.write_record(&["loop", "value", "disperse"]).unwrap();

        for i in 0..100 {
            let disperse = if args[1] == "calcdis" {
                true
            } else {
                if i == 0 {
                    true
                } else {
                    false
                }
            };
            iterator.iterate(disperse);
            let prob = iterator.get_prob(0);
            println!("Iteration {} completed", i);
            println!("New probability = {}", prob);
            let record_loop = i.to_string();
            let record_prob = prob.to_string();
            let record_disperse = if disperse {
                "yes".to_string()
            } else {
                "no".to_string()
            };
            wtr.write_record(&[record_loop, record_prob, record_disperse])
                .unwrap();
            wtr.flush().unwrap();
        }
    } else {
        gui::mainloop();
    }
}
