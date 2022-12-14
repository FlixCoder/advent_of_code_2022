//! Day 14.

use std::{collections::HashMap, str::FromStr};

fn input() -> &'static str {
	include_str!("day_14.txt")
}

pub fn run() {
	let mut map_1: Map = input().parse().expect("parsing");
	let mut map_2 = map_1.clone();

	println!("Part 1:");
	let sand_capacity = map_1.count_sand_capacity(false);
	println!("Sand capacity: {sand_capacity}");
	println!();

	println!("Part 2:");
	let sand_capacity = map_2.count_sand_capacity(true);
	println!("Sand capacity: {sand_capacity}");
}

impl Map {
	fn possible_direction(&self, from_pos: Position) -> Option<Position> {
		let down = Position { x: from_pos.x, y: from_pos.y + 1 };
		if self.blocks.get(&down).copied().unwrap_or_default() == Block::Air {
			return Some(down);
		}

		let left = Position { x: from_pos.x - 1, y: from_pos.y + 1 };
		if self.blocks.get(&left).copied().unwrap_or_default() == Block::Air {
			return Some(left);
		}

		let right = Position { x: from_pos.x + 1, y: from_pos.y + 1 };
		if self.blocks.get(&right).copied().unwrap_or_default() == Block::Air {
			return Some(right);
		}

		None
	}

	/// Spawn a new sand and let is fall down step by step. Possible to run with
	/// or without floor. Return whether it came to a stop.
	pub fn new_sand_falls(&mut self, floor: bool) -> bool {
		let mut pos = Position { x: 500, y: 0 };
		if self.blocks.get(&pos).copied().unwrap_or_default() != Block::Air {
			return false;
		}

		while let Some(next) = self.possible_direction(pos) {
			pos = next;
			if pos.y > self.lowest_rock {
				if floor {
					break;
				} else {
					return false;
				}
			}
		}
		self.blocks.insert(pos, Block::Sand);
		true
	}

	/// Count the number of sand blocks dropped until they land in abyss or
	/// blocks the source.
	pub fn count_sand_capacity(&mut self, floor: bool) -> usize {
		let mut sands = 0;
		while self.new_sand_falls(floor) {
			sands += 1;
		}
		sands
	}
}

/// The map of the cave.
#[derive(Debug, Clone)]
struct Map {
	/// The map information, i.e. where is which block.
	blocks: HashMap<Position, Block>,
	/// Cache of the lowest rock y.
	lowest_rock: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
	x: usize,
	y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Block {
	#[default]
	Air,
	Rock,
	Sand,
}

fn make_rock_line(
	blocks: &mut HashMap<Position, Block>,
	start: Position,
	end: Position,
) -> Result<(), String> {
	if start.x == end.x {
		let from = start.y.min(end.y);
		let to = start.y.max(end.y);
		for y in from..=to {
			let pos = Position { x: start.x, y };
			blocks.insert(pos, Block::Rock);
		}
	} else if start.y == end.y {
		let from = start.x.min(end.x);
		let to = start.x.max(end.x);
		for x in from..=to {
			let pos = Position { x, y: start.y };
			blocks.insert(pos, Block::Rock);
		}
	} else {
		return Err("Diagonal lines in rock definitions!".to_owned());
	}
	Ok(())
}

impl FromStr for Map {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut blocks = HashMap::new();
		let mut lowest_rock = 0;
		for line in s.lines() {
			let mut positions = line.split(" -> ").peekable();
			while let Some(start_str) = positions.next() {
				if let Some(end_str) = positions.peek() {
					let start = start_str.parse::<Position>()?;
					let end = end_str.parse::<Position>()?;
					make_rock_line(&mut blocks, start, end)?;
					lowest_rock = lowest_rock.max(start.y).max(end.y);
				}
			}
		}
		Ok(Self { blocks, lowest_rock })
	}
}

impl FromStr for Position {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (x, y) =
			s.split_once(',').ok_or_else(|| format!("No comma in position `{s}` found!"))?;
		let x = x.parse().map_err(|err| format!("Error parsing number: {err}"))?;
		let y = y.parse().map_err(|err| format!("Error parsing number: {err}"))?;
		Ok(Self { x, y })
	}
}
