use std::collections::HashMap;

static SHINY_GOLD: &str = "shiny gold";

pub fn run() -> () {
	println!("=== Day 07 ===");

	let answer1 = part1();
	println!("Part1: {}", answer1);

	let answer2 = part2();
	println!("Part2: {}", answer2);
}

fn get_data() -> HashMap<String, Vec<(i32, String)>> {
	let mut data = HashMap::new();

	for row in std::fs::read_to_string("data/day07.txt")
		.expect("Couldn't read data file")
		.split("\n")
	{
		let row = row.split(" contain ").collect::<Vec<&str>>();
		let key = String::from(&row[0][..row[0].len() - 5]);

		if row[1] == "no other bags." {
			data.insert(key, Vec::new());
		} else {
			let value = row[1][..row[1].len() - 1]
				.split(", ")
				.map(|s| {
					let mut s = s.split(" ").collect::<Vec<&str>>();
					let count = s.drain(0..1).collect::<Vec<&str>>()[0]
						.parse::<i32>()
						.expect("Wrong number");
					s.pop();
					(count, s.join(" "))
				})
				.collect();
			data.insert(key, value);
		}
	}

	data
}

fn contains_shiny_gold(data: &HashMap<String, Vec<(i32, String)>>, color: &String) -> bool {
	let sub = data.get(color).expect("Wrong color");

	if sub.iter().find(|(_, color)| *color == SHINY_GOLD).is_some() {
		return true;
	} else {
		for (_, sub_color) in sub {
			if contains_shiny_gold(data, sub_color) {
				return true;
			}
		}
	}

	false
}

fn part1() -> usize {
	let data = get_data();
	data.iter()
		.filter(|(color, _)| contains_shiny_gold(&data, color))
		.count()
}

fn count_bags(data: &HashMap<String, Vec<(i32, String)>>, color: &str) -> i32 {
	data.get(color)
		.expect("Wrong color")
		.iter()
		.fold(1, |acc, (sub_count, sub_color)| {
			acc + sub_count * count_bags(data, sub_color)
		})
}

fn part2() -> i32 {
	let data = get_data();
	count_bags(&data, SHINY_GOLD) - 1
}
