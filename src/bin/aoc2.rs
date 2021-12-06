use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/aoc2.txt").unwrap();
    let mut x = 0;
    let mut y = 0;
    let commands = input.lines();
    commands.for_each(|commands| {
        let parts: Vec<&str> = commands.split( ' ' ).collect();
        let arg = i32::from_str_radix( parts.get(1).unwrap(), 10 ).unwrap();
        let c = *parts.get(0).unwrap();
        x += match c {
            "forward" => arg,
            _ => 0
        };
        y += match c {
            "up" => -arg,
            "down" => arg,
            _ => 0
        }
    });
    println!("Pos: {}", x*y);

    let input = fs::read_to_string("inputs/aoc2.txt").unwrap();
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    let commands = input.lines();
    commands.for_each(|commands| {
        let parts: Vec<&str> = commands.split( ' ' ).collect();
        let arg = i32::from_str_radix( parts.get(1).unwrap(), 10 ).unwrap();
        let c = *parts.get(0).unwrap();
        match c {
            "forward" => {
                x += arg;
                y += aim*arg;
            },
            _ => {}
        };
        aim += match c {
            "up" => -arg,
            "down" => arg,
            _ => 0
        }
    });
    println!("Pos: {}", x*y);

}
