use std::collections::HashMap;

pub fn run() {
	println!("=== Day 17 ===");

	let answer1 = part1();
	println!("Part1: {}", answer1);

	let answer2 = part2();
	println!("Part2: {}", answer2);
}

struct EnergySource {
	pub cubes: HashMap<(i32, i32, i32, i32), bool>,
	pub range_x: (i32, i32),
	pub range_y: (i32, i32),
}

fn get_data() -> EnergySource {
	let mut cubes = HashMap::new();
	let mut max_x = 0;
	let mut max_y = 0;

	for (y, row) in std::fs::read_to_string("data/day17.txt")
		.expect("Couldn't read data file")
		.split('\n')
		.enumerate()
	{
		max_y = y as i32;

		for (x, value) in row.chars().enumerate() {
			max_x = x as i32;
			cubes.insert((x as i32, y as i32, 0, 0), value == '#');
		}
	}

	EnergySource {
		cubes,
		range_x: (0, max_x + 1),
		range_y: (0, max_y + 1),
	}
}

fn print_layer(data: &[Vec<bool>]) {
	for row in data.iter() {
		for column in row.iter() {
			print!("{}", if *column { '#' } else { '.' });
		}
		println!();
	}
}

fn part1() -> i32 {
	let mut data = get_data();

	for i in 0..6 {
		let mut new_data = HashMap::new();

		for x in data.range_x.0 - 1 - i..data.range_x.1 + 1 + i {
			for y in data.range_y.0 - 1 - i..data.range_y.1 + 1 + i {
				for z in -1 - i..2 + i {
					let mut neighbors = 0;

					for xx in -1..2 {
						for yy in -1..2 {
							for zz in -1..2 {
								if xx == 0 && yy == 0 && zz == 0 {
									continue;
								}

								if *data
									.cubes
									.entry((x + xx, y + yy, z + zz, 0))
									.or_insert(false)
								{
									neighbors += 1;
								}
							}
						}
					}

					let cube = *data.cubes.entry((x, y, z, 0)).or_insert(false);

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
			data.cubes.insert(key, value);
		}
	}

	let mut count = 0;

	for (_, value) in data.cubes {
		if value {
			count += 1;
		}
	}

	count
}

fn part2() -> i32 {
	let mut data = get_data();

	for i in 0..6 {
		let mut new_data = HashMap::new();

		for x in data.range_x.0 - 1 - i..data.range_x.1 + 1 + i {
			for y in data.range_y.0 - 1 - i..data.range_y.1 + 1 + i {
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
											.cubes
											.entry((x + xx, y + yy, z + zz, w + ww))
											.or_insert(false)
										{
											neighbors += 1;
										}
									}
								}
							}
						}

						let cube = *data.cubes.entry((x, y, z, w)).or_insert(false);

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
			data.cubes.insert(key, value);
		}
	}

	let mut count = 0;

	for (_, value) in data.cubes {
		if value {
			count += 1;
		}
	}

	count
}
