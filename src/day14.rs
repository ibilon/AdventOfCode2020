use std::collections::HashMap;

pub fn run() -> () {
	println!("=== Day 14 ===");

	let answer1 = part1();
	println!("Part1: {}", answer1);

	let answer2 = part2();
	println!("Part2: {}", answer2);
}

enum Instruction {
	Mask(String),
	Write(u64, u64),
}

fn get_data() -> Vec<Instruction> {
	std::fs::read_to_string("data/day14.txt")
		.expect("Couldn't read data file")
		.split("\n")
		.map(|row| {
			if row.starts_with("mask = ") {
				Instruction::Mask(String::from(&row[7..]))
			} else {
				let i = row.find("]").expect("Wrong format");
				Instruction::Write(
					row[4..i].parse().expect("Not a number"),
					row[i + 4..].parse().expect("Not a number"),
				)
			}
		})
		.collect()
}

fn part1() -> u64 {
	let mut memory = HashMap::new();
	let mut mask = String::new();

	for instruction in get_data() {
		match instruction {
			Instruction::Mask(m) => mask = m,

			Instruction::Write(index, mut value) => {
				let ptr = memory.entry(index).or_insert(0);

				for (i, c) in mask.char_indices() {
					match c {
						'X' => (),
						'0' => value &= !0 - (1 << (36 - i - 1)),
						'1' => value |= 1 << (36 - i - 1),
						_ => panic!("Unknown char in mask"),
					}
				}

				*ptr = value;
			}
		}
	}

	memory.iter().fold(0, |acc, (_, value)| acc + value)
}

fn set_memory_mult(
	memory: &mut HashMap<u64, u64>,
	mask: &String,
	start: usize,
	index: u64,
	value: u64,
) -> () {
	for (i, c) in mask.char_indices().skip(start) {
		match c {
			'X' => {
				set_memory_mult(memory, mask, i + 1, index | (1 << (36 - i - 1)), value);
				set_memory_mult(
					memory,
					mask,
					i + 1,
					index & (!0 - (1 << (36 - i - 1))),
					value,
				);
				return;
			}
			_ => (),
		}
	}

	*memory.entry(index).or_insert(0) = value;
}

fn part2() -> u64 {
	let mut memory = HashMap::new();
	let mut mask = String::new();

	for instruction in get_data() {
		match instruction {
			Instruction::Mask(m) => mask = m,

			Instruction::Write(mut index, value) => {
				for (i, c) in mask.char_indices() {
					match c {
						'1' => index |= 1 << (36 - i - 1),
						_ => (),
					}
				}

				set_memory_mult(&mut memory, &mask, 0, index, value)
			}
		}
	}

	memory.iter().fold(0, |acc, (_, value)| acc + value)
}
