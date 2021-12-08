use std::fs;

fn main() {
    // Part 1 ( completely naive implementaiton simulating individual fish)
    let input = fs::read_to_string("inputs/aoc6.txt").unwrap();
    let mut fishes: Vec<i32> = input.split(',').map(|x|i32::from_str_radix(x, 10).unwrap()).collect();

    for _days in 0..80 {
        let num_new_fish = fishes.iter().filter(|x| **x == 0 ).count();
        for fish in &mut fishes{
            if *fish == 0 { 
                *fish = 6;
            } else {
                *fish -= 1;
            }
        }
        fishes.append(&mut vec![ 8; num_new_fish]);
    }
    println!( "There are {} fish", fishes.len());

    // Part 2 ( Just keep track of how many fish there are of each age)
    let input = fs::read_to_string("inputs/aoc6.txt").unwrap();
    let mut fish_counts = [ 0u64;9];
    let fishes: Vec<u32> = input.split(',').map(|x|u32::from_str_radix(x, 10).unwrap()).collect();
    for fish in fishes {
        fish_counts[ fish as usize ] += 1;
    }

    for _days in 0..256 {
        let new_fish = fish_counts[0];
        fish_counts.rotate_left(1);
        fish_counts[ 6 ] += new_fish;
        fish_counts[ 8 ] = new_fish;
    }

    print!( "There are {} fish", fish_counts.iter().sum::<u64>());
}

 