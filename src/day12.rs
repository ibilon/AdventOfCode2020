pub fn run() {
	println!("=== Day 12 ===");

	let answer1 = part1();
	println!("Part1: {}", answer1);

	let answer2 = part2();
	println!("Part2: {}", answer2);
}

#[derive(Debug)]
enum Action {
	North,
	South,
	East,
	West,
	Left,
	Right,
	Forward,
}

fn get_data() -> Vec<(Action, i32)> {
	std::fs::read_to_string("data/day12.txt")
		.expect("Couldn't read data file")
		.split('\n')
		.map(|row| {
			let value = row
				.chars()
				.skip(1)
				.collect::<String>()
				.parse::<i32>()
				.unwrap();

			(
				match row.chars().next().unwrap() {
					'N' => Action::North,
					'S' => Action::South,
					'E' => Action::East,
					'W' => Action::West,
					'L' => Action::Left,
					'R' => Action::Right,
					'F' => Action::Forward,
					a => panic!(format!("Unknown action: {}", a)),
				},
				value,
			)
		})
		.collect::<Vec<_>>()
}

fn part1() -> i32 {
	let mut dir = 90;
	let mut east = 0;
	let mut north = 0;

	for action in get_data() {
		let value = action.1;
		let mut action = action.0;

		if let Action::Forward = action {
			action = match dir {
				0 => Action::North,
				90 => Action::East,
				180 => Action::South,
				270 => Action::West,
				_ => panic!(format!("Unknown dir: {}", dir)),
			}
		}

		match action {
			Action::North => north += value,
			Action::South => north -= value,
			Action::East => east += value,
			Action::West => east -= value,
			Action::Left => dir = (dir - value + 360) % 360,
			Action::Right => dir = (dir + value) % 360,
			_ => (),
		}
	}

	east.abs() + north.abs()
}

fn part2() -> i32 {
	let mut ship_east = 0;
	let mut ship_north = 0;
	let mut waypoint_east = 10;
	let mut waypoint_north = 1;

	for action in get_data() {
		let mut value = action.1;
		let mut action = action.0;

		if let Action::Left = action {
			action = Action::Right;
			value = 360 - value;
		}

		match action {
			Action::North => waypoint_north += value,
			Action::South => waypoint_north -= value,
			Action::East => waypoint_east += value,
			Action::West => waypoint_east -= value,
			Action::Right => {
				for _ in 0..value / 90 {
					let tmp = waypoint_east;
					waypoint_east = waypoint_north;
					waypoint_north = -tmp;
				}
			}
			Action::Forward => {
				ship_east += waypoint_east * value;
				ship_north += waypoint_north * value;
			}
			_ => panic!(format!("Unhandled {:?} {}", action, value)),
		}
	}

	ship_east.abs() + ship_north.abs()
}
