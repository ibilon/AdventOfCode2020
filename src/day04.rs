use std::collections::HashMap;

pub fn run() -> () {
	println!("=== Day 04 ===");

	let answer1 = part1();
	println!("Part1: {}", answer1);

	let answer2 = part2();
	println!("Part2: {}", answer2);
}

fn get_data() -> Vec<HashMap<String, String>> {
	let mut data = Vec::new();

	for line in std::fs::read_to_string("data/day04.txt")
		.expect("Couldn't read data file")
		.split("\n\n")
	{
		let mut passport = HashMap::new();

		for entry in line.split_ascii_whitespace() {
			let entry = entry.split_at(entry.find(':').expect("Malformed key/value pair"));
			passport.insert(String::from(entry.0), String::from(&entry.1[1..]));
		}

		data.push(passport);
	}

	data
}

fn part1() -> i32 {
	let data = get_data();
	let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
	let mut valid = 0;

	for passport in data {
		if required_fields
			.iter()
			.filter(|field| passport.contains_key(&field.to_string()))
			.count() == required_fields.len()
		{
			valid += 1;
		}
	}

	valid
}

fn part2() -> i32 {
	let data = get_data();
	let mut valid = 0;

	for passport in data.iter() {

		if let Some(byr) = passport.get("byr") {
			if let Ok(byr) = byr.parse::<i32>() {
				if byr < 1920 || byr > 2002 {
					continue;
				}
			} else {
				continue;
			}
		} else {
			continue;
		}

		if let Some(iyr) = passport.get("iyr") {
			if let Ok(iyr) = iyr.parse::<i32>() {
				if iyr < 2010 || iyr > 2020 {
					continue;
				}
			} else {
				continue;
			}
		} else {
			continue;
		}

		if let Some(eyr) = passport.get("eyr") {
			if let Ok(eyr) = eyr.parse::<i32>() {
				if eyr < 2020 || eyr > 2030 {
					continue;
				}
			} else {
				continue;
			}
		} else {
			continue;
		}

		if let Some(hgt) = passport.get("hgt") {
			let cm = hgt.ends_with("cm");

			if !cm && !hgt.ends_with("in") {
				continue;
			}

			if let Ok(hgt) = hgt[..hgt.len() - 2].parse::<i32>() {
				if (cm && (hgt < 150 || hgt > 193)) || (!cm && (hgt < 59 || hgt > 76)) {
					continue;
				}
			} else {
				continue;
			}
		} else {
			continue;
		}

		if let Some(hcl) = passport.get("hcl") {
			if let Some(c) = hcl.chars().next() {
				if c != '#' {
					continue;
				}
			} else {
				continue;
			}

			if hcl.len() != 7
				|| hcl
					.chars()
					.skip(1)
					.filter(|c| c.is_ascii_hexdigit())
					.count() != 6
			{
				continue;
			}
		} else {
			continue;
		}

		if let Some(ecl) = passport.get("ecl") {
			if ecl != "amb"
				&& ecl != "blu" && ecl != "brn"
				&& ecl != "gry" && ecl != "grn"
				&& ecl != "hzl" && ecl != "oth"
			{
				continue;
			}
		} else {
			continue;
		}

		if let Some(pid) = passport.get("pid") {
			if pid.len() != 9 || pid.chars().filter(|c| c.is_ascii_digit()).count() != 9 {
				continue;
			}
		} else {
			continue;
		}
		
		valid += 1;
	}

	valid
}
