//! Day 11.

use std::{
	collections::VecDeque,
	ops::{Deref, DerefMut},
};

use nom::{
	branch::alt,
	bytes::complete::tag_no_case,
	character::complete::{char as nom_char, digit1, multispace0, multispace1},
	combinator::{map, map_res},
	error::{convert_error, VerboseError},
	multi::{many1, separated_list1},
	sequence::{delimited, terminated, tuple},
	Finish,
};

/// Number type for items.
type Number = u64;

/// Results of parsing.
type MyIResult<'a, O> = nom::IResult<&'a str, O, VerboseError<&'a str>>;

fn input() -> &'static str {
	include_str!("day_11.txt")
}

pub fn run() {
	let mut monkeys1 = parse_monkeys(input());
	let mut monkeys2 = monkeys1.clone();

	println!("Part 1:");
	for _ in 0..20 {
		monkeys1.run_round(3);
	}
	let mut handled: Vec<usize> = monkeys1.iter().map(|monkey| monkey.handled).collect();
	handled.sort();
	handled.reverse();
	let business = handled[0] * handled[1];
	println!("Monkey business: {business}");
	println!();

	println!("Part 2:");
	for _ in 0..10_000 {
		monkeys2.run_round(1);
	}
	let mut handled: Vec<usize> = monkeys2.iter().map(|monkey| monkey.handled).collect();
	handled.sort();
	handled.reverse();
	let business = handled[0] * handled[1];
	println!("Monkey business: {business}");
}

/// The monkeys in this AoC.
#[derive(Debug, Clone)]
struct Monkeys(Vec<Monkey>);

impl Monkeys {
	/// Play a round of monkey business. `div_by` is the divisor lowering the
	/// worry level.
	pub fn run_round(&mut self, div_by: Number) {
		let common_ground = self.common_ground();
		for i in 0..self.len() {
			while let Some(item) = self[i].items.pop_front() {
				self[i].handled += 1;
				let worry_level = self[i].operation.compute(item) / div_by % common_ground;
				let pass_to = if worry_level % self[i].test_divisible_by == 0 {
					self[i].if_true
				} else {
					self[i].if_false
				};
				self[pass_to].items.push_back(worry_level);
			}
		}
	}

	/// Calculate the common factor of the test divisors to keep the numbers
	/// down.
	fn common_ground(&self) -> Number {
		self.iter().map(|monkey| monkey.test_divisible_by).product()
	}
}

impl Deref for Monkeys {
	type Target = Vec<Monkey>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Monkeys {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

/// Information about a monkey.
#[derive(Debug, Clone)]
struct Monkey {
	/// ID.
	id: usize,
	/// Items the monkey has (number is worry-level).
	items: VecDeque<Number>,
	/// Operation to update.
	operation: Operation,
	/// Test if divisible by this number.
	test_divisible_by: Number,
	/// If test true, throw to this monkey.
	if_true: usize,
	/// If test false, throw to this monkey.
	if_false: usize,
	/// Store of how many items where handled.
	handled: usize,
}

/// Operation to update worry level. New = value operator value.
#[derive(Debug, Clone, Copy)]
struct Operation {
	a: Value,
	operator: Operator,
	b: Value,
}

impl Operation {
	/// Calculate the result of this operation, given the old value. Return the
	/// new value.
	pub fn compute(&self, old: Number) -> Number {
		let a = match self.a {
			Value::Old => old,
			Value::Number(num) => num,
		};

		let b = match self.b {
			Value::Old => old,
			Value::Number(num) => num,
		};

		match self.operator {
			Operator::Add => a + b,
			Operator::Multiply => a * b,
		}
	}
}

/// A value in the operation.
#[derive(Debug, Clone, Copy)]
enum Value {
	Old,
	Number(Number),
}

/// Operator in the operation.
#[derive(Debug, Clone, Copy)]
enum Operator {
	Add,
	Multiply,
}

/// Parse the AoC monkeys input.
/// Panics on failure.
fn parse_monkeys(input: &str) -> Monkeys {
	let (_rest, monkeys) = nom_monkeys(input)
		.finish()
		.map_err(|err| convert_error(input, err))
		.map_err(|err| {
			eprintln!("{err}");
			err
		})
		.expect("parsing");

	if monkeys.iter().enumerate().any(|(i, monkey)| i != monkey.id) {
		panic!("One of the monkey IDs does not match its position.");
	}

	monkeys
}

fn nom_monkeys(input: &str) -> MyIResult<Monkeys> {
	map(many1(delimited(multispace0, nom_monkey, multispace0)), Monkeys)(input)
}

fn nom_monkey(input: &str) -> MyIResult<Monkey> {
	let (input, _) = tag_no_case("Monkey ")(input)?;
	let (input, num) =
		terminated(map_res(digit1, |digit: &str| digit.parse()), nom_char(':'))(input)?;
	let (input, _) = multispace1(input)?;

	let (input, _) = tag_no_case("Starting items: ")(input)?;
	let (input, items) = separated_list1(tuple((nom_char(','), multispace0)), nom_number)(input)?;
	let (input, _) = multispace1(input)?;

	let (input, operation) = nom_operation(input)?;
	let (input, _) = multispace1(input)?;

	let (input, _) = tag_no_case("Test: divisible by ")(input)?;
	let (input, test_divisible_by) = nom_number(input)?;
	let (input, _) = multispace1(input)?;

	let (input, _) = tag_no_case("If true: throw to monkey ")(input)?;
	let (input, if_true) = map_res(digit1, |digit: &str| digit.parse())(input)?;
	let (input, _) = multispace1(input)?;

	let (input, _) = tag_no_case("If false: throw to monkey ")(input)?;
	let (output, if_false) = map_res(digit1, |digit: &str| digit.parse())(input)?;

	Ok((
		output,
		Monkey {
			id: num,
			items: items.into(),
			operation,
			test_divisible_by,
			if_true,
			if_false,
			handled: 0,
		},
	))
}

fn nom_operation(input: &str) -> MyIResult<Operation> {
	let (input, _) = tag_no_case("Operation: new = ")(input)?;
	let (input, a) = nom_value(input)?;
	let (input, _) = multispace0(input)?;
	let (input, operator) = nom_operator(input)?;
	let (input, _) = multispace0(input)?;
	let (output, b) = nom_value(input)?;
	Ok((output, Operation { a, operator, b }))
}

fn nom_value(input: &str) -> MyIResult<Value> {
	let old = map(tag_no_case("old"), |_| Value::Old);
	let number = map(nom_number, Value::Number);
	alt((old, number))(input)
}

fn nom_number(input: &str) -> MyIResult<Number> {
	map_res(digit1, |digit: &str| digit.parse())(input)
}

fn nom_operator(input: &str) -> MyIResult<Operator> {
	let add = map(nom_char('+'), |_| Operator::Add);
	let multiply = map(nom_char('*'), |_| Operator::Multiply);
	alt((add, multiply))(input)
}
