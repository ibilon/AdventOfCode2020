use std::collections::HashMap;

pub fn run() -> () {
	println!("=== Day 17 ===");

	let answer1 = part1();
	println!("Part1: {}", answer1);

	let answer2 = part2();
	println!("Part2: {}", answer2);
}

fn get_data() -> (HashMap<(i32, i32, i32, i32), bool>, (i32, i32), (i32, i32)) {
	let mut data = HashMap::new();
	let mut max_x = 0;
	let mut max_y = 0;

	for (y, row) in std::fs::read_to_string("data/day17.txt")
		.expect("Couldn't read data file")
		.split("\n")
		.enumerate()
	{
		max_y = y as i32;

		for (x, value) in row.chars().enumerate() {
			max_x = x as i32;
			data.insert((x as i32, y as i32, 0, 0), value == '#');
		}
	}

	(data, (0, max_x + 1), (0, max_y + 1))
}

fn print_layer(data: &Vec<Vec<bool>>) -> () {
	for row in data.iter() {
		for column in row.iter() {
			print!("{}", if *column { '#' } else { '.' });
		}
		println!("");
	}
}

fn part1() -> i32 {
	let (mut data, range_x, range_y) = get_data();

	for i in 0..6 {
		let mut new_data = HashMap::new();

		for x in range_x.0 - 1 - i..range_x.1 + 1 + i {
			for y in range_y.0 - 1 - i..range_y.1 + 1 + i {
				for z in -1 - i..2 + i {
					let mut neighbors = 0;

					for xx in -1..2 {
						for yy in -1..2 {
							for zz in -1..2 {
								if xx == 0 && yy == 0 && zz == 0 {
									continue;
								}

								if *data.entry((x + xx, y + yy, z + zz, 0)).or_insert(false) {
									neighbors += 1;
								}
							}
						}
					}

					let cube = *data.entry((x, y, z, 0)).or_insert(false);

					new_data.insert(
						(x, y, z, 0),
						if cube && neighbors != 2 && neighbors != 3 {
							false
						} else if !cube && neighbors == 3 {
							true
						} else {
							cube
						},
					);
				}
			}
		}

		for (key, value) in new_data {
			data.insert(key, value);
		}
	}

	let mut count = 0;

	for (_, value) in data {
		if value {
			count += 1;
		}
	}

	count
}

fn part2() -> i32 {
	let (mut data, range_x, range_y) = get_data();

	for i in 0..6 {
		let mut new_data = HashMap::new();

		for x in range_x.0 - 1 - i..range_x.1 + 1 + i {
			for y in range_y.0 - 1 - i..range_y.1 + 1 + i {
				for z in -1 - i..2 + i {
					for w in -1 - i..2 + i {
						let mut neighbors = 0;

						for xx in -1..2 {
							for yy in -1..2 {
								for zz in -1..2 {
									for ww in -1..2 {
										if xx == 0 && yy == 0 && zz == 0 && ww == 0 {
											continue;
										}

										if *data
											.entry((x + xx, y + yy, z + zz, w + ww))
											.or_insert(false)
										{
											neighbors += 1;
										}
									}
								}
							}
						}

						let cube = *data.entry((x, y, z, w)).or_insert(false);

						new_data.insert(
							(x, y, z, w),
							if cube && neighbors != 2 && neighbors != 3 {
								false
							} else if !cube && neighbors == 3 {
								true
							} else {
								cube
							},
						);
					}
				}
			}
		}

		for (key, value) in new_data {
			data.insert(key, value);
		}
	}

	let mut count = 0;

	for (_, value) in data {
		if value {
			count += 1;
		}
	}

	count
}
