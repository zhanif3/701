#[macro_use]
//extern crate bincode;
extern crate common;

//use serde::{Serialize, Deserialize};
//use std::io::BufWriter;


use std::io;
use std::io::Read;
use std::fs::File;
use std::io::prelude::*;
use std::iter::repeat;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str;
use std::env;
use std::mem;

use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use std::time::{Duration, SystemTime};

// I want this to return the bwt string, the bwm (matrix), and the rank array

// OK. Split to library, create command line clients, and then benchmark.
// Then, get a large DNA strand to match against.

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args.last());
    let file_contents = common::suffix_array::read_file(args.get(1).expect("Could not get the cmdline arguments"));

    let T = file_contents.as_str();


    let now = SystemTime::now();
    let sa = common::suffix_array::suffix_array(&T);
    let suffix_array_time = match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            elapsed.as_millis()
        }
        Err(e) => {
            // an error occurred!
            0
        }
    };

    let now = SystemTime::now();
    let lf = common::suffix_array::construct_lf(&T, &sa);
    let lf_time = match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            elapsed.as_millis()
        }
        Err(e) => {
            // an error occurred!
            0
        }
    };

    let original_string = common::suffix_array::reverse_bwt(&lf.Index);

    let zipped: Vec<(&common::types::Nucleotide, &common::types::Nucleotide)> = lf.F.par_iter().zip(lf.L.par_iter()).collect();

    let now = SystemTime::now();
    let coordinates = common::search::search(args.last().expect("Could not unwrap last arg. Were there args?"), &zipped, &lf.FCounts);
    let search_time = match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            elapsed.as_millis()
        }
        Err(e) => {
            // an error occurred!
            0
        }
    };
    let positions = match coordinates {
        Some(x) => common::suffix_array::get_suffixes_by_pos(sa, x),
        None => (0, vec![])
    };


    //println!("You searched for: {:?}.", args.last().expect("Could not unwrap last arg. Were there args?"));
    //println!("There are {:?} matches.", positions.0);

    /*
    let suffix_strings: Vec<&str> = (positions.1).iter().map(|x| {
        str::from_utf8(x.1).expect("Do we have non-UTF8 encoded strings?")
    }).collect();

    println!("SIZEOF LF: FC; {:?}, F; {:?}, L; {:?}, Index; {:?}", lf.FCounts.len(), lf.F.len()*(1+8), lf.L.len()*(1+8), lf.Index.len()*(9+9));
    */
    println!("{}, {}, {}, {}, {:?}, {:?}", search_time, suffix_array_time, lf_time, positions.0, args.last().expect("Could not unwrap last arg. Were there args?"), args.get(1).expect("Could not get the cmdline arguments"));
    Ok(())
}
