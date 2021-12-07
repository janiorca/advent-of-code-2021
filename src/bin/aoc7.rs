use std::{ fs, cmp::{min, max}};

fn main() {
    // Part 1
    let input = fs::read_to_string("inputs/aoc7.txt").unwrap();
    let positions: Vec<i32> = input.split(',').map(|x|i32::from_str_radix(x, 10).unwrap()).collect();
    let min_pos = positions.iter().fold(positions[0], |acc, &x| min( acc, x));
    let max_pos = positions.iter().fold(positions[0], |acc, &x| max( acc, x));
    let cheapest = i32::MAX;
    let cheapest_dest = 0;
    for destination in min_pos..=max_pos {
        let cost = positions.iter().fold(0,|acc,pos| { acc + (destination-pos).abs() } );
        println!( "dest: {} cost: {}", destination, cost );
    }
    let t = 0;
}