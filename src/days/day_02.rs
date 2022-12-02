//! Day 2.

use std::str::FromStr;

/// Puzzle input.
fn input() -> &'static str {
	include_str!("day_02.txt")
}

/// Run Day 2.
pub fn run() {
	println!("Part 1:");
	let score: u32 = input()
		.lines()
		.filter_map(|line| line.split_once(' '))
		.map(|(opponent, me)| {
			(
				RockPaperScissors::from_str(opponent).expect("parsing"),
				RockPaperScissors::from_str(me).expect("parsing"),
			)
		})
		.map(|(opponent, me)| me.score(opponent))
		.map(u32::from)
		.sum();
	println!("Total score: {score}");
	println!();

	println!("Part 2:");
	let score: u32 = input()
		.lines()
		.filter_map(|line| line.split_once(' '))
		.map(|(opponent, my_outcome)| {
			(
				RockPaperScissors::from_str(opponent).expect("parsing"),
				Outcome::from_str(my_outcome).expect("parsing"),
			)
		})
		.map(|(opponent, my_outcome)| my_outcome.to_rps_against(opponent).score(opponent))
		.map(u32::from)
		.sum();
	println!("Total score: {score}");
}

/// Rock paper or scissors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum RockPaperScissors {
	/// Rock.
	Rock = 1,
	/// Paper.
	Paper = 2,
	/// Scissors.
	Scissors = 3,
}

impl FromStr for RockPaperScissors {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_uppercase().as_str() {
			"A" | "X" => Ok(Self::Rock),
			"B" | "Y" => Ok(Self::Paper),
			"C" | "Z" => Ok(Self::Scissors),
			_ => Err(format!("Wrong rock paper scissors: {s}; expected ABC/XYZ!")),
		}
	}
}

impl RockPaperScissors {
	/// Get next rock paper scissors entry, that is stronger than self.
	pub fn next(self) -> Self {
		match self {
			Self::Rock => Self::Paper,
			Self::Paper => Self::Scissors,
			Self::Scissors => Self::Rock,
		}
	}

	/// Play against opponent.
	fn play(self, other: Self) -> Outcome {
		if self == other {
			Outcome::Draw
		} else if self.next() == other {
			Outcome::Lose
		} else {
			Outcome::Win
		}
	}

	/// Score of a round against the opponent.
	fn score(self, other: Self) -> u8 {
		self as u8 + self.play(other) as u8
	}
}

/// Round outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Outcome {
	/// Lose.
	Lose = 0,
	/// Draw.
	Draw = 3,
	/// Win.
	Win = 6,
}

impl FromStr for Outcome {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_uppercase().as_str() {
			"X" => Ok(Self::Lose),
			"Y" => Ok(Self::Draw),
			"Z" => Ok(Self::Win),
			_ => Err(format!("Wrong outcome: {s}; expected XYZ!")),
		}
	}
}

impl Outcome {
	/// Convert to [`RockPaperScissors`] when playing against the given
	/// opponent.
	fn to_rps_against(self, opponent: RockPaperScissors) -> RockPaperScissors {
		match self {
			Outcome::Lose => opponent.next().next(),
			Outcome::Draw => opponent,
			Outcome::Win => opponent.next(),
		}
	}
}
