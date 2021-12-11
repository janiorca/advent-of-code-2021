use std::fs;
use std::collections::VecDeque;

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
    let mut basin_seeds: Vec<(usize,usize)> = Vec::new();           // Store the basin seeds for part 2
    for y in 0..height {
        for x in 0..width {
            let depth = map[y][x];
            if ( x==0 || map[y][x-1] > depth ) && ( x==width-1 || map[y][x+1] > depth ) &&   
                ( y==0 || map[y-1][x] > depth ) && ( y==height-1 || map[y+1][x] > depth ) {
                risk += depth as u32  + 1;
                basin_seeds.push( (x,y));
            }
        }
    }
    println!("total risk {}", risk);

    // Part 2
    let mut basin_sizes = vec![0u32;0];
    for basin in basin_seeds {
        let mut basin_size = 0;
        let mut fill_map = vec![ vec![ 0; width]; height];
        let mut fill_queue: VecDeque<(usize,usize)> = VecDeque::new();
        fill_queue.push_back( ( basin.0, basin.1 ));

        // fill the basin ( very ineffcient way of doing it)
        while !fill_queue.is_empty() {
            let (x,y) : (usize,usize) = fill_queue.pop_front().unwrap();
            if fill_map[y][x] == 1 {
                continue;
            } else {
                fill_map[y][x] = 1;
                basin_size +=1;
                if x>0 && map[y][x-1] != 9 { fill_queue.push_back( ( x-1, y )) ;}
                if x<width-1 && map[y][x+1] != 9 { fill_queue.push_back( ( x+1, y )) ;}
                if y>0 && map[y-1][x] != 9 { fill_queue.push_back( ( x, y-1 )) ;}
                if y<height-1 && map[y+1][x] != 9 { fill_queue.push_back( ( x, y+1 )) ;}
            }
        }
        basin_sizes.push(basin_size);
    }
    basin_sizes.sort();
    basin_sizes.reverse();
    println!( "result {}", basin_sizes[0]*basin_sizes[1]*basin_sizes[2]);

}