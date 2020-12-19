pub fn run() {
	println!("=== Day 10 ===");

	let answer1 = part1();
	println!("Part1: {}", answer1);

	let answer2 = part2();
	println!("Part2: {}", answer2);
}

fn get_data() -> Vec<i32> {
	let mut data = std::fs::read_to_string("data/day10.txt")
		.expect("Couldn't read data file")
		.split('\n')
		.map(|row| row.parse().expect("Not a number"))
		.collect::<Vec<_>>();
	data.push(0);
	data.sort_unstable();
	data.push(data.last().expect("Empty list") + 3);
	data
}

fn part1() -> i32 {
	let data = get_data();

	let mut one = 0;
	let mut three = 1;

	for i in 1..data.len() - 1 {
		if data[i] - data[i - 1] == 1 {
			one += 1;
		} else {
			three += 1;
		}
	}

	one * three
}

fn part2() -> i64 {
	0
}
