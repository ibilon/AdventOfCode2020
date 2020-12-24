use std::collections::HashMap;

enum Dir {
	East,
	SouthEast,
	SouthWest,
	West,
	NorthWest,
	NorthEast,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Tile {
	White,
	Black,
}

fn get_data() -> Vec<Vec<Dir>> {
	std::fs::read_to_string("data/day24.txt")
		.expect("Couldn't read data file")
		.split('\n')
		.map(|line| {
			let mut chars = line.chars();
			let mut dirs = Vec::new();

			while let Some(c) = chars.next() {
				match c {
					'e' => dirs.push(Dir::East),
					'w' => dirs.push(Dir::West),
					's' => match chars.next().unwrap() {
						'e' => dirs.push(Dir::SouthEast),
						'w' => dirs.push(Dir::SouthWest),
						cc => panic!("Unknown char {}", cc),
					},
					'n' => match chars.next().unwrap() {
						'e' => dirs.push(Dir::NorthEast),
						'w' => dirs.push(Dir::NorthWest),
						cc => panic!("Unknown char {}", cc),
					},
					cc => panic!("Unknown char {}", cc),
				}
			}

			dirs
		})
		.collect()
}

pub fn run() {
	println!("=== Day 24 ===");

	let mut tiles = HashMap::new();

	for tile in get_data() {
		let mut x = 0;
		let mut y = 0;

		for dir in tile {
			match dir {
				Dir::East => x += 2,
				Dir::SouthEast => {
					x += 1;
					y += 1;
				}
				Dir::SouthWest => {
					x -= 1;
					y += 1;
				}
				Dir::West => x -= 2,
				Dir::NorthWest => {
					x -= 1;
					y -= 1;
				}
				Dir::NorthEast => {
					x += 1;
					y -= 1;
				}
			}
		}

		let tile = tiles.entry((x, y)).or_insert(Tile::White);
		*tile = match *tile {
			Tile::White => Tile::Black,
			Tile::Black => Tile::White,
		};
	}

	let count_black_tiles = |tiles: &HashMap<(i32, i32), Tile>| {
		tiles
			.iter()
			.filter(|(_, tile)| **tile == Tile::Black)
			.count()
	};

	println!("Part1: {}", count_black_tiles(&tiles));

	let offsets = [(2, 0), (1, 1), (-1, 1), (-2, 0), (-1, -1), (1, -1)];

	for _ in 0..100 {
		let mut expanded_tiles = Vec::new();

		for ((x, y), _) in tiles.iter() {
			for (dx, dy) in offsets.iter() {
				let p = (*x + *dx, *y + *dy);

				if tiles.contains_key(&p) {
					expanded_tiles.push(p);
				}
			}
		}

		for p in expanded_tiles {
			tiles.insert(p, Tile::White);
		}

		let mut new_tiles = HashMap::new();

		for ((x, y), &tile) in tiles.iter() {
			let neighbors = offsets
				.iter()
				.map(|(dx, dy)| tiles.get(&(*x + *dx, *y + *dy)).unwrap_or(&Tile::White))
				.filter(|tile| **tile == Tile::Black)
				.count();

			new_tiles.insert(
				(*x, *y),
				if tile == Tile::Black && (neighbors == 0 || neighbors > 2) {
					Tile::White
				} else if tile == Tile::White && neighbors == 2 {
					Tile::Black
				} else {
					tile
				},
			);
		}

		for ((x, y), value) in new_tiles {
			tiles.insert((x, y), value);
		}
	}

	println!("Part2: {}", count_black_tiles(&tiles));
}
