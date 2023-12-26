use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Recipe {
    reactants: HashMap<String, u64>,
    product: (String, u64),
}

/*
 * This queueing approach was inspired by a gist from u/aurele.
 * My previous approach was an attempt at recursion and failed due
 * to the borrow checker and my horrible stucture.
 */
fn ore_per_fuel(recipes: &[Recipe], fuel: u64) -> u64 {
    let mut required_materials: Vec<(String, u64)> = Vec::new();
    let mut created_materials: HashMap<String, u64> = HashMap::new();
    let mut ore: u64 = 0;
    required_materials.push(("FUEL".to_string(), fuel));
    while let Some((product, mut amount)) = required_materials.pop() {
        if let Some(&created) = created_materials.get(&product) {
            let consumed_materials = std::cmp::min(created, amount);
            amount -= consumed_materials;
            created_materials.insert(product.to_string(), created - consumed_materials);
        }
        //if there's any leftovers
        if amount > 0 {
            let product_recipe = recipes.iter().find(|x| x.product.0 == product).unwrap();
            let factor = (amount + product_recipe.product.1 - 1) / product_recipe.product.1;
            for (k, v) in product_recipe.reactants.clone() {
                if k == "ORE" {
                    ore += (v * factor) as u64;
                } else {
                    required_materials.push((k.to_string(), v * factor));
                }
            }
            created_materials.insert(
                product.to_string(),
                product_recipe.product.1 * factor - amount,
            );
        }
    }
    ore
}

#[aoc_generator(day14)]
fn generate_recipes(input: &str) -> Vec<Recipe> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?P<reactants>(?P<reactant>[0-9]+ [A-Z]+,? )+)=> (?P<product>[0-9]+ [A-Z]+)"
        )
        .unwrap();
    }
    let mut recipes = Vec::new();
    for l in input.lines() {
        let mut recipe_reactants: HashMap<String, u64> = HashMap::new();
        let caps = RE.captures(l.trim()).unwrap();
        let reactants_str: Vec<&str> = caps
            .name("reactants")
            .unwrap()
            .as_str()
            .trim()
            .split(',')
            .collect();
        for r in reactants_str {
            let mut take2 = r.trim().split(' ').take(2);
            let i: (u64, String) = (
                take2.next().unwrap().parse::<u64>().unwrap(),
                take2.next().unwrap().to_string(),
            );
            recipe_reactants.insert(i.1, i.0);
        }
        let mut product_take2 = caps
            .name("product")
            .unwrap()
            .as_str()
            .trim()
            .split(' ')
            .take(2);
        let i: (u64, String) = (
            product_take2.next().unwrap().trim().parse::<u64>().unwrap(),
            product_take2.next().unwrap().trim().to_string(),
        );
        let r: Recipe = Recipe {
            reactants: recipe_reactants.clone(),
            product: (i.1, i.0),
        };
        recipes.push(r);
    }
    recipes
}

#[aoc(day14, part1)]
pub fn solution_14a(recipes: &[Recipe]) -> u64 {
    ore_per_fuel(recipes, 1)
}

#[aoc(day14, part2)]
pub fn solution_14b(recipes: &[Recipe]) -> u64 {
    // Iteration was too slow, so a bsearch was obvious
    let mut low: u64 = 0;
    let mut high: u64 = 4_000_000_000;
    let mut mid: u64 = 0;
    const ORE_TARGET: u64 = 1_000_000_000_000u64;
    while low <= high {
        mid = (high + low) / 2;
        let opf = ore_per_fuel(recipes, mid);
        if opf < ORE_TARGET {
            low = mid + 1;
        } else if opf > ORE_TARGET {
            high = mid - 1;
        } else {
            break;
        }
    }
    mid
}

#[cfg(test)]
mod tests {
    use crate::day14::generate_recipes;
    use crate::day14::solution_14a;
    use crate::day14::solution_14b;
    use std::fs;
    const ANSWER_14A: u64 = 443_537;
    const ANSWER_14B: u64 = 2_910_558;

    #[test]
    fn t14a() {
        assert_eq!(
            ANSWER_14A,
            solution_14a(&generate_recipes(
                &fs::read_to_string("input/2019/day14.txt").unwrap().trim()
            ))
        );
    }
    #[test]
    fn t14b() {
        assert_eq!(
            ANSWER_14B,
            solution_14b(&generate_recipes(
                &fs::read_to_string("input/2019/day14.txt").unwrap().trim()
            ))
        );
    }
}
