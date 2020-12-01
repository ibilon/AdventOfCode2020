pub fn run() -> () {
	println!("=== Day 01 ===");

	let answer1 = part1().expect("Couldn't compute part 1");
	println!("Part1: {}", answer1);

	let answer2 = part2().expect("Couldn't compute part 2");
	println!("Part2: {}", answer2);
}

fn get_data() -> Vec<i32> {
	std::fs::read_to_string("data/day01.txt")
		.expect("Couldn't read data file")
		.split("\n")
		.map(|e| e.parse::<i32>().expect("Not a number"))
		.collect()
}

fn part1() -> Result<i32, ()> {
	let data = get_data();

	for i in data.iter() {
		for j in data.iter() {
			if i + j == 2020 {
				return Ok(i * j);
			}
		}
	}

	Err(())
}

fn part2() -> Result<i32, ()> {
	let data = get_data();

	for i in data.iter() {
		for j in data.iter() {
			for k in data.iter() {
				if i + j + k == 2020 {
					return Ok(i * j * k);
				}
			}
		}
	}

	Err(())
}
