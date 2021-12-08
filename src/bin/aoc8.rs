use std::{fs, collections::{BTreeSet, BTreeMap}};

fn main() {
    // Part 1
    let input_file = fs::read_to_string("inputs/aoc8.txt").unwrap();
    let mut easy_counts = 0; // number of 1, 4, 7 or 8
    for line in input_file.lines() {
        let parts: Vec<&str> = line.split("|").collect();
        let outputs: Vec<&str> = parts[ 1 ].split_whitespace().collect();
        for part in outputs {
            easy_counts += match part.len() {
                2|4|3|7 => 1,
                _ => 0
            }
        }
    }
    println!( "1,4,7 or 8 appears {} times in results", easy_counts );

    let input_file = fs::read_to_string("inputs/test.txt").unwrap();
    for line in input_file.lines() {
        let parts: Vec<&str> = line.split("|").collect();
        let inputs: Vec<&str> = parts[ 0 ].split_whitespace().collect();
        let mut segment_counts: BTreeMap<char,u32> = BTreeMap::new();
        let mut segments_to_digit: BTreeMap<BTreeSet<char>, u32> = BTreeMap::new();
        let mut unresolved_segments: Vec<BTreeSet<char>> = Vec::new();
        for part in inputs {
            let mut segments: BTreeSet<char> = part.as_bytes().iter().map(|x|*x as char).collect();
            for segment in &segments {
                *segment_counts.entry(*segment).or_insert(0)+=1;
            }
            match part.len() {
                2 => { segments_to_digit.insert(segments, 1 ); },
                4 => { segments_to_digit.insert(segments, 4 ); },
                3 => { segments_to_digit.insert(segments, 7 ); },
                7 => { segments_to_digit.insert(segments, 8 ); },
                _ => { unresolved_segments.push(segments);}
            }
        }
        // Find segment F used in nine digits
        let segment_F = segment_counts.iter().find(|(key,val)|**val==9).unwrap().0;
        // the one not containing it is 2
        let ( digit_2, unresolved_segments ):(Vec<BTreeSet<char>>, Vec<BTreeSet<char>> ) = unresolved_segments.iter().partition_in_place(|segs|!segs.contains(segment_F));


        let t = 0;

    }
 
}