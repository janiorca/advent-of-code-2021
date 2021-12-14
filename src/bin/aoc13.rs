use std::fs;

#[derive(Copy, Clone, PartialEq)]
struct Point{
    x: i32,
    y: i32
}

fn fold_x( points: Vec<Point>, fold_pos: i32) -> Vec<Point>{
    let (mut old,mut changed): (Vec<Point>,Vec<Point>) = points.iter().filter( |pnt|(*pnt).x != fold_pos ).partition( |pnt|(*pnt).x<fold_pos);
    changed = changed.iter().map( |pnt| Point{ x:fold_pos - (pnt.x-fold_pos), y:pnt.y }).filter(|pnt|!old.contains(pnt)).collect();
    old.append(&mut changed);
    return old;
}

fn fold_y( points: Vec<Point>, fold_pos: i32) -> Vec<Point>{
    let (mut old,mut changed): (Vec<Point>,Vec<Point>) = points.iter().filter( |pnt|(*pnt).y != fold_pos ).partition( |pnt|(*pnt).y<fold_pos);
    changed = changed.iter().map( |pnt| Point{ x:pnt.x, y:fold_pos - (pnt.y-fold_pos) }).filter(|pnt|!old.contains(pnt)).collect();
    old.append(&mut changed);
    return old;
}

fn main() {
    let input = fs::read_to_string("inputs/aoc13.txt").unwrap();
    let mut lines = input.lines();
    let mut points: Vec<Point> = Vec::new();
    loop{
        let line = lines.next().unwrap();
        if line.len() == 0 {
            break;
        }
        let xy: Vec<&str> = line.split(',').collect();
        let x = i32::from_str_radix(xy[0], 10 ).unwrap();
        let y = i32::from_str_radix(xy[1], 10 ).unwrap();
        points.push(Point{x,y});
    }

    // Part1, fold once
    points = fold_x( points, 655);
    println!("{} points remaining", points.len());

    // PArts 2, perform all the folds and show the image
    let mut x_size = 0;
    let mut y_size = 0;
    while let Some( fld ) = lines.next() {
        let prepped = fld.to_owned().replace(&"fold along ", &"");
        let parts: Vec<&str> = prepped.split('=').collect();
        if parts[0].chars().next().unwrap() =='x' {
            x_size = i32::from_str_radix(parts[1],10).unwrap();
            points = fold_x( points, x_size);
        } else {
            y_size = i32::from_str_radix(parts[1],10).unwrap();
            points = fold_y( points, y_size);
        }
    }

    let mut grid = vec![vec![0;x_size as usize];y_size as usize];
    for point in points {
        grid[point.y as usize][point.x as usize] = 1;
    }

    for y in 0..y_size {
        for x in 0..x_size {
            if grid[y as usize][x as usize] == 0 {
                print!(".")
            } else {
                print!("#")
            }
        }
        println!("");
    }
}