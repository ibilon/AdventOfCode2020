pub fn run() -> () {
	println!("=== Day 02 ===");

	let answer1 = part1();
	println!("Part1: {}", answer1);

	let answer2 = part2();
	println!("Part2: {}", answer2);
}

struct Data {
	start: usize,
	end: usize,
	value: char,
	chars: Vec<char>,
}

fn get_data() -> Vec<Data> {
	std::fs::read_to_string("data/day02.txt")
		.expect("Couldn't read data file")
		.split("\n")
		.map(|e| {
			let c = e.split(" ").collect::<Vec<&str>>();
			let range = c[0]
				.split("-")
				.map(|e| e.parse::<usize>().expect("Not a number"))
				.collect::<Vec<usize>>();

			Data {
				start: range[0],
				end: range[1],
				value: c[1].chars().next().expect("Empty string"),
				chars: c[2].chars().collect(),
			}
		})
		.collect()
}

fn part1() -> i32 {
	let data = get_data();
	let mut valid = 0;

	for password in data {
		let count = password
			.chars
			.iter()
			.filter(|c| **c == password.value)
			.count();
		if count >= password.start && count <= password.end {
			valid += 1;
		}
	}

	valid
}

fn part2() -> i32 {
	let data = get_data();
	let mut valid = 0;

	for password in data {
		let c1 = password.chars[password.start - 1];
		let c2 = password.chars[password.end - 1];
		let c = password.value;

		if (c1 == c && c2 != c) || (c1 != c && c2 == c) {
			valid += 1;
		}
	}

	valid
}
