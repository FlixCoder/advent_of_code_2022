//! Day 4.

use std::{ops::RangeInclusive, str::FromStr};

use error_stack::{IntoReport, Report, ResultExt};
use thiserror::Error;

fn input() -> &'static str {
	include_str!("day_04.txt")
}

pub fn run() {
	let pairings: Vec<Pairing> =
		input().lines().map(Pairing::from_str).collect::<Result<_, _>>().expect("parsing");

	println!("Part 1:");
	let num_fully_contained =
		pairings.iter().filter(|pairing| pairing.has_fully_contained_overlap()).count();
	println!("Number of pairs with fully contained overlaps: {num_fully_contained}");
	println!();

	println!("Part 2:");
	let num_overlap = pairings.iter().filter(|pairing| pairing.has_overlap()).count();
	println!("Number of pairs with overlap: {num_overlap}");
}

/// Elf pairing.
struct Pairing {
	/// First elf.
	elf_a: Assignment,
	/// Second elf.
	elf_b: Assignment,
}

/// Elf work assignment
struct Assignment {
	/// Assignment range.
	range: RangeInclusive<u32>,
}

impl Pairing {
	/// Check whether for this pairing, one assignment fully contains the other.
	fn has_fully_contained_overlap(&self) -> bool {
		self.elf_a.range.start() == self.elf_b.range.start()
			|| self.elf_a.range.end() == self.elf_b.range.end()
			|| (self.elf_a.range.start() < self.elf_b.range.start()
				&& self.elf_a.range.end() > self.elf_b.range.end())
			|| (self.elf_a.range.start() > self.elf_b.range.start()
				&& self.elf_a.range.end() < self.elf_b.range.end())
	}

	/// Check whether this pairing has an overlap in the assignments.
	fn has_overlap(&self) -> bool {
		self.elf_a.range.contains(self.elf_b.range.start())
			|| self.elf_a.range.contains(self.elf_b.range.end())
			|| self.elf_b.range.contains(self.elf_a.range.start())
			|| self.elf_b.range.contains(self.elf_a.range.end())
	}
}

impl FromStr for Pairing {
	type Err = Report<ParseError>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (a, b) = s.split_once(',').ok_or(ParseError::MissingCommaInPair)?;
		let elf_a = a.parse()?;
		let elf_b = b.parse()?;
		Ok(Self { elf_a, elf_b })
	}
}

impl FromStr for Assignment {
	type Err = Report<ParseError>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (from, to) = s.split_once('-').ok_or(ParseError::MissingDashInRange)?;

		let from = from.parse().into_report().change_context(ParseError::Number)?;
		let to = to.parse().into_report().change_context(ParseError::Number)?;

		Ok(Self { range: from..=to })
	}
}

/// Error while parsing the input.
#[derive(Debug, Error)]
enum ParseError {
	#[error("No comma found to split the pair")]
	MissingCommaInPair,
	#[error("No `-` found in range string")]
	MissingDashInRange,
	#[error("Could not parse number")]
	Number,
}
