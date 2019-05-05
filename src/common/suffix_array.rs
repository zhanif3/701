//extern crate bincode;

//use serde::{Serialize, Deserialize};

use crate::types::Nucleotide;
use crate::types::Suffix;
use std::collections::HashMap;
use rayon::iter::IntoParallelRefIterator;

use std::io::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

use std::time::{Duration, SystemTime};
use rayon::prelude::*;

//#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[derive(Debug)]
pub struct LF {
    pub L: Vec<Nucleotide>,
    pub F: Vec<Nucleotide>, // Character,
    pub Index: HashMap<Nucleotide, Nucleotide>,
    pub FCounts: HashMap<u8, usize>,
    //FMIndex: Vec<char, char>,

}


pub fn read_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Something went wrong when reading the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    let mut stripped = contents.replace('\n', "");
    let final_char = stripped.chars().last().expect("Something went wrong when getting the last character. Does the file have contents?");
    match final_char {
        '$' => (),
        _ => stripped.push('$'),
    }
    stripped
}

// Arguably, we could return JUST in indicies.
pub fn suffix_array(t: &str) -> Vec<Suffix> {
    let now = SystemTime::now();
    let mut s: Vec<(Suffix)> = (0 .. t.len()).map(|x: usize| {
        (x, t[x..].as_bytes())
    }).collect();
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            //println!("BUILT VECTOR OF SUFFIXES: {:?}", elapsed.as_secs());
        }
        Err(e) => {
            // an error occurred!
            //println!("Error: {:?}", e);
        }
    };
    let now = SystemTime::now();
    s.par_sort_unstable_by(|a, b| a.1.cmp(b.1));

//    s.sort_by(|a, b| a.1.cmp(b.1));
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            //println!("BUILT SORTED THE VECTOR: {:?}", elapsed.as_secs());
        }
        Err(e) => {
            // an error occurred!
            //println!("Error: {:?}", e);
        }
    };
    //let r = vec![(5, "s".to_string())];
    return s
}

pub fn reverse_bwt(index: &HashMap<Nucleotide, Nucleotide>) -> Vec<char> {
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


pub fn construct_lf(t: &str, sa: &Vec<Suffix>) -> LF {
    let mut f_counter = HashMap::new();
    /*
    f_counter.insert(65u8, 0 as usize);
    f_counter.insert(67u8, 0 as usize);
    f_counter.insert(71u8, 0 as usize);
    f_counter.insert(84u8, 0 as usize);
    */
    let mut l_counter = HashMap::new();
    /*
    l_counter.insert(65u8, 0 as usize);
    l_counter.insert(67u8, 0 as usize);
    l_counter.insert(71u8, 0 as usize);
    l_counter.insert(84u8, 0 as usize);
    */
    let mut first_seen = HashMap::new();

    let now = SystemTime::now();
    let F: Vec<Nucleotide> = sa.iter().enumerate().map(|suffix| {
        let c = t.as_bytes()[(suffix.1).0];
        let count = f_counter.entry(c).or_insert(0);
        *count += 1;
        first_seen.entry(c).or_insert(suffix.0);
        (c as char, *count)

    }).collect();
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            //println!("MADE F: {:?}", elapsed.as_secs());
        }
        Err(e) => {
            // an error occurred!
            //println!("Error: {:?}", e);
        }
    };

    let now = SystemTime::now();
    let L: Vec<Nucleotide> = sa.iter().map(|suffix| {
        let e = match suffix.0 {
            0 => 36 as u8,
            v if v > 0 => t.as_bytes()[v-1],
            _ => 90 as u8,
        };
        let count = l_counter.entry(e).or_insert(0);
        //println!("Printing Character and Count: {:?}", (&suffix.1[0], *count));
        *count += 1;
        //println!("Printing Pos and Count: {:?}", (e as char, *count));
        (e as char, *count)
    }).collect();
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            //println!("MADE L: {:?}", elapsed.as_secs());
        }
        Err(e) => {
            // an error occurred!
            //println!("Error: {:?}", e);
        }
    };

    let now = SystemTime::now();
    let Index: HashMap<Nucleotide, Nucleotide> = (0 .. t.len()).map(|i| {
        (L[i], F[i])
    }).collect();
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            //println!("MADE INDEX: {:?}", elapsed.as_secs());
        }
        Err(e) => {
            // an error occurred!
            //println!("Error: {:?}", e);
        }
    };
    //let positions =
    return LF {L, F, Index, FCounts: first_seen}
}


pub fn get_suffixes_by_pos(suffix_arr: Vec<Suffix>, positions: (usize, usize)) -> (usize, Vec<Suffix>) {
    let pos = match positions {
        (0, 0) => (0, 0),
        (0, x) if x != 0 => (0, x-1),
        (x, y) => (x-1, y-1)
    };
    let suffixes = suffix_arr[pos.0 ..= pos.1].to_vec();
    let size = suffixes.len();
    return (size, suffixes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bwt_reverse_correct() {
        let T = "ABAABA$";
        let sa = suffix_array(&T);
        let lf = construct_lf(&T, &sa);
        let original_string = reverse_bwt(&lf.Index);

        assert_eq!(original_string.as_slice().into_iter().collect::<String>().as_str(), T);
    }

    #[test]
    fn bwt_longer_reverse_correct() {
        let T = "ABAABAABAABAABAABAABAABAABAABAABAABAABAABAABAABAABAABAABAABAABAABAABAABAABAABAABAABA$";
        let sa = suffix_array(&T);
        let lf = construct_lf(&T, &sa);
        let original_string = reverse_bwt(&lf.Index);

        assert_eq!(original_string.as_slice().into_iter().collect::<String>().as_str(), T);
    }

    #[test]
    fn get_suffixes_by_pos_test() {
        let positions = (0, 0);
        let pos = match positions {
            (0, 0) => (0, 0),
            (0, x) if x != 0 => (0, x-1),
            x => x,
        };
        assert_eq!(positions, pos);

        let positions = (0, 9);
        let pos = match positions {
            (0, 0) => (0, 0),
            (0, x) if x != 0 => (0, x-1),
            x => x,
        };
        assert_eq!(pos, (0, 8));

        let positions = (9, 9);
        let pos = match positions {
            (0, 0) => (0, 0),
            (0, x) if x != 0 => (0, x-1),
            x => x,
        };
        assert_eq!(positions, pos);
    }
}