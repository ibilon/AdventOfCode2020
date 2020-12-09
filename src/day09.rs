use std::cmp::{max, min};

pub fn run() -> () {
	println!("=== Day 09 ===");

	let answer1 = part1().expect("Couldn't compute part 1");
	println!("Part1: {}", answer1);

	let answer2 = part2(answer1).expect("Couldn't compute part 2");
	println!("Part2: {}", answer2);
}

fn get_data() -> Vec<i64> {
	std::fs::read_to_string("data/day09.txt")
		.expect("Couldn't read data file")
		.split("\n")
		.map(|row| row.parse().expect("Invalid number"))
		.collect()
}

fn valid(data: &[i64], d: i64) -> bool {
	for a in 0..data.len() {
		for b in a + 1..data.len() {
			if data[a] + data[b] == d {
				return true;
			}
		}
	}

	false
}

fn part1() -> Result<i64, ()> {
	let data = get_data();

	for i in 25..data.len() {
		if !valid(&data[i - 25..i], data[i]) {
			return Ok(data[i]);
		}
	}

	Err(())
}

fn part2(target:i64) -> Result<i64, ()> {
	let data = get_data();

	for i in 0..data.len() {
		let mut sum = 0;
		let mut count = 0;
		let mut smallest = i64::MAX;
		let mut highest = i64::MIN;

		while sum < target {
			let v = data[i + count];

			sum += v;
			count += 1;

			smallest = min(smallest, v);
			highest = max(highest, v);
		}

		if sum == target && count > 1 {
			return Ok(smallest + highest);
		}
	}

	Err(())
}
