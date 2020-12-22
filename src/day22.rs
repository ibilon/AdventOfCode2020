use std::cmp::Ordering;

pub fn run() {
	println!("=== Day 22 ===");

	let answer1 = part1();
	println!("Part1: {}", answer1);

	let answer2 = part2();
	println!("Part2: {}", answer2);
}

fn get_data() -> (Vec<i32>, Vec<i32>) {
	let file = std::fs::read_to_string("data/day22.txt").expect("Couldn't read data file");
	let mut data = file.split("\n\n");

	let mut player = || {
		data.next()
			.unwrap()
			.split('\n')
			.skip(1)
			.map(|i| i.parse().unwrap())
			.collect()
	};

	(player(), player())
}

fn part1() -> usize {
	let (mut player1, mut player2) = get_data();

	while !player1.is_empty() && !player2.is_empty() {
		let c1 = player1.remove(0);
		let c2 = player2.remove(0);

		match c1.cmp(&c2) {
			Ordering::Greater => {
				player1.push(c1);
				player1.push(c2);
			}
			Ordering::Less => {
				player2.push(c2);
				player2.push(c1);
			}
			Ordering::Equal => panic!("draw"),
		}
	}

	if player1.is_empty() { player2 } else { player1 }
		.iter()
		.rev()
		.enumerate()
		.map(|(i, &v)| (i + 1) * v as usize)
		.sum()
}

fn part2() -> i32 {
	0
}
