use std::cmp::min;

pub fn run() {
	println!("=== Day 11 ===");

	let answer1 = part1();
	println!("Part1: {}", answer1);

	let answer2 = part2();
	println!("Part2: {}", answer2);
}

fn part1() -> i32 {
	eval(
		|data, x, y| {
			let height = data.len() as i32;
			let width = data[0].len() as i32;
			let mut adjacent = 0;

			for yy in (if y == 0 { y } else { y - 1 })..min(y + 2, height) {
				for xx in (if x == 0 { x } else { x - 1 })..min(x + 2, width) {
					if yy == y && xx == x {
						continue;
					}

					if let Seat::Occupied = data[yy as usize][xx as usize] {
						adjacent += 1;
					}
				}
			}

			adjacent
		},
		4,
	)
}

fn part2() -> i32 {
	eval(
		|data, x, y| {
			let mut adjacent = 0;

			for yy in -1..2 {
				for xx in -1..2 {
					if yy == 0 && xx == 0 {
						continue;
					}

					if dir_has_seat_occupied(data, (x, y), (xx, yy)) {
						adjacent += 1;
					}
				}
			}

			adjacent
		},
		5,
	)
}

#[derive(Copy, Clone)]
enum Seat {
	Floor,
	Empty,
	Occupied,
}

fn get_data() -> Vec<Vec<Seat>> {
	std::fs::read_to_string("data/day11.txt")
		.expect("Couldn't read data file")
		.split('\n')
		.map(|row| {
			row.chars()
				.map(|c| match c {
					'.' => Seat::Floor,
					'L' => Seat::Empty,
					'#' => Seat::Occupied,
					_ => panic!("Unknown seat type"),
				})
				.collect()
		})
		.collect()
}

fn eval(adjacent: fn(data: &Vec<Vec<Seat>>, x: i32, y: i32) -> i32, required: i32) -> i32 {
	let mut data = get_data();
	let height = data.len();
	let width = data[0].len();

	loop {
		let mut changes = 0;
		let mut new_data = data.to_vec();

		for y in 0..height {
			for x in 0..width {
				let adjacent_count = adjacent(&data, x as i32, y as i32);

				match data[y][x] {
					Seat::Empty if adjacent_count == 0 => {
						new_data[y][x] = Seat::Occupied;
						changes += 1;
					}
					Seat::Occupied if adjacent_count >= required => {
						new_data[y][x] = Seat::Empty;
						changes += 1;
					}
					_ => (),
				}
			}
		}

		if changes == 0 {
			break;
		}

		for y in 0..height {
			for x in 0..width {
				data[y][x] = new_data[y][x];
			}
		}
	}

	count_occupied(&data)
}

fn count_occupied(data: &[Vec<Seat>]) -> i32 {
	let mut count = 0;

	for row in data {
		for seat in row {
			if let Seat::Occupied = seat {
				count += 1;
			}
		}
	}

	count
}

fn dir_has_seat_occupied(data: &[Vec<Seat>], start: (i32, i32), step: (i32, i32)) -> bool {
	let height = data.len() as i32;
	let width = data[0].len() as i32;

	let mut x = start.0;
	let mut y = start.1;

	loop {
		x += step.0;
		y += step.1;

		if x < 0 || y < 0 || x >= width || y >= height {
			break;
		}

		match data[y as usize][x as usize] {
			Seat::Occupied => return true,
			Seat::Empty => return false,
			_ => (),
		}
	}

	false
}
