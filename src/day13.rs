pub fn run() {
	println!("=== Day 13 ===");

	let answer1 = part1();
	println!("Part1: {}", answer1);

	let answer2 = part2();
	println!("Part2: {}", answer2);
}

fn get_data() -> (i32, Vec<(usize, i32)>) {
	let data = std::fs::read_to_string("data/day13.txt")
		.expect("Couldn't read data file")
		.split('\n')
		.map(String::from)
		.collect::<Vec<_>>();

	(
		data[0].parse().expect("Not a number"),
		data[1]
			.split(',')
			.enumerate()
			.filter(|e| e.1 != "x")
			.map(|e| (e.0, e.1.parse().expect("Not a number")))
			.collect(),
	)
}

fn part1() -> i32 {
	let data = get_data();
	let mut timestamp = data.0;

	loop {
		for bus in data.1.iter() {
			if timestamp % bus.1 == 0 {
				return (timestamp - data.0) * bus.1;
			}
		}

		timestamp += 1;
	}
}

fn part2() -> usize {
	let data = get_data();

	println!(
		"https://www.wolframalpha.com/input/?i={}",
		data.1
			.iter()
			.map(|e| format!("t%3D{}n{}-{}", e.1, e.0, e.0))
			.collect::<Vec<_>>()
			.join("+and+")
	);

	let mut n = 0;

	loop {
		// Integer solution of t computed by wolfram alpha:
		// https://www.wolframalpha.com/input/?i=t%3D23n0-0+and+t%3D41n13-13+and+t%3D37n17-17+and+t%3D421n23-23+and+t%3D17n40-40+and+t%3D19n42-42+and+t%3D29n52-52+and+t%3D487n54-54+and+t%3D13n67-67
		let t = 667437230788118 + 871100667227947 * n;
		let mut valid = true;

		for bus in data.1.iter() {
			if (t + bus.0) % (bus.1 as usize) != 0 {
				valid = false;
				break;
			}
		}

		if valid {
			return t;
		}

		n += 1;
	}
}
