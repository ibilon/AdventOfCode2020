pub fn run() {
	println!("=== Day 08 ===");

	let answer1 = part1();
	println!("Part1: {}", answer1);

	let answer2 = part2().expect("Couldn't compute part 2");
	println!("Part2: {}", answer2);
}

#[derive(Copy, Clone)]
enum InstructionType {
	Acc,
	Jmp,
	Nop,
}

fn get_data() -> Vec<(InstructionType, i32)> {
	std::fs::read_to_string("data/day08.txt")
		.expect("Couldn't read data file")
		.split('\n')
		.map(|row| {
			let row = row.split(' ').collect::<Vec<&str>>();
			(
				match row[0] {
					"nop" => InstructionType::Nop,
					"jmp" => InstructionType::Jmp,
					"acc" => InstructionType::Acc,
					_ => panic!("Unknown instruction"),
				},
				row[1].parse().expect("Invalid number"),
			)
		})
		.collect()
}

fn eval(data: &[(InstructionType, i32)]) -> Result<i32, i32> {
	let mut data = data
		.iter()
		.map(|i| (false, i.0, i.1))
		.collect::<Vec<(bool, InstructionType, i32)>>();
	let mut acc = 0;
	let mut ip: i32 = 0;

	loop {
		if ip as usize == data.len() {
			return Ok(acc);
		}

		let i = &mut data[ip as usize];

		if i.0 {
			return Err(acc);
		}

		i.0 = true;

		match i.1 {
			InstructionType::Acc => {
				acc += i.2;
				ip += 1;
			}
			InstructionType::Jmp => ip += i.2,
			InstructionType::Nop => ip += 1,
		}
	}
}

fn part1() -> i32 {
	let data = get_data();
	eval(&data).expect_err("Should have looped")
}

fn part2() -> Result<i32, ()> {
	let mut data = get_data();
	let mut i = 0;

	while i < data.len() {
		match data[i].0 {
			InstructionType::Nop if data[i].1 != 0 => {
				data[i].0 = InstructionType::Jmp;

				if let Ok(result) = eval(&data) {
					return Ok(result);
				}

				data[i].0 = InstructionType::Nop;
			}

			InstructionType::Jmp => {
				data[i].0 = InstructionType::Nop;

				if let Ok(result) = eval(&data) {
					return Ok(result);
				}

				data[i].0 = InstructionType::Jmp;
			}

			_ => (),
		}

		i += 1;
	}

	Err(())
}
