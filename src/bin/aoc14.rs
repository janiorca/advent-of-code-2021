use std::{fs, collections::HashMap};

fn main() {
    let input = fs::read_to_string("inputs/aoc14.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut pattern = lines[0].to_owned();
    let mut rules: HashMap<&str,&str> = HashMap::new();
    for line in &lines[2..] {
        let mut parts = line.split(" -> ");
        rules.insert( parts.next().unwrap(), parts.next().unwrap());
    }

    // Part1. brute force 10 generations
    for _step in 0..10 {
        let mut new_pattern: String = String::new();
        for pos in 0..pattern.len()-1 {
            new_pattern.push_str(&pattern[pos..pos+1]);
            let rule_key = &pattern[ pos..pos+2]; 
            if rules.contains_key(rule_key) {
                new_pattern.push_str(&rules.get(rule_key).unwrap());
            }
        }
        new_pattern.push(pattern.pop().unwrap());
        pattern = new_pattern;
    }
    let mut counts: HashMap<char,u32> = HashMap::new();
    for c in pattern.chars() {
        *counts.entry(c).or_insert(0)+=1;
    }
    let diff= counts.values().max().unwrap() - counts.values().min().unwrap();
    println!("Len {} diff {}", pattern.len(), diff);

    // PArt 2. Keep track of the number of pairs. Each pair generates two new pairs but the system is always self contained (the pairs are always part of the existing set of pairs so we can just count pairs)
    let pattern = lines[0].to_owned();
    let mut counts: HashMap<char,u64> = HashMap::new();
    let mut pair_counts: HashMap<&str, u64 > = HashMap::new();
    let mut pair_rules: HashMap<&str,(char, String,String)> = HashMap::new();       // rule key,  ( generating char, first pair, second pair )
    for c in pattern.chars() {
        *counts.entry(c).or_insert(0)+=1;
    }
    for pos in 0..pattern.len()-1 {
        *pair_counts.entry(&pattern[ pos..pos+2]).or_insert(0)+=1;
    }
    for rule in rules{
        let mut chars = rule.0[0..2].chars();
        let left = chars.next().unwrap().to_string() + &rule.1.to_owned();
        let  right = rule.1.to_owned() + &chars.next().unwrap().to_string();
        pair_rules.insert(rule.0, (rule.1.chars().next().unwrap(), left,right));
    }

    for _step in 0..40 {
        let mut new_pair_counts: HashMap<&str, u64 > = HashMap::new();
        for pair in &pair_counts {
            let action = pair_rules.get(pair.0).unwrap();
            *counts.entry( action.0 ).or_insert(0)+=pair.1;
            *new_pair_counts.entry(&action.1).or_insert(0)+=pair.1;
            *new_pair_counts.entry(&action.2).or_insert(0)+=pair.1;
        }
        pair_counts = new_pair_counts;
    }
    let len: u64 = counts.iter().map(|(_key,val)|val).sum::<u64>();
    let max_count = *counts.iter().map(|(_key,val)|val).max().unwrap();
    let min_count = *counts.iter().map(|(_key,val)|val).min().unwrap();
    println!( "Len {} diff {} ",len, max_count-min_count );

}
