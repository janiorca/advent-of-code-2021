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

    // Part 2
    // a over conplicated solution that uses the number of times the the individual segments appear to figure out which segments
    // map to digits.... Good excercise with using Rust sets...
    let mut total = 0;
    let input_file = fs::read_to_string("inputs/aoc8.txt").unwrap();
    for line in input_file.lines() {
        let parts: Vec<&str> = line.split("|").collect();
        let inputs: Vec<&str> = parts[ 0 ].split_whitespace().collect();
        let mut segment_counts: BTreeMap<char,u32> = BTreeMap::new();
        let mut segments_to_digit: BTreeMap<BTreeSet<char>, u32> = BTreeMap::new();
        let mut segments_5: Vec<BTreeSet<char>> = Vec::new();
        let mut segments_6: Vec<BTreeSet<char>> = Vec::new();
        for part in inputs {
            let segments: BTreeSet<char> = part.as_bytes().iter().map(|x|*x as char).collect();
            for segment in &segments {
                *segment_counts.entry(*segment).or_insert(0)+=1;
            }
            match part.len() {
                2 => { segments_to_digit.insert(segments, 1 ); },
                4 => { segments_to_digit.insert(segments, 4 ); },
                3 => { segments_to_digit.insert(segments, 7 ); },
                7 => { segments_to_digit.insert(segments, 8 ); },
                5 => { segments_5.push(segments); },
                6 => { segments_6.push(segments); },
                _ => { panic!("Unexpected segment combo"); }
            }
        }
        // Find segment F (only one used in nine )digits
        let segment_F = segment_counts.iter().find(|(key,val)|**val==9).unwrap().0;
        // the 5-segmenter containing it is 2
        let number_2 = segments_5.iter().position(|segs| !segs.contains(segment_F)).unwrap();
        segments_to_digit.insert(segments_5.remove(number_2), 2 );

        // The segment B is the only one used by six dights
        let segment_B = segment_counts.iter().find(|(key,val)|**val==6).unwrap().0;
        // the five segmenter containing it is 5
        let number_5 = segments_5.iter().position(|segs| segs.contains(segment_B)).unwrap();
        segments_to_digit.insert(segments_5.remove(number_5), 5 );
        // the one the doesnt is 3
        segments_to_digit.insert(segments_5.remove(0), 3 );

        // The segment E is the only one used by four dights
        let segment_E = segment_counts.iter().find(|(key,val)|**val==4).unwrap().0;
        // 9 is the only 6 segmenter that doesnt use it
        let number_9 = segments_6.iter().position(|segs| !segs.contains(segment_E)).unwrap();
        segments_to_digit.insert(segments_6.remove(number_9), 9 );
        
        let seg8s = segments_6[0].iter().filter(|c|*segment_counts.get(*c).unwrap() == 8).count();
        if seg8s == 2  {
            segments_to_digit.insert(segments_6.remove(0), 0 );
            segments_to_digit.insert(segments_6.remove(0), 6 );
        } else {
            segments_to_digit.insert(segments_6.remove(0), 6 );
            segments_to_digit.insert(segments_6.remove(0), 0 );
        }

        // We have a map, now convert the output with it
        let outputs: Vec<&str> = parts[ 1 ].split_whitespace().collect();
        let mut value = 0;
        for part in outputs {
            value *= 10;
            let segments: BTreeSet<char> = part.as_bytes().iter().map(|x|*x as char).collect();
            let digit = segments_to_digit.get(&segments).unwrap();
            value += digit;
        }
        total += value;
    }
    println!("the total is {}", total);
 
}