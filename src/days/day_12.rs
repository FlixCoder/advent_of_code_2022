//! Day 12.

use std::{
	collections::{HashMap, VecDeque},
	str::FromStr,
};

fn input() -> &'static str {
	include_str!("day_12.txt")
}

pub fn run() {
	let grid: Grid = input().parse().expect("parsing");

	println!("Part 1:");
	let shortest_way = grid.find_shortest_way_from(grid.start);
	println!("Shortest way: {shortest_way}");
	println!();

	println!("Part 2:");
	let mut minimum = usize::MAX;
	for y in 0..grid.height() {
		for x in 0..grid.width() {
			let pos = Position { x, y };
			if grid.get(pos) == 0 {
				let length = grid.find_shortest_way_from(pos);
				minimum = minimum.min(length);
			}
		}
	}
	println!("Shortest way: {minimum}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Position {
	pub x: usize,
	pub y: usize,
}

impl Position {
	pub fn go(&self, direction: Direction) -> Self {
		match direction {
			Direction::Up => Self { x: self.x, y: self.y - 1 },
			Direction::Right => Self { x: self.x + 1, y: self.y },
			Direction::Down => Self { x: self.x, y: self.y + 1 },
			Direction::Left => Self { x: self.x - 1, y: self.y },
		}
	}
}

enum Direction {
	Up,
	Right,
	Down,
	Left,
}

/// The map as a grid of heights.
struct Grid {
	width: usize,
	height: Vec<u8>,
	start: Position,
	end: Position,
}

impl Grid {
	pub fn width(&self) -> usize {
		self.width
	}

	pub fn height(&self) -> usize {
		self.height.len() / self.width
	}

	/// Get the height at the position.
	pub fn get(&self, pos: Position) -> u8 {
		self.height[pos.y * self.width + pos.x]
	}

	/// Get the possible next directions from this position. Step size is the
	/// height difference that can be passed upwards.
	pub fn possible_directions(&self, pos: Position, step_size: u8) -> Vec<Direction> {
		let mut directions = Vec::new();
		let value = self.get(pos) + step_size;

		if pos.y > 0 && self.get(Position { x: pos.x, y: pos.y - 1 }) <= value {
			directions.push(Direction::Up);
		}
		if pos.x < self.width() - 1 && self.get(Position { x: pos.x + 1, y: pos.y }) <= value {
			directions.push(Direction::Right);
		}
		if pos.y < self.height() - 1 && self.get(Position { x: pos.x, y: pos.y + 1 }) <= value {
			directions.push(Direction::Down);
		}
		if pos.x > 0 && self.get(Position { x: pos.x - 1, y: pos.y }) <= value {
			directions.push(Direction::Left);
		}

		directions
	}

	/// Find the shortest way and return its length. `usize::MAX` means there is
	/// no way to the end.
	pub fn find_shortest_way_from(&self, start: Position) -> usize {
		let mut visited = HashMap::new();
		visited.insert(start, 0);
		let mut queue = VecDeque::new();
		queue.push_back(start);

		while let Some(current) = queue.pop_front() {
			for direction in self.possible_directions(current, 1) {
				let new_pos = current.go(direction);

				#[allow(clippy::map_entry)]
				if !visited.contains_key(&new_pos) {
					visited.insert(new_pos, visited[&current] + 1);
					queue.push_back(new_pos);
				}
			}
		}

		visited.get(&self.end).copied().unwrap_or(usize::MAX)
	}
}

impl FromStr for Grid {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut width = 0;
		let mut height = Vec::new();
		let mut start = Position::default();
		let mut end = Position::default();
		for (y, line) in s.lines().enumerate() {
			if width == 0 {
				width = line.chars().count();
			}

			for (x, c) in line.chars().enumerate() {
				if c == 'S' {
					start = Position { x, y };
					height.push(0);
				} else if c == 'E' {
					end = Position { x, y };
					height.push(b'z' - b'a');
				} else {
					height.push(c as u8 - b'a');
				}
			}
		}
		Ok(Self { width, height, start, end })
	}
}
