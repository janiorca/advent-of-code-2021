use std::fs;

/*
    Solution I implented as 'the first thing that pops into my head'. It tracks the cost of reaching the given cell in path_cost. 
    Initially this is unknown for most cells ( set their value to 1,000,000) except for the top left which costs 0 to reach
    Each iteration updates the smallest known cost to reach the cell. this updates will make further updates necessary until there are no more updates
    ( at this point each cell stores the shortest cost to reach it)

    This is very inefficient compared to somehting like Dijkstra
 */
fn solve( grid_cost: &Vec<Vec<u32>> ) -> u32 {
    let mut path_cost = vec![ vec![ 1000000; grid_cost[0].len() ]; grid_cost.len() ]; // cost of reaching the point from the top left
    path_cost[0][0] = 0;
    loop{
        let mut updates = 0;
        for y in 0..path_cost.len() as i32{
            for x in 0..path_cost[0].len() as i32{
                for offsets in [ (0 as i32,1 as i32), (0,-1), (1,0),(-1,0)] {
                    let src_x = x+offsets.0;
                    let src_y = y+offsets.1;
                    if src_x>=0 && src_x<path_cost[0].len() as i32 && src_y>=0 && src_y<path_cost.len() as i32 {
                        let new_path_cost = path_cost[src_y as usize][src_x as usize] + grid_cost[y as usize][x as usize];
                        if new_path_cost < path_cost[y as usize][x as usize] {
                            path_cost[y as usize][x as usize] = new_path_cost;
                            updates += 1;
                        }
                    }
                }
            }
        }
        println!("Updates {}", updates);
        if updates == 0 {
            break;
        }
    }
    return path_cost[ path_cost.len()-1][path_cost[99].len()-1];
}

fn main() {
    let input = fs::read_to_string("inputs/aoc15.txt").unwrap();
    let mut grid_cost: Vec<Vec<u32>> = Vec::new();       // cost of entering the grid point
    for line in input.lines() {
        grid_cost.push( line.as_bytes().iter().map(|x|(x-'0' as u8 ) as u32 ).collect() );
    }

    println!( "Part1: {}", solve(&grid_cost));

    let mut large_grid_cost = vec![ vec![ 0; grid_cost[0].len()*5 ]; grid_cost.len()*5 ];
    for y in 0..large_grid_cost.len(){
        for x in 0..large_grid_cost[0].len(){
            let mut cost = grid_cost[ (y%grid_cost.len()) as usize][(x%grid_cost.len()) as usize ];
            let offset = ( y/grid_cost.len() + x/grid_cost[0].len() ) as u32;
            cost += offset;
            if cost > 9 {
                cost -= 9;
            }
            large_grid_cost[y][x] = cost;
        }
    }
    println!( "Part2: {}", solve(&large_grid_cost));
}