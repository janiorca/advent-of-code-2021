use std::fs;

fn crab_move_cost( dest: i32, pos: i32 ) -> i32 {
    let x = (dest-pos).abs();
    return x*(x+1)/2;
}

fn main() {
    // Part 1
    let input = fs::read_to_string("inputs/aoc7.txt").unwrap();
    let positions: Vec<i32> = input.split(',').map(|x|i32::from_str_radix(x, 10).unwrap()).collect();
    let min_pos = *positions.iter().min().unwrap();
    let max_pos = *positions.iter().max().unwrap();
    let mut cheapest_cost = i32::MAX;
    let mut cheapest_dest = 0;
    for destination in min_pos..=max_pos {
        let cost = positions.iter().fold(0,|acc,pos| { acc + (destination-pos).abs() } );
        if cost < cheapest_cost {
            cheapest_cost =  cost;
            cheapest_dest = destination;
        }
    }
    println!( "cheapest dest: {} cost: {}", cheapest_dest, cheapest_cost );

    // Part 2 ( sames as 1 with a different cost function )
    let mut cheapest_cost = i32::MAX;
    let mut cheapest_dest = 0;
    for destination in min_pos..=max_pos {
        let cost = positions.iter().fold(0,|acc,pos| { acc + crab_move_cost(destination,*pos) } );
        if cost < cheapest_cost {
            cheapest_cost =  cost;
            cheapest_dest = destination;
        }
    }
    println!( "cheapest dest: {} cost: {}", cheapest_dest, cheapest_cost );
}

 