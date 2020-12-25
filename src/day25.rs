pub fn run() {
	println!("=== Day 25 ===");

	let answer1 = part1();
	println!("Part1: {}", answer1);

	let answer2 = part2();
	println!("Part2: {}", answer2);
}

fn get_data() -> Vec<i64> {
	std::fs::read_to_string("data/day25.txt")
		.expect("Couldn't read data file")
		.split('\n')
		.map(|line| line.parse().unwrap())
		.collect()
}

fn transform(subject: i64, loop_size: i64) -> i64 {
	let mut a = 1;

	for _ in 0..loop_size {
		a *= subject;
		a %= 20201227;
	}

	a
}

fn part1() -> i64 {
	let keys = get_data();
	let loop_sizes = keys
		.iter()
		.map(|key| {
			let mut a = 1;
			let mut s = 0;

			loop {
				s += 1;
				a *= 7;
				a %= 20201227;

				if a == *key {
					break;
				}
			}

			s
		})
		.collect::<Vec<_>>();

	let handshake0 = transform(keys[0], loop_sizes[1]);
	let handshake1 = transform(keys[1], loop_sizes[0]);
	assert!(handshake0 == handshake1);

	handshake0
}

fn part2() -> i32 {
	0
}
