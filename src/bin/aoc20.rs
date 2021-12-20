use std::fs;

fn add_map_border( map: &Vec<Vec<u32>>, border_size: usize  ) -> Vec<Vec<u32>> {
    let mut result: Vec<Vec<u32>> = Vec::new();
    for _t in 0..border_size {
        let row: Vec<u32> = vec![0;(border_size*2+map[0].len()) as usize];
        result.push( row )
    }
    for src_row in map {
        let row: Vec<u32> = [ vec![0;border_size], src_row.to_owned(), vec![0;border_size] ].concat();
        result.push( row )
    }
    for _t in 0..border_size {
        let row: Vec<u32> = vec![0;(border_size*2+map[0].len()) as usize];
        result.push( row )
    }

    return result;
}

fn set_map_border( map: &mut Vec<Vec<u32>>, border_size: usize, value: u32  ) {
    let width = map[0].len();
    let height = map.len();
    for y in 0..height {
        for x in 0..width {
            if x<border_size || x >= width-border_size || y<border_size || y>=height-border_size{
                map[y][x] = value;
            }
        }
    }
}

fn enhance( src_map: &Vec<Vec<u32>>, mapping: &Vec<u32> ) -> Vec<Vec<u32>> {
    let mut result = vec![ vec![ 0u32; src_map[1].len() ]; src_map.len()];

    for y in 1..src_map.len()-1 {
        for x in 1..src_map[0].len()-1 {
            let mut idx = 0;
            for (xo,yo) in [ (-1i32,-1i32), ( 0,-1), ( 1,-1),  (-1,0), ( 0,0), ( 1,0),  (-1,1), ( 0,1), ( 1,1) ] {
                idx <<=1;
                idx |= src_map[(y as i32+yo) as usize][(x as i32+xo) as usize] as u32;
            }
            result[y][x] = mapping[ idx as usize ];
        }
    }
    return result;
}

fn print_map( map: &Vec<Vec<u32>> ) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            print!("{}", if map[y][x] == 1 { "#"} else { "."});
        }
        println!("");
    }
    println!("");
}
fn main() {
    let input = fs::read_to_string("inputs/aoc20.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut mapping = vec![0u32;0];
    let mut line_iter = lines.iter();
    // Read mapping
    loop{
        let line = *line_iter.next().unwrap();
        if line.len() == 0 {
            break;
        }
        let mapping_line: Vec<u32> = line.chars().map(|x|if x=='.' { 0u32 } else {1u32 }).collect();
        mapping = [ mapping, mapping_line ].concat();
    }
    println!( "map len {}", mapping.len());

    // Read map
    let mut original_map: Vec<Vec<u32>> = Vec::new();
    while let Some( line ) = line_iter.next(){
        let map_row: Vec<u32> = line.chars().map(|x|if x=='.' { 0u32 } else {1u32 }).collect();
        original_map.push( map_row );
    }
    print_map( &original_map );

    // Part1
    let mut map = original_map.clone();
    map = add_map_border( &map, 4 );
    map = enhance(&map, &mapping);
    map = enhance(&map, &mapping);
    set_map_border( &mut map, 2, 0);
    let lit: u32 = map.iter().map(|x|x.iter().sum::<u32>() as u32 ).sum();
    println!( "{} lit pixels", lit );


    // Part2
    let mut map = original_map.clone();
    let num_iterations = 50;
    map = add_map_border( &map, num_iterations+1 );
    for it in 0..num_iterations {
        set_map_border( &mut map, num_iterations+1-it, (it as u32)&0x01);
        map = enhance(&map, &mapping);
    }
    let lit: u32 = map.iter().map(|x|x.iter().sum::<u32>() as u32 ).sum();
    println!( "{} lit pixels", lit );

}
