pub fn run() {
	println!("=== Day 23 ===");

	let answer1 = part1();
	println!("Part1: {}", answer1);

	let answer2 = part2();
	println!("Part2: {}", answer2);
}

fn get_data() -> Vec<u32> {
	std::fs::read_to_string("data/day23.txt")
		.expect("Couldn't read data file")
		.chars()
		.map(|c| c.to_digit(10).unwrap())
		.collect()
}

fn index_of(data: &[u32], value: u32) -> Option<usize> {
	for (i, &v) in data.iter().enumerate() {
		if v == value {
			return Some(i);
		}
	}

	None
}

fn part1() -> String {
	let mut data = get_data();
	let max = *data.iter().max().unwrap();

	for _ in 0..100 {
		let mut destination = data[0] - 1;
		if destination == 0 {
			destination = max;
		}

		let mut dest_index;
		loop {
			dest_index = index_of(&data, destination).unwrap();

			if dest_index >= 1 && dest_index < 4 {
				if destination == 1 {
					destination = max;
				} else {
					destination -= 1;
				}
			} else {
				break;
			}
		}

		for _ in 0..3 {
			let v = data.remove(1);
			data.insert(dest_index, v);
		}

		let v = data.remove(0);
		data.push(v);
	}

	loop {
		let p1 = index_of(&data, 1).unwrap();

		if p1 == 0 {
			break;
		}

		let v = data.remove(0);
		data.push(v);
	}

	(&data[1..])
		.iter()
		.map(|&i| i.to_string())
		.collect::<Vec<_>>()
		.join("")
}

fn part2() -> i32 {
	0
}
