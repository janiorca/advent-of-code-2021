use std::fs;

fn main() {
    // Part 1
    let input = fs::read_to_string("inputs/test.txt").unwrap();
    let t = 
    let mut sea_map = Box::new( vec![vec![1024;0]0]); 
    for line in input.lines(){
        let coords: Vec<u32> = line.replace(" -> ", ",").split(',').map(|v|u32::from_str_radix( v, 10 ).unwrap() ).collect();
        if coords[0] == coords[2] { // Same X
            for y in coords[1]..=coords[3] {
                sea_map[ y as usize ][ coords[0] as usize] += 1;
            }
        }
        if coords[1] == coords[3] { // Same Y
            for x in coords[0]..=coords[2] {
                sea_map[ coords[1] as usize ][ x as usize] += 1;
            }
        }
    }
    // Count map locations with value >= 2
    let mut count = 0;
    for row in &sea_map..iter() {
        count += row.iter().filter(|x|**x>=2).count();
    }
    print!( "dangerous positions {}", count);
}