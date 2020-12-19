pub fn run() {
	println!("=== Day 16 ===");

	let answer1 = part1();
	println!("Part1: {}", answer1);

	let answer2 = part2();
	println!("Part2: {}", answer2);
}

struct Field {
	pub name: String,
	pub range1: (i32, i32),
	pub range2: (i32, i32),
}

struct Data {
	pub fields: Vec<Field>,
	pub ticket: Vec<i32>,
	pub nearby_tickets: Vec<Vec<i32>>,
}

fn get_data() -> Data {
	let data = std::fs::read_to_string("data/day16.txt")
		.expect("Couldn't read data file")
		.split("\n\n")
		.map(String::from)
		.collect::<Vec<_>>();

	let fields = data[0]
		.split('\n')
		.map(|row| {
			let p0 = row.find(':').expect("Wrong format");
			let p1 = row.find(" or").expect("Wrong format");
			let p2 = row.find('-').expect("Wrong format");
			let s = &row[p1 + 4..];
			let p3 = s.find('-').expect("Wrong format");

			Field {
				name: String::from(&row[..p0]),
				range1: (
					(&row[p0 + 2..p2]).parse().expect("Not a number"),
					(&row[p2 + 1..p1]).parse().expect("Not a number"),
				),
				range2: (
					(&s[..p3]).parse().expect("Not a number"),
					(&s[p3 + 1..]).parse().expect("Not a number"),
				),
			}
		})
		.collect();

	let p = data[1].find(":\n").expect("Wrong format");
	let ticket = (&data[1][p + 2..])
		.split(',')
		.map(|elem| elem.parse().expect("Not a number"))
		.collect();

	let nearby_tickets = data[2]
		.split('\n')
		.skip(1)
		.map(|row| {
			row.split(',')
				.map(|elem| elem.parse().expect("Not a number"))
				.collect()
		})
		.collect();

	Data {
		fields,
		ticket,
		nearby_tickets,
	}
}

fn part1() -> i32 {
	let data = get_data();
	let mut error_rate = 0;

	for ticket in data.nearby_tickets.iter() {
		for value in ticket.iter() {
			let mut valid = false;

			for field in data.fields.iter() {
				if (*value >= field.range1.0 && *value <= field.range1.1)
					|| (*value >= field.range2.0 && *value <= field.range2.1)
				{
					valid = true;
					break;
				}
			}

			if !valid {
				error_rate += value;
			}
		}
	}

	error_rate
}

fn remove_vec(data: &mut Vec<usize>, value: usize) -> bool {
	let mut index = -1;
	for (i, v) in data.iter().enumerate() {
		if *v == value {
			index = i as i32;
			break;
		}
	}

	if index != -1 {
		data.remove(index as usize);
		true
	} else {
		false
	}
}

fn part2() -> i64 {
	let data = get_data();

	let mut possibilities = data
		.fields
		.iter()
		.map(|_| (0..data.fields.len()).collect::<Vec<_>>())
		.collect::<Vec<_>>();

	for ticket in data.nearby_tickets.iter() {
		let mut valid_ticket = true;

		for value in ticket.iter() {
			let mut valid = false;

			for field in data.fields.iter() {
				if (*value >= field.range1.0 && *value <= field.range1.1)
					|| (*value >= field.range2.0 && *value <= field.range2.1)
				{
					valid = true;
					break;
				}
			}

			if !valid {
				valid_ticket = false;
				break;
			}
		}

		if valid_ticket {
			for (i, value) in ticket.iter().enumerate() {
				for (j, field) in data.fields.iter().enumerate() {
					if !((*value >= field.range1.0 && *value <= field.range1.1)
						|| (*value >= field.range2.0 && *value <= field.range2.1))
					{
						remove_vec(&mut possibilities[i], j);

						loop {
							let mut did_remove = false;

							for n in 0..possibilities.len() {
								if possibilities[n].len() == 1 {
									let value = possibilities[n][0];
									for (p, pp) in possibilities.iter_mut().enumerate() {
										if p == n {
											continue;
										}

										if remove_vec(pp, value) {
											did_remove = true;
										}
									}
								}
							}

							if !did_remove {
								break;
							}
						}
					}
				}
			}
		}
	}

	let mut count = 1i64;

	for (i, p) in possibilities.iter().enumerate() {
		if p[0] < 6 {
			count *= data.ticket[i] as i64;
		}
	}

	count
}
