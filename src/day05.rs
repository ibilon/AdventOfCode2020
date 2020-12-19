use std::{cmp::max, collections::HashSet};

pub fn run() {
	println!("=== Day 05 ===");

	let answer1 = part1();
	println!("Part1: {}", answer1);

	let answer2 = part2().expect("Couldn't find answer to part 2");
	println!("Part2: {}", answer2);
}

fn get_data() -> Vec<(Vec<bool>, Vec<bool>)> {
	std::fs::read_to_string("data/day05.txt")
		.expect("Couldn't read data file")
		.split('\n')
		.map(|row| {
			let mut chars = row.chars();
			(
				chars.by_ref().take(7).map(|c| c == 'F').collect(),
				chars.map(|c| c == 'L').collect(),
			)
		})
		.collect()
}

fn bsp(p: Vec<bool>, mut min: i32, mut max: i32) -> i32 {
	for e in p {
		let half = (max - min + 1) / 2;

		if e {
			max -= half;
		} else {
			min += half;
		}
	}

	min
}

fn compute_seats() -> HashSet<i32> {
	let data = get_data();
	let mut ids = HashSet::new();

	for pass in data {
		let row = bsp(pass.0, 0, 127);
		let seat = bsp(pass.1, 0, 7);
		let id = row * 8 + seat;
		ids.insert(id);
	}

	ids
}

fn part1() -> i32 {
	compute_seats().iter().fold(0, |acc, e| max(acc, *e))
}

fn part2() -> Result<usize, ()> {
	let data = compute_seats();
	let mut seats = Vec::new();

	for id in data {
		let id = id as usize;

		while id >= seats.len() {
			seats.push(false);
		}

		seats[id] = true;
	}

	for i in (0..seats.len()).rev() {
		if !seats[i] {
			return Ok(i);
		}
	}

	Err(())
}
