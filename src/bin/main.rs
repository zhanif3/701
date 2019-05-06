#[macro_use]
extern crate bincode;
extern crate common;

use serde::{Serialize, Deserialize};
use std::io::BufWriter;


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

    let now = SystemTime::now();
    let T = file_contents.as_str();
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("READ THE FILE: {:?}", elapsed.as_secs());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    };

    //let bwt = suffix_array("banana$");
    //let T = "ABAABA$";
    //let T = "PABAABAZ$";
    let now = SystemTime::now();
    let sa = common::suffix_array::suffix_array(&T);
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("BUILT THE SUFFIX ARRAY: {:?}", elapsed.as_secs());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    };

    //println!("{:?}", sa);

    let now = SystemTime::now();
    let lf = common::suffix_array::construct_lf(&T, &sa);
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("BUILT THE LF STRUCTURE: {:?}", elapsed.as_secs());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    };
    //println!("{:?}", lf);
    let now = SystemTime::now();
    let original_string = common::suffix_array::reverse_bwt(&lf.Index);
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("BUILT THE ORIGINAL STRING: {:?}", elapsed.as_secs());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    };
    //println!("reversed string {:?}", original_string);
    let now = SystemTime::now();
    let zipped: Vec<(&common::types::Nucleotide, &common::types::Nucleotide)> = lf.F.par_iter().zip(lf.L.par_iter()).collect();
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("BUILT ZIPPED: {:?}", elapsed.as_secs());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    };

    // If we serde the arguments here, we can do searches.
    let now = SystemTime::now();
    let mut lf_buffer = BufWriter::new(File::create("lf.bincode").unwrap());
    let g = bincode::serialize_into(&mut lf_buffer, &lf);
    lf_buffer.flush();
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("WROTE LF: {:?}", elapsed.as_secs());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    };

    let coordinates = common::search::search(args.last().expect("Could not unwrap last arg. Were there args?"), &zipped, &lf.FCounts);
    println!("ABA SHOULD BE SOME(2, 4): {:?}", coordinates);

    let positions = match coordinates {
        Some(x) => common::suffix_array::get_suffixes_by_pos(sa, x),
        None => (0, vec![])
    };
    println!("You searched for: {:?}.", args.last().expect("Could not unwrap last arg. Were there args?"));
    println!("There are {:?} matches.", positions.0);

    let suffix_strings: Vec<&str> = (positions.1).iter().map(|x| {
        str::from_utf8(x.1).expect("Do we have non-UTF8 encoded strings?")
    }).collect();
    //println!("The suffixes are: {:?}", suffix_strings);
    println!("SIZEOF LF: FC; {:?}, F; {:?}, L; {:?}, Index; {:?}", lf.FCounts.len(), lf.F.len()*(1+8), lf.L.len()*(1+8), lf.Index.len()*(9+9));


    Ok(())
}
