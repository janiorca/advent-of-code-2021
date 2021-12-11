use std::fs;

fn main() {
    // Part 1
    let input = fs::read_to_string("inputs/aoc9.txt").unwrap();
    let mut map: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        let row: Vec<u8> = line.as_bytes().iter().map(|ch|ch-'0' as u8).collect();
        map.push( row );        
    }

    let mut risk:u32 = 0;
    let width = map[0].len();
    let height = map.len();
    for y in 0..height {
        for x in 0..width {
            let depth = map[y][x];
            if ( x>0 && map[y][x-1] >= depth ) || ( x<width-1 && map[y][x+1] >= depth ) ||   
                ( y>0 && map[y-1][x] >= depth ) || ( x<height-1 && map[y+1][x] >= depth ) {
                risk += depth as u32  + 1;                            
            }
        }
    }
    println!("total risk {}", risk);
}