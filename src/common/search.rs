#[macro_use]

use rayon::prelude::*;

use std::collections::HashMap;
use crate::types::Nucleotide;
use crate::types::Suffix;
use crate::suffix_array;

pub fn select_rows<'a>(zipped: &'a Vec<(&'a Nucleotide, &'a Nucleotide)>, search_char: char,
                       filter_char: Option<&char>) -> (Vec<&'a(&'a Nucleotide, &'a Nucleotide)>, usize, usize, char) {
    // if Filter Char is NONE, just return the rows without the filter

    let first_rows: Vec<&(&Nucleotide, &Nucleotide)> = zipped.par_iter().filter(|item| {
        let f: &Nucleotide = &item.0;
        let l: &Nucleotide = &item.1;
        let b = match filter_char {
            Some(v) => {
                //println!("Have a filter character {:?}, and it is in the row: {:?}", filter_char, (f.0 == search_char as char) & (l.0 == *v));
                (f.0 == search_char as char) & (l.0 == *v)
            },
            None => {
                //println!("Have no filter character, CAN EXIT EARLY");
                //true
                f.0 == search_char as char //IF WE ARE HERE, WE'RE DONE
            },//
        };
        b
    }).collect();
    // Now, emit the start, end
    match first_rows.is_empty() {
        true => return (first_rows, 0, 0, '$'),
        false => {
            let start = (first_rows.first().unwrap().1).1;
            let end = (first_rows.last().unwrap().1).1;
            let offset_char = ((first_rows.first().unwrap().1).0);
            //println!("START AND END {:?}, {:?}", start, end);
            return (first_rows, start, end, offset_char)
        },
    }
}

pub fn search(query: &str, zipped: &Vec<(&Nucleotide, &Nucleotide)>, Counts: &HashMap<u8, usize>) -> Option<(usize, usize)> {
    let mut s = query.chars().rev().peekable();
    let mut start: usize;
    let mut end: usize;
    //let mut return_rows;
    let mut working_rows: Vec<(&(char, usize), &(char, usize))> = Vec::new();
    // Start, End, Offset maintain our state during the iteration.
    // For char in search_char, until s is exhausted
    for pos in 0 .. query.len() {
        let search_char = s.next().unwrap();
        let filter_char = s.peek();

        // If either search or filter are NOT in Counts, fail out, return NONE

        match Counts.get(&(search_char as u8)) {
            Some(x) => (),
            None => return None,
        }
        match filter_char {
            Some(x) => {
                match Counts.get(&(*x as u8)) {
                    Some(c) => (),
                    None => return None,
                }
            },
            None => {
                ()
            },
        };
        //println!("Getting rows with SEARCH: {:?}, and FILTER: {:?}", search_char, filter_char);
        let (first_rows, start, end, offset_char) = match working_rows.is_empty() {
            true => select_rows(zipped, search_char, filter_char),
            false => select_rows(&working_rows, search_char, filter_char),
        };

        //println!("{:?}, {:?}, {:?}", pos, first_rows, first_rows.len());
        // Get the next set of rows to iterate over
        match first_rows.is_empty() {
            // If we have NO FOUND ROWS, we can exit immediately
            true => return None,
            // If we have ANY OTHER NUMBER, we keep processing
            _ => ()
        };
        let offset = Counts.get(&(offset_char as u8)).unwrap();
        //println!("Filtered to rows with offset: {:?}, start {:?}, and end {:?}", offset, start-1, end-1);
        match pos {
            x if x+1 == query.len() => {
                // Return offset+pos of start and end.
                let pos = get_pos_from_vec(&first_rows, Counts)?;
                //println!("WOULD EXIT WITH {:?}, a total of {:?} matches", zipped[pos.0 ..= pos.1].to_vec(), zipped[pos.0 ..= pos.1].to_vec().len());
                return Some((pos.0, pos.1))
            },
            _ => (),
        }
        working_rows = zipped[start+offset-1 ..= end+offset-1].to_vec();
        //println!("ROWS ARE: {:?}", working_rows);
    }
    return Some((9,9))
}

fn get_pos_from_vec(rows: &Vec<&(&(char, usize), &(char, usize))>, Counts: &HashMap<u8, usize>) -> Option<(usize, usize)> {
    let start_pos = match rows.first() {
        Some(x) => {
            (x.0).1
        },
        None => return None
    };
    let end_pos = match rows.last() {
        Some(x) => {
            (x.0).1
        },
        None => return None
    };
    let offset_char = match rows.first() {
        Some(x) => {
            let c = (x.0).0;
            Counts.get(&(c as u8)).unwrap()
        },
        None => return None,
    };
    //println!("{:?}, {:?}, {:?}", start_pos+offset_char, end_pos+offset_char, offset_char);
    return Some((start_pos+offset_char, end_pos+offset_char))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::suffix_array;

    #[test]
    fn test_search_capabilities() {
        let T = "ABAABA$";
        let sa = suffix_array::suffix_array(&T);
        let lf = suffix_array::construct_lf(&T, &sa);

        let zipped = lf.F.iter().zip(lf.L.iter()).collect();
        let coordinates = search("ABA", &zipped, &lf.FCounts);

        assert_eq!(Some((3,5)), coordinates);
    }

    #[test]
    fn test_search_capabilities_nonexistant() {
        let T = "ABAABA$";
        let sa = suffix_array::suffix_array(&T);
        let lf = suffix_array::construct_lf(&T, &sa);

        let zipped = lf.F.iter().zip(lf.L.iter()).collect();
        let coordinates = search("P", &zipped, &lf.FCounts);

        assert_eq!(None, coordinates);
    }

    #[test]
    fn test_search_capabilities_start() {
        let T = "ABAABA$";
        let sa = suffix_array::suffix_array(&T);
        let lf = suffix_array::construct_lf(&T, &sa);

        let zipped = lf.F.iter().zip(lf.L.iter()).collect();
        let coordinates = search("AA", &zipped, &lf.FCounts);

        assert_eq!(Some((2,3)), coordinates);
    }
    #[test]
    fn test_search_capabilities_long_end() {
        let T = "ABAABA$";
        let sa = suffix_array::suffix_array(&T);
        let lf = suffix_array::construct_lf(&T, &sa);

        let zipped = lf.F.iter().zip(lf.L.iter()).collect();
        let coordinates = search("AABA", &zipped, &lf.FCounts);

        assert_eq!(Some((2,3)), coordinates);
    }
    #[test]
    fn test_search_capabilities_end() {
        let T = "ABAABA$";
        let sa = suffix_array::suffix_array(&T);
        let lf = suffix_array::construct_lf(&T, &sa);

        let zipped = lf.F.iter().zip(lf.L.iter()).collect();
        let coordinates = search("BA", &zipped, &lf.FCounts);

        assert_eq!(Some((5,7)), coordinates);
    }
    fn test_search_capabilities_partial() {
        let T = "ABAABA$";
        let sa = suffix_array::suffix_array(&T);
        let lf = suffix_array::construct_lf(&T, &sa);

        let zipped = lf.F.iter().zip(lf.L.iter()).collect();
        let coordinates = search("AAZ", &zipped, &lf.FCounts);

        assert_eq!(None, coordinates);
    }
}