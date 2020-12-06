use std::collections::HashMap;

pub fn run() -> () {
	println!("=== Day 06 ===");

	let answer1 = part1();
	println!("Part1: {}", answer1);

	let answer2 = part2();
	println!("Part2: {}", answer2);
}

fn get_data() -> Vec<(usize, HashMap<char, usize>)> {
	std::fs::read_to_string("data/day06.txt")
		.expect("Couldn't read data file")
		.split("\n\n")
		.map(|group| {
			let group = group
				.split("\n")
				.map(|row| row.chars().collect())
				.collect::<Vec<Vec<char>>>();
			let mut questions = HashMap::new();

			for person in group.iter() {
				for question in person {
					*questions.entry(*question).or_insert(0) += 1;
				}
			}

			(group.len(), questions)
		})
		.collect()
}

fn part1() -> usize {
	get_data()
		.iter()
		.fold(0, |acc, (_, questions)| acc + questions.len())
}

fn part2() -> usize {
	get_data().iter().fold(0, |acc, (size, questions)| {
		acc + questions.iter().filter(|(_, v)| **v == *size).count()
	})
}
