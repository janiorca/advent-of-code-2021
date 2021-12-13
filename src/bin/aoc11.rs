use std::fs;

fn flash( grid: &mut Vec<Vec<u8>>, pos: (i32,i32 ), flash_counter: &mut u32 ) {   
    *flash_counter += 1;
    for y in 0i32.max(pos.0-1 )..=9.min(pos.0+1){
        for x in 0i32.max(pos.1-1 )..=9.min(pos.1+1){
            grid[y as usize][x as usize] += 1;
            if grid[y as usize][x as usize] == 10 {
                flash( grid, (y,x), flash_counter);
            }
        }
    }
}

fn main() {
    // Part 1 and 2 solved in same part
    let input = fs::read_to_string("inputs/aoc11.txt").unwrap();
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        grid.push( line.as_bytes().iter().map(|x|x-'0' as u8).collect() );
    }

    let mut total_flash_counter = 0;
    for _k in 0..1000 {
        let mut flash_counter = 0;
        // Age all the octopusses and flah them at the same time 
        for y in 0i32..10{
            for x in 0i32..10 {
                grid[y as usize][x as usize] += 1;
                if grid[y as usize][x as usize] == 10 {
                    flash(&mut grid, (y,x), &mut flash_counter );
                }
            }
        }

        // Reset any flashed to 0
        for y in 0i32..10{
            for x in 0i32..10 {
                if grid[y as usize][x as usize] > 9 {
                    grid[y as usize][x as usize] = 0;
                }
            }
        }

        total_flash_counter += flash_counter;
        if _k == 99 {
            println!("{} flashes in 100 rounds", total_flash_counter );
        }

        if flash_counter == 100 {
            println!("all synchronized after {} rounds", _k+1 );
            break;
        }
    }
}