use crate::utils::remove_vec;
use std::collections::{HashMap, HashSet};

pub fn run() {
	println!("=== Day 21 ===");

	let answer1 = part1();
	println!("Part1: {}", answer1);

	let answer2 = part2();
	println!("Part2: {}", answer2);
}

fn get_data() -> Vec<(Vec<String>, Vec<String>)> {
	std::fs::read_to_string("data/day21.txt")
		.expect("Couldn't read data file")
		.split('\n')
		.map(|row| {
			let p = row.find('(').unwrap();
			(
				row[..p - 1]
					.split(' ')
					.map(String::from)
					.collect::<Vec<_>>(),
				row[p + 10..row.len() - 1]
					.split(", ")
					.map(String::from)
					.collect::<Vec<_>>(),
			)
		})
		.collect::<Vec<_>>()
}

fn compute_allergens<'a>(
	allergens: &'a [String],
	allergens_possibilities: &mut HashMap<&'a String, Vec<String>>,
	ingredients: &[String],
) {
	for allergen in allergens {
		let intersection = allergens_possibilities
			.entry(allergen)
			.or_insert_with(|| ingredients.to_vec())
			.drain(..)
			.filter(|ingredient| ingredients.contains(&ingredient))
			.collect();

		allergens_possibilities.insert(allergen, intersection);
	}
}

fn part1() -> usize {
	let data = get_data();

	let mut all_ingredients = HashMap::new();
	let mut allergens_possibilities = HashMap::new();

	for (ingredients, allergens) in data.iter() {
		for ingredient in ingredients.iter() {
			*all_ingredients.entry(ingredient).or_insert(0) += 1;
		}

		compute_allergens(allergens, &mut allergens_possibilities, ingredients);
	}

	let mut possibles = HashSet::new();

	for (_, p) in allergens_possibilities.iter() {
		for pp in p {
			possibles.insert(pp.clone());
		}
	}

	all_ingredients
		.iter()
		.filter(|(i, _)| !possibles.contains(**i))
		.map(|(_, c)| *c)
		.sum()
}

fn part2() -> String {
	let data = get_data();

	let mut allergens_possibilities = HashMap::new();

	for (ingredients, allergens) in data.iter() {
		compute_allergens(allergens, &mut allergens_possibilities, ingredients);
	}

	let keys = allergens_possibilities
		.iter()
		.map(|(i, _)| (*i).clone())
		.collect::<Vec<_>>();
	loop {
		let mut did_remove = false;

		for n in 0..keys.len() {
			if allergens_possibilities[&keys[n]].len() == 1 {
				let value = allergens_possibilities.get(&keys[n]).unwrap()[0].clone();
				for (p, (_, pp)) in allergens_possibilities.iter_mut().enumerate() {
					if p == n {
						continue;
					}

					if remove_vec(pp, &value) {
						did_remove = true;
					}
				}
			}
		}

		if !did_remove {
			break;
		}
	}

	let mut values = allergens_possibilities
		.iter()
		.map(|(k, v)| (&v[0], *k))
		.collect::<Vec<_>>();
	values.sort_by(|(_, a), (_, b)| a.cmp(b));

	values
		.iter()
		.map(|(k, _)| (*k).clone())
		.collect::<Vec<_>>()
		.join(",")
}
