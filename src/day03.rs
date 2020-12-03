pub fn run() -> () {
	println!("=== Day 03 ===");

	let answer1 = part1();
	println!("Part1: {}", answer1);

	let answer2 = part2();
	println!("Part2: {}", answer2);
}

fn get_data() -> Vec<Vec<bool>> {
	std::fs::read_to_string("data/day03.txt")
		.expect("Couldn't read data file")
		.split("\n")
		.map(|row| row.chars().map(|c| c == '#').collect())
		.collect()
}

fn count_trees(right: usize, down: usize) -> i32 {
	let data = get_data();
	let width = data[0].len();
	let mut count = 0;
	let mut col = 0;

	for row in (0..data.len()).step_by(down) {
		if data[row][col % width] {
			count += 1;
		}

		col += right;
	}

	count
}

fn part1() -> i32 {
	count_trees(3, 1)
}

fn part2() -> i64 {
	let a = count_trees(1, 1) as i64;
	let b = count_trees(3, 1) as i64;
	let c = count_trees(5, 1) as i64;
	let d = count_trees(7, 1) as i64;
	let e = count_trees(1, 2) as i64;

	a * b * c * d * e
}
