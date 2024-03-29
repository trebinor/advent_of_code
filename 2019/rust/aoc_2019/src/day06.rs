use itertools::Itertools;
use std::collections::HashMap;

#[aoc(day06, part1, original)]
pub fn original_06a(input: &str) -> u32 {
    let objects: HashMap<_, _> = input
        .lines()
        .flat_map(|l| l.trim().rsplitn(2, ")"))
        .tuples()
        .collect();
    let mut orbit_count = 0;
    for mut parent in objects.keys() {
        while let Some(node) = objects.get(parent) {
            orbit_count += 1;
            parent = node;
        }
    }
    orbit_count
}

#[aoc(day06, part2, original)]
pub fn original_06b(input: &str) -> u32 {
    let objects: HashMap<_, _> = input
        .lines()
        .flat_map(|l| l.trim().rsplitn(2, ")"))
        .tuples()
        .collect();
    let mut orbital_transfers = 0;
    let mut you_parents: Vec<&str> = Vec::new();
    let mut san_parents: Vec<&str> = Vec::new();
    let mut you_orbits_chain = objects.get("YOU");
    let mut san_orbits_chain = objects.get("SAN");
    while let Some(node) = you_orbits_chain {
        you_parents.push(you_orbits_chain.unwrap());
        you_orbits_chain = objects.get(node);
    }
    while let Some(node) = san_orbits_chain {
        san_parents.push(san_orbits_chain.unwrap());
        san_orbits_chain = objects.get(node);
    }

    for y in you_parents {
        if let Some(pos) = san_parents.iter().position(|n| *n == y) {
            orbital_transfers += pos as u32;
            break;
        } else {
            orbital_transfers += 1;
        }
    }
    orbital_transfers
}
