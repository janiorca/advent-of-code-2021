use std::fs;

fn main() {
    // Part 1
    let lines = fs::read_to_string("inputs/test.txt").unwrap().lines();
    for line in lines{
        let nums: Vec<u32> = line.split( |x|" -> ").;
    }
    let mut fishes: Vec<i32> = input.split(',').map(|x|i32::from_str_radix(x, 10).unwrap()).collect();
}