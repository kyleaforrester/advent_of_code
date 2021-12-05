use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin");

    let mut map: HashMap<String, Vec<HashSet<String>>> = HashMap::new();
    let mut all_ingredients_counts: HashMap<String, u32> = HashMap::new();

    for line in buffer.split('\n').filter(|x| x.len() > 0) {
        let allergy_split: Vec<&str> = line.split("(contains ").collect();

        let ingredients: HashSet<String> = allergy_split[0]
            .trim()
            .split(' ')
            .map(|x| x.to_string())
            .collect();
        let allergies: Vec<&str> = allergy_split[1]
            .trim()
            .strip_suffix(')')
            .unwrap()
            .split(", ")
            .collect();

        for allergy in allergies.iter() {
            if map.contains_key(&allergy.to_string()) {
                map.get_mut(&allergy.to_string())
                    .unwrap()
                    .push(ingredients.clone());
            } else {
                let mut vec = Vec::new();
                vec.push(ingredients.clone());
                map.insert(allergy.to_string(), vec);
            }
        }

        for ingredient in ingredients.iter() {
            match all_ingredients_counts.get_mut(&ingredient.to_string()) {
                Some(i) => *i += 1,
                None => {
                    all_ingredients_counts.insert(ingredient.to_string(), 1);
                    ()
                }
            }
        }
    }

    let mut possible_ingredients: HashMap<String, HashSet<String>> = HashMap::new();
    for allergy in map.keys() {
        let mut ingredient_counts: HashMap<String, u32> = HashMap::new();
        for set in map.get(&allergy.to_string()).unwrap().iter() {
            for ingredient in set.iter() {
                match ingredient_counts.get_mut(&ingredient.to_string()) {
                    Some(i) => *i += 1,
                    None => {
                        ingredient_counts.insert(ingredient.to_string(), 1);
                        ()
                    }
                }
            }
        }

        possible_ingredients.insert(
            allergy.to_string(),
            ingredient_counts
                .iter()
                .filter(|x| *x.1 as usize == map.get(&allergy.to_string()).unwrap().len())
                .map(|x| x.0.to_string())
                .collect(),
        );
    }

    let mut dangerous_ingredients = HashSet::new();
    for p_i in possible_ingredients.values() {
        for d_i in p_i.iter() {
            dangerous_ingredients.insert(d_i.to_string());
        }
    }

    let safe_ingredients: HashSet<String> = all_ingredients_counts
        .keys()
        .filter(|x| !dangerous_ingredients.contains(*x))
        .map(|x| x.to_string())
        .collect();

    let sum: u32 = safe_ingredients
        .iter()
        .map(|x| all_ingredients_counts.get(&x.to_string()).unwrap())
        .sum();

    println!("{}", sum);
}
