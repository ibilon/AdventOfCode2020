use std::collections::HashMap;

pub fn run() -> () {
	println!("=== Day 15 ===");

	let answer1 = part1();
	println!("Part1: {}", answer1);

	let answer2 = part2();
	println!("Part2: {}", answer2);
}

fn part1() -> i32 {
	eval(2020)
}

fn part2() -> i32 {
	eval(30_000_000)
}

fn get_data() -> Vec<i32> {
	std::fs::read_to_string("data/day15.txt")
		.expect("Couldn't read data file")
		.split("\n")
		.map(|row| row.parse().expect("Not a number"))
		.collect()
}

fn eval(max_turn: i32) -> i32 {
	let mut memory = HashMap::new();
	let mut turn = 1;
	let mut last = 0;

	for number in get_data() {
		memory.insert(number, (turn, 0));
		last = number;
		turn += 1;
	}

	while turn <= max_turn {
		let previous_turn = *memory.entry(last).or_default();

		if previous_turn.1 != 0 {
			last = previous_turn.0 - previous_turn.1;
		} else {
			last = 0;
		}

		let e = memory.entry(last).or_default();
		*e = (turn, e.0);

		turn += 1;
	}

	last
}
