pub fn run() -> () {
	println!("=== Day 18 ===");

	let answer1 = part1();
	println!("Part1: {}", answer1);

	let answer2 = part2();
	println!("Part2: {}", answer2);
}

struct Parser {
	chars: Vec<char>,
	index: usize,
	precedence: bool,
}

impl Parser {
	fn parse_expression(&mut self) -> Expr {
		let mut expression = self.parse_sub_expression();

		while !self.end_of_file() {
			match self.peek() {
				'+' if !self.precedence => {
					self.consume();
					expression =
						Expr::Addition(Box::new(expression), Box::new(self.parse_sub_expression()))
				}
				'*' => {
					self.consume();
					expression = Expr::Multiplication(
						Box::new(expression),
						Box::new(self.parse_sub_expression()),
					)
				}
				_ => break,
			}
		}

		expression
	}

	fn parse_sub_expression(&mut self) -> Expr {
		if self.precedence {
			self.parse_addition()
		} else {
			self.parse_primary()
		}
	}

	fn parse_addition(&mut self) -> Expr {
		let mut expression = self.parse_primary();

		while !self.end_of_file() {
			match self.peek() {
				'+' => {
					self.consume();
					expression =
						Expr::Addition(Box::new(expression), Box::new(self.parse_primary()))
				}
				_ => break,
			}
		}

		expression
	}

	fn parse_primary(&mut self) -> Expr {
		if self.consume_if('(') {
			let expression = self.parse_expression();
			self.require(')');
			expression
		} else {
			Expr::Number(self.consume().to_digit(10).expect("Not a number"))
		}
	}

	fn consume(&mut self) -> char {
		let c = self.chars[self.index];
		self.index += 1;
		c
	}

	fn consume_if(&mut self, c: char) -> bool {
		if self.peek() == c {
			self.consume();
			true
		} else {
			false
		}
	}

	fn end_of_file(&self) -> bool {
		self.index >= self.chars.len()
	}

	fn peek(&self) -> char {
		self.chars[self.index]
	}

	fn require(&mut self, c: char) -> () {
		if self.peek() != c {
			panic!(format!("Expected {} but found {}", c, self.peek()));
		}

		self.consume();
	}
}

enum Expr {
	Number(u32),
	Addition(Box<Expr>, Box<Expr>),
	Multiplication(Box<Expr>, Box<Expr>),
}

fn get_data(precedence: bool) -> Vec<Expr> {
	std::fs::read_to_string("data/day18.txt")
		.expect("Couldn't read data file")
		.split("\n")
		.map(|row| {
			Parser {
				chars: row.chars().filter(|c| !c.is_ascii_whitespace()).collect(),
				index: 0,
				precedence,
			}
			.parse_expression()
		})
		.collect()
}

fn eval(expr: &Expr) -> u64 {
	match expr {
		Expr::Number(number) => *number as u64,
		Expr::Addition(e1, e2) => eval(e1) + eval(e2),
		Expr::Multiplication(e1, e2) => eval(e1) * eval(e2),
	}
}

fn part1() -> u64 {
	get_data(false)
		.iter()
		.map(|expr| eval(expr))
		.fold(0, |acc, x| acc + x)
}

fn part2() -> u64 {
	get_data(true)
		.iter()
		.map(|expr| eval(expr))
		.fold(0, |acc, x| acc + x)
}
