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
use common::types::Nucleotide;

pub type Nuc = (char, usize);

// I want this to return the bwt string, the bwm (matrix), and the rank array

// OK. Split to library, create command line clients, and then benchmark.
// Then, get a large DNA strand to match against.

pub fn reverse_bwt(index: &HashMap<Nuc, Nuc>) -> Vec<char> {
    // Starting with ('$', 1) (we might want to replace this with a type),
    let mut nodes: Vec<char> = Vec::with_capacity(index.len());

    let mut current_node = ('$', 1 as usize);
    //println!("{:?}", index);
    for i in 0 .. index.len() {
        let v = index.get(&current_node);
        let value = match v {
            Some(x) => x,
            None => {
                println!("SOMETHING WENT WRONG, WE COULD NOT FIND: {:?}", current_node);
                &current_node
            }
        };
        //let next_query = *index.get(value).unwrap();
        let next_query = value;
        //println!("Querying {:?}, got {:?}; answer is {:?}", current_node, value, current_node);
        nodes.push(current_node.0);
        current_node = *next_query

    }
    nodes.reverse();
    return nodes;
}

pub fn read_ros_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Something went wrong when reading the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    let mut stripped = contents.replace('\n', "");
    let final_char = stripped.chars().last().expect("Something went wrong when getting the last character. Does the file have contents?");
    stripped
}

// Convert L and F into (char, usize), where usize is the count per character for both.

pub fn generate_l(T: &str) -> Vec<Nuc> {
    // L is just the vector.
    let mut l_counter = HashMap::new();
    let L: Vec<Nucleotide> = T.as_bytes().iter().map(|c| {
        let count = l_counter.entry(c).or_insert(0);
        *count += 1;
        (*c as char, *count)
    }).collect();
    L
}
pub fn generate_f(T: &str) -> Vec<Nuc> {
    // F is the sorted and counted T.
    let mut f_counter = HashMap::new();
    let mut F: Vec<Nucleotide> = T.as_bytes().iter().map(|c| {
        let count = f_counter.entry(c).or_insert(0);
        *count += 1;
        (*c as char, *count)
    }).collect();
    F.sort();
    F
}
/*
pub fn generate_index() -> Vec<(Nucleotide, Nucleotide)> {
    let Index: HashMap<Nucleotide, Nucleotide> = (0 .. t.len()).map(|i| {
        (L[i], F[i])
    }).collect();
}
*/
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_contents = read_ros_file(args.get(1).expect("Could not get the cmdline arguments"));
    let T = file_contents.as_str();

    let L = generate_l(&T);
    let F = generate_f(&T);

    let Index: HashMap<Nucleotide, Nucleotide> = (0..T.len()).map(|i| {
        (F[i], L[i])
    }).collect();

    let original_string = reverse_bwt(&Index);

    let output: String = original_string.iter().collect();

    println!("{}", output);


    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ROSALIND_TEST_TRUE() {
        let T = "TTCCTAACG$A";
        let L = generate_l(&T);
        let F = generate_f(&T);
        let Index: HashMap<Nucleotide, Nucleotide> = (0..T.len()).map(|i| {
            (F[i], L[i])
        }).collect();

        let ground_truth = "TACATCACGT$".to_string();
        let original_string = reverse_bwt(&Index);
        let answer: String = original_string.iter().collect();
        assert_eq!(ground_truth, answer);
    }
}