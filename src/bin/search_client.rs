#[macro_use]
extern crate common;

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

    let mut lf_buffer = fs::read("lf.bincode")?;

    let lf: LF = bincode::deserialize(&lf_buffer).unwrap();
    let zipped: Vec<(&common::types::Nucleotide, &common::types::Nucleotide)> = lf.F.par_iter().zip(lf.L.par_iter()).collect();

    let coordinates = common::search::search(args.last().expect("Could not unwrap last arg. Were there args?"), &zipped, &lf.FCounts);

    let positions = match coordinates {
        Some(x) => x.1 - x.0 +1,
        None => 0
    };
    println!("You searched for: {:?}.", args.last().expect("Could not unwrap last arg. Were there args?"));
    println!("There are {:?} matches.", positions);

    Ok(())
}