//! Day 9.

use std::{collections::HashSet, str::FromStr};

fn input() -> &'static str {
	include_str!("day_09.txt")
}

pub fn run() {
	let movements: Vec<Movement> =
		input().lines().map(Movement::from_str).collect::<Result<_, _>>().expect("parsing");

	println!("Part 1:");
	let mut state = State::<2>::new();
	let positions = state.run_movements(&movements);
	let unique_positions: HashSet<Position> = positions.into_iter().collect();
	println!("Unique tail positions: {}", unique_positions.len());
	println!();

	println!("Part 2:");
	let mut state = State::<10>::new();
	let positions = state.run_movements(&movements);
	let unique_positions: HashSet<Position> = positions.into_iter().collect();
	println!("Unique tail positions: {}", unique_positions.len());
}

/// A position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Position {
	pub x: isize,
	pub y: isize,
}

/// Current state of the head and tails positions.
#[derive(Debug)]
struct State<const KNOTS: usize> {
	/// Head and tails position.
	positions: [Position; KNOTS],
}

impl<const KNOTS: usize> State<KNOTS> {
	/// Create a new [State].
	pub fn new() -> Self {
		Self { positions: [Position::default(); KNOTS] }
	}

	/// Adjust the tails to the new position of the head.
	pub fn adjust_tails(&mut self) {
		for i_tail in 1..KNOTS {
			let xdiff = (self.positions[i_tail].x - self.positions[i_tail - 1].x).abs();
			let ydiff = (self.positions[i_tail].y - self.positions[i_tail - 1].y).abs();
			if xdiff > 1 {
				if ydiff > 0 {
					// Move diagonally
					self.positions[i_tail].x = self.positions[i_tail - 1].x
						+ if self.positions[i_tail].x < self.positions[i_tail - 1].x {
							-1
						} else {
							1
						};
					if ydiff > 1 {
						self.positions[i_tail].y = self.positions[i_tail - 1].y
							+ if self.positions[i_tail].y < self.positions[i_tail - 1].y {
								-1
							} else {
								1
							};
					} else {
						self.positions[i_tail].y = self.positions[i_tail - 1].y;
					}
				} else {
					// Move horizontally
					self.positions[i_tail].x = self.positions[i_tail - 1].x
						+ if self.positions[i_tail].x < self.positions[i_tail - 1].x {
							-1
						} else {
							1
						};
				}
			} else if ydiff > 1 {
				if xdiff > 0 {
					// Move diagonally
					if xdiff > 1 {
						self.positions[i_tail].x = self.positions[i_tail - 1].x
							+ if self.positions[i_tail].x < self.positions[i_tail - 1].x {
								-1
							} else {
								1
							};
					} else {
						self.positions[i_tail].x = self.positions[i_tail - 1].x;
					}
					self.positions[i_tail].y = self.positions[i_tail - 1].y
						+ if self.positions[i_tail].y < self.positions[i_tail - 1].y {
							-1
						} else {
							1
						};
				} else {
					// Move vertically
					self.positions[i_tail].y = self.positions[i_tail - 1].y
						+ if self.positions[i_tail].y < self.positions[i_tail - 1].y {
							-1
						} else {
							1
						};
				}
			}
		}
	}

	/// Run a series of head motions and return the tail's positions.
	pub fn run_movements(&mut self, movements: &[Movement]) -> Vec<Position> {
		let mut tail_positions =
			vec![self.positions.last().copied().expect("At least one knot must exist")];
		for movement in movements {
			for _ in 0..movement.steps {
				// Move head.
				match movement.direction {
					Direction::Left => self.positions[0].x -= 1,
					Direction::Right => self.positions[0].x += 1,
					Direction::Up => self.positions[0].y -= 1,
					Direction::Down => self.positions[0].y += 1,
				}

				// Move tail if it is too far.
				self.adjust_tails();
				tail_positions
					.push(self.positions.last().copied().expect("At least one knot must exist"));
			}
		}
		tail_positions
	}
}

/// One movement.
struct Movement {
	/// Direction of movement.
	direction: Direction,
	/// Number of steps to take.
	steps: usize,
}

impl FromStr for Movement {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (direction, steps) = s
			.split_once(' ')
			.ok_or_else(|| format!("`{s}` is an invalid movement! No space found!"))?;

		let direction = Direction::from_str(direction)?;
		let steps = steps.parse().map_err(|err| format!("`{steps}` is not a number: {err}"))?;

		Ok(Self { direction, steps })
	}
}

/// Direction of movement.
enum Direction {
	Left,
	Right,
	Up,
	Down,
}

impl FromStr for Direction {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"L" => Ok(Self::Left),
			"R" => Ok(Self::Right),
			"U" => Ok(Self::Up),
			"D" => Ok(Self::Down),
			_ => Err(format!("`{s}` is not a valid direction in [LRUD]!")),
		}
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	/// Example input.
	const EXAMPLE_PART1: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

	#[test]
	fn example_part1() {
		let movements: Vec<Movement> = EXAMPLE_PART1
			.lines()
			.map(Movement::from_str)
			.collect::<Result<_, _>>()
			.expect("parsing");
		let mut state = State::<2>::new();
		let positions = state.run_movements(&movements);
		let unique_positions: HashSet<Position> = positions.into_iter().collect();
		assert_eq!(unique_positions.len(), 13);
	}

	const EXAMPLE_PART2: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

	#[test]
	fn example_part2() {
		let movements: Vec<Movement> = EXAMPLE_PART2
			.lines()
			.map(Movement::from_str)
			.collect::<Result<_, _>>()
			.expect("parsing");
		let mut state = State::<10>::new();
		let positions = state.run_movements(&movements);
		let unique_positions: HashSet<Position> = positions.into_iter().collect();
		assert_eq!(unique_positions.len(), 36);
	}
}
