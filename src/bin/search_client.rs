#[macro_use]
//extern crate bincode;
extern crate common;

//use bincode::serde::{serialize, deserialize};
//use serde::{Serialize, Deserialize};

use std::io;
use std::io::Read;
use std::fs::File;
use std::io::prelude::*;
use std::iter::repeat;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str;
use std::fs;
use std::env;

use common::suffix_array::LF;
use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    /*
    let mut lf_buffer = fs::read("lf.bincode")?;
    //let mut zipped_buffer = fs::read("zipped.bincode")?;
    let mut sa_buffer = fs::read("sa.bincode")?;

    let lf: LF = bincode::deserialize(&lf_buffer).unwrap();
    let decoded_suffix: Vec<(usize, &[u8])> = bincode::deserialize(&sa_buffer).unwrap();
    let zipped: Vec<(&common::types::Nucleotide, &common::types::Nucleotide)> = lf.F.par_iter().zip(lf.L.par_iter()).collect();
    /*
    let coordinates = common::search::search("ABA", &zipped, &lf.FCounts);
    println!("ABA SHOULD BE SOME(2, 4): {:?}", coordinates);
    let coordinates = common::search::search("A", &zipped, &lf.FCounts);
    println!("A SHOULD BE SOME(0, 4): {:?}", coordinates);
    let coordinates = common::search::search("BBA", &zipped, &lf.FCounts);
    println!("BBA SHOULD BE NONE: {:?}", coordinates);
    let coordinates = common::search::search("J", &zipped, &lf.FCounts);
    println!("J SHOULD BE NONE: {:?}", coordinates);
    let coordinates = common::search::search("BAZ", &zipped, &lf.FCounts);
    println!("BAZ SHOULD BE NONE: {:?}", coordinates);
    let coordinates = common::search::search("BZAA", &zipped, &lf.FCounts);
    println!("BZAA SHOULD BE NONE: {:?}", coordinates);
    let coordinates = common::search::search("ABAA", &zipped, &lf.FCounts);
    println!("ABAA SHOULD BE SOME(5,5): {:?}", coordinates);
    */
    let coordinates = common::search::search(args.last().expect("Could not unwrap last arg. Were there args?"), &zipped, &lf.FCounts);
    //println!("ALL SUFFIXES {:?}", &decoded_suffix);
    let positions = match coordinates {
        Some(x) => common::suffix_array::get_suffixes_by_pos(decoded_suffix, x),
        None => (0, vec![])
    };
    println!("You searched for: {:?}.", args.last().expect("Could not unwrap last arg. Were there args?"));
    println!("There are {:?} matches.", positions.0);

    let suffix_strings: Vec<&str> = (positions.1).iter().map(|x| {
        str::from_utf8(x.1).expect("Do we have non-UTF8 encoded strings?")
    }).collect();
    println!("The suffixes are: {:?}", suffix_strings);
    */
    Ok(())
}