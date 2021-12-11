use std::fs;
use std::collections::HashMap;

fn main() {
    let closers = HashMap::from([ ('(',')'), ('{','}'), ('[', ']'), ('<', '>') ]);
    let mut illegal_counts: HashMap<char,u32> = HashMap::from([ (')',0), ('}',0), (']',0), ('>',0) ]);

    // Part 1 ( count corrupted lines -> meet unexpected chars)
    let input = fs::read_to_string("inputs/aoc10.txt").unwrap();
    for line in input.lines() {
        let mut stack: Vec<char> = Vec::new();
        for ch in line.chars() {
            if "([{<".find(ch ).is_some()  {
                stack.push(ch);
            } else if ch!=*closers.get(&stack.pop().unwrap()).unwrap() {
                *illegal_counts.get_mut(&ch).unwrap() += 1;
                break
            }
        }
    }
    println!( "error score {}", 3*illegal_counts.get(&')').unwrap() + 57*illegal_counts.get(&']').unwrap()+1197*illegal_counts.get(&'}').unwrap() + 25137*illegal_counts.get(&'>').unwrap() );

    // Part 2
    let input = fs::read_to_string("inputs/aoc10.txt").unwrap();
    let error_vals: HashMap<char,u64> = HashMap::from([ ('(',1), ('{',3), ('[',2), ('<',4) ]);
    let mut error_scores = vec![0u64;0];
    'lines: for line in input.lines() {
        let mut stack: Vec<char> = Vec::new();
        for ch in line.chars() {
            if "([{<".find(ch ).is_some()  {
                stack.push(ch);
            } else if ch!=*closers.get(&stack.pop().unwrap()).unwrap() {
                continue 'lines; 
            }
        }
        // Got here. If the stack is non-zero this is an inomplete line
        let mut line_error: u64 = 0;
        while !stack.is_empty() {
            line_error *= 5;
            line_error += error_vals.get(&stack.pop().unwrap()).unwrap();
        }
        error_scores.push(line_error);
    }
    error_scores.sort();
    println!( "error score {}", error_scores[ error_scores.len()/2] );

}