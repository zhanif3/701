#[macro_use]

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

use std::borrow::Borrow;
use std::time::{Duration, SystemTime};

use bio::pattern_matching::bndm;
use rayon::prelude::*;

fn parallel_search(text: &[u8], pattern: &[u8]) -> Vec<(usize, usize)> {
    text.par_windows(pattern.len()).enumerate()
        .map(|x| {
            let e = match x {
                (pos, value) if value == pattern => Some((pos, pos+pattern.len())),
                _ => None,
            };
            e
        })
        .filter(|x| {
            x.is_some()
        })
        .map(|x| {
            x.unwrap()
        })
        .collect()
}



fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args.last());
    let file_contents = common::suffix_array::read_file(args.get(1).expect("Could not get the cmdline arguments"));

    let T = file_contents.as_str();
    //let pattern = args.get(1).expect("Could not get the cmdline arguments");
    //let text = b"dhjalkjwqnnnannanaflkjdklfj";
    let pattern = b"TTAGAGAAA";

    let now = SystemTime::now();
    let windows = parallel_search(T.as_bytes(), pattern);
    let parallel_search_time = match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            elapsed.as_millis()
        }
        Err(e) => {
            // an error occurred!
            0
        }
    };

    //let pattern = b"AAAA";
    //let text = b"ACGGCTAGAAAAGGCTAGAAAA";

    let now = SystemTime::now();
    let bndm = bndm::BNDM::new(pattern);
    let occ: Vec<usize> = bndm.find_all(T.as_bytes()).collect();
    let bndm_time = match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            elapsed.as_millis()
        }
        Err(e) => {
            // an error occurred!
            0
        }
    };

    println!("{}, {}, {}, {}, {:?}, {:?}", bndm_time, parallel_search_time, windows.len(), occ.len(), args.last().expect("Could not unwrap last arg. Were there args?"), args.get(1).expect("Could not get the cmdline arguments"));
    Ok(())
}
