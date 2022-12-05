//! Day 5.

use std::str::FromStr;

use error_stack::{report, IntoReport, Report, ResultExt};
use lazy_regex::regex_captures;
use thiserror::Error;

fn input() -> &'static str {
	include_str!("day_05.txt")
}

pub fn run() {
	let mut hanoi1: Hanoi = input().parse().expect("parsing");
	let mut hanoi2 = hanoi1.clone();

	println!("Part 1:");
	hanoi1.apply_steps_one().expect("applying steps");
	let top_crates = hanoi1.top_crates();
	println!("Top crates: {top_crates}");
	println!();

	println!("Part 2:");
	hanoi2.apply_steps_multiple().expect("applying steps");
	let top_crates = hanoi2.top_crates();
	println!("Top crates: {top_crates}");
}

/// Towers of hanoi, advent of code edition.
#[derive(Debug, Clone)]
struct Hanoi {
	towers: Vec<Vec<ItemCrate>>,
	steps: Vec<HanoiStep>,
}

impl Hanoi {
	/// Apply the steps to the towers, moving one crate at a time.
	fn apply_steps_one(&mut self) -> Result<(), Error> {
		for step in self.steps.drain(..) {
			for _ in 0..step.moved {
				let item = self
					.towers
					.get_mut(step.from - 1)
					.ok_or(Error::MovementIndexOutOfBounds)?
					.pop()
					.ok_or(Error::MovementIndexOutOfBounds)?;
				self.towers.get_mut(step.to - 1).ok_or(Error::MovementIndexOutOfBounds)?.push(item);
			}
		}
		Ok(())
	}

	/// Apply the steps to the towers, moving multiple crates at a time.
	fn apply_steps_multiple(&mut self) -> Result<(), Error> {
		for step in self.steps.drain(..) {
			let tower =
				self.towers.get_mut(step.from - 1).ok_or(Error::MovementIndexOutOfBounds)?;
			let items = tower.split_off(tower.len() - step.moved as usize);
			self.towers.get_mut(step.to - 1).ok_or(Error::MovementIndexOutOfBounds)?.extend(items);
		}
		Ok(())
	}

	/// Get the top crate of each tower and combine them to a string.
	fn top_crates(&self) -> String {
		self.towers.iter().filter_map(|tower| tower.last()).map(|item| item.0).collect()
	}
}

impl FromStr for Hanoi {
	type Err = Report<ParseError>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (towers, steps) = s.split_once("\n\n").ok_or(ParseError::NoTowerStepsSeparation)?;

		let mut tower_lines: Vec<&str> = towers.lines().collect();
		let number_of_towers = tower_lines
			.last()
			.ok_or(ParseError::WrongTower)?
			.split_whitespace()
			.filter(|s| !s.is_empty())
			.count();
		tower_lines.pop();
		tower_lines.reverse();
		let mut towers = Vec::new();
		for x in 0..number_of_towers {
			let crates = tower_lines
				.iter()
				.map(|line| line.split_at(x * 4).1.split_at(3).0)
				.filter(|item| !item.trim().is_empty())
				.map(ItemCrate::from_str)
				.collect::<Result<_, _>>()?;
			towers.push(crates);
		}

		let steps = steps.lines().map(HanoiStep::from_str).collect::<Result<_, _>>()?;
		Ok(Self { towers, steps })
	}
}

/// A crate, denoted by e.g. `[C]`.
#[derive(Debug, Clone, Copy)]
struct ItemCrate(char);

impl FromStr for ItemCrate {
	type Err = Report<ParseError>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let trimmed = s.trim();
		if !(trimmed.starts_with('[') && trimmed.ends_with(']')) {
			return Err(report!(ParseError::WrongCrateFormat(s.to_owned())));
		}

		let c = trimmed.chars().nth(1).ok_or_else(|| ParseError::WrongCrateFormat(s.to_owned()))?;

		Ok(Self(c))
	}
}

/// A step of Hanoi movement: move of x crates from one tower to the other.
#[derive(Debug, Clone, Copy)]
struct HanoiStep {
	/// Number of creates moved.
	moved: u8,
	/// From this tower (1-indexed).
	from: usize,
	/// To this tower (1-indexed).
	to: usize,
}

impl FromStr for HanoiStep {
	type Err = Report<ParseError>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (_whole, moved, from, to) = regex_captures!(r#"move (\d+) from (\d+) to (\d+)"#, s)
			.ok_or_else(|| ParseError::MovementWrong(s.to_owned()))?;

		let moved = moved
			.parse()
			.into_report()
			.change_context_lazy(|| ParseError::NumberParsing(moved.to_owned()))?;
		let from = from
			.parse()
			.into_report()
			.change_context_lazy(|| ParseError::NumberParsing(from.to_owned()))?;
		let to = to
			.parse()
			.into_report()
			.change_context_lazy(|| ParseError::NumberParsing(to.to_owned()))?;

		Ok(Self { moved, from, to })
	}
}

/// Parsing error.
#[derive(Debug, Error)]
enum ParseError {
	#[error("No separation between towers part and steps part found")]
	NoTowerStepsSeparation,
	#[error("Wrong Tower format")]
	WrongTower,
	#[error("`{0}` is not in correct crate format like `[C]`")]
	WrongCrateFormat(String),
	#[error("This movement step was not recognized: {0}")]
	MovementWrong(String),
	#[error("Error parsing a number: {0}")]
	NumberParsing(String),
}

/// Execution error.
#[derive(Debug, Error)]
enum Error {
	#[error("Movement index out of bounds")]
	MovementIndexOutOfBounds,
}
