use std::{ fs};

fn main() {
    // Part 1
    let input = fs::read_to_string("inputs/aoc3.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let code_width = lines[0].len();
    let mut counts = vec![0u32; code_width];
    for code in &lines {
        for pos in 0..code.len() {
            if code.as_bytes()[pos] == '1' as u8{
                counts[pos] +=1;
            }
        }
    };

    let mut gamma: u32= 0;
    let mut epsilon: u32= 0;
    for pos in 0..code_width {
        epsilon <<= 1;
        gamma <<= 1;
        if counts[pos] > lines.len() as u32/2 {
            gamma +=1;
        } else {
            epsilon += 1;
        };
    }
    println!( "gamma_string  {} {}",gamma, gamma * epsilon);


    // Part 2
    let input = fs::read_to_string("inputs/aoc3.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut values = Vec::new();
    for most_common in [ true, false] {
        let mut ogr_list: Vec<&str> = Vec::from_iter( lines.clone() );
        for bit_pos in 0..code_width{
            if ogr_list.len() == 1 {
                break;
            }
            let( ones, zeros ): (Vec<&str>, Vec<&str>) = ogr_list.iter().partition(|code| code.as_bytes()[bit_pos] == '1' as u8 );
            if (most_common && ones.len() >= zeros.len()) || (!most_common && ones.len() < zeros.len()) {
                ogr_list = ones;
            } else {
                ogr_list = zeros;
            }
        }
        values.push(ogr_list.pop().unwrap());
    }
    let result: Vec<u32> = values.iter().map( |x|u32::from_str_radix(x,2).unwrap()).collect();
    println!( "{:?}",result[0]*result[1] );
  

}
