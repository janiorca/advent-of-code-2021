use std::{fs, collections::HashMap};

fn find_paths<'a>( cave_connections: &HashMap<&str,Vec<&'a str>>, found_paths: &mut Vec<Vec<&'a str>>, current_path: &Vec<&'a str> ) {
    for destination in cave_connections.get(current_path.last().unwrap()).unwrap() {
        // Did we reach the end
        if (*destination).eq("end") { 
            found_paths.push(current_path.clone());
            continue;
        }
        // Are we revisiting a small cave
        if (*destination).chars().next().unwrap().is_lowercase() {
            if current_path.contains(&destination) {
                continue;
            }
        }
        //otherwise. keep recursing
        let mut deeper_path: Vec<&str> = current_path.to_owned();
        deeper_path.push(destination);
        find_paths(cave_connections, found_paths, &deeper_path);
    }
}

fn find_paths_2<'a>( cave_connections: &HashMap<&str,Vec<&'a str>>, found_paths: &mut Vec<Vec<&'a str>>, current_path: &Vec<&'a str>, has_multi_visit: bool ) {
    for destination in cave_connections.get(current_path.last().unwrap()).unwrap() {
        let mut local_multi_visit = has_multi_visit;
        if (*destination).eq("start") {  
            continue;
        }

        // Did we reach the end
        if (*destination).eq("end") { 
            found_paths.push(current_path.clone());
            continue;
        }
        // Are we revisiting a small cave
        if (*destination).chars().next().unwrap().is_lowercase() {
            // Does it already have one multi visit
            if current_path.contains(&destination) {
                if has_multi_visit {
                    continue;
                } else {
                    local_multi_visit = true;
                }
            }
        }
        //otherwise. keep recursing
        let mut deeper_path: Vec<&str> = current_path.to_owned();
        deeper_path.push(destination);
        find_paths_2(cave_connections, found_paths, &deeper_path, local_multi_visit);
    }
}

fn main() {
    let input = fs::read_to_string("inputs/aoc12.txt").unwrap();
    // Build the connection map
    let mut cave_connections: HashMap<&str,Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let locations: Vec<&str> = line.split('-').collect();
        cave_connections.entry(locations[0]).or_insert(Vec::new()).push(locations[1]);
        cave_connections.entry(locations[1]).or_insert(Vec::new()).push(locations[0]);
    }

    // FInd all paths for part1
    let mut found_paths: Vec<Vec<&str>> = Vec::new();
    find_paths(&cave_connections, &mut found_paths, &vec!["start"]);
    println!( "There are {} unique paths", found_paths.len() );

    // Find all paths for part2
    let mut found_paths: Vec<Vec<&str>> = Vec::new();
    find_paths_2(&cave_connections, &mut found_paths, &vec!["start"], false);
    println!( "There are {} unique paths with double visits", found_paths.len() );
}
