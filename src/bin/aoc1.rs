use std::fs;

fn main() {
    // Part 1
    let input = fs::read_to_string("inputs/aoc1.txt").unwrap();
    let mut increments = 0;
    let mut old_value : u64 = u64::MAX;
    let values = input.lines();
    values.for_each(|value| {
        let new_value = u64::from_str_radix(value,10).unwrap();
        if new_value > old_value {
            increments +=1;
        }
        old_value = new_value;
    });
    println!("Increments: {}", increments);

    // Part 2
    let input = fs::read_to_string( "inputs/aoc1.txt").unwrap();
    let values: Vec<u64> = input.lines().map(|x|u64::from_str_radix(x,10).unwrap()).collect();
    let mut averages = Vec::<u64>::new();
    for idx in 0..values.len()-2 {
        averages.push( values.get( idx .. idx+3 ).unwrap().iter().sum() );
    }
    increments = 0;
    for idx in 0..averages.len()-1 {
        if averages[ idx+1] > averages[idx] {
            increments += 1;
        }
    }
    println!("Averaged Increments: {}", increments);
}
