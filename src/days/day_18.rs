//! Day 18.

use std::{
	collections::{HashSet, VecDeque},
	ops::{Add, AddAssign, RangeInclusive, Sub, SubAssign},
	str::FromStr,
};

use eyre::{eyre, Report, Result};
use rayon::prelude::*;

const DIRECTIONS: [Position; 6] = [
	Position { x: -1, y: 0, z: 0 },
	Position { x: 0, y: -1, z: 0 },
	Position { x: 0, y: 0, z: -1 },
	Position { x: 1, y: 0, z: 0 },
	Position { x: 0, y: 1, z: 0 },
	Position { x: 0, y: 0, z: 1 },
];

fn input() -> &'static str {
	include_str!("day_18.txt")
}

pub fn run() {
	let cuboids = parse_cuboids(input()).expect("parsing");

	println!("Part 1:");
	let total_area = total_surface_area_1(&cuboids);
	println!("Total surface area: {total_area}");
	println!();

	println!("Part 2:");
	let total_area = total_surface_area_2(&cuboids);
	println!("Total surface area: {total_area}");
}

fn total_surface_area_1(positions: &HashSet<Position>) -> u64 {
	positions
		.iter()
		.map(|pos| {
			let mut area = 6;
			for direction in DIRECTIONS {
				if positions.contains(&(*pos + direction)) {
					area -= 1;
				}
			}
			area
		})
		.sum()
}

fn total_surface_area_2(positions: &HashSet<Position>) -> u64 {
	let min_x = positions.iter().map(|pos| pos.x).min().expect("missing elements") - 1;
	let min_y = positions.iter().map(|pos| pos.y).min().expect("missing elements") - 1;
	let min_z = positions.iter().map(|pos| pos.z).min().expect("missing elements") - 1;
	let max_x = positions.iter().map(|pos| pos.x).max().expect("missing elements") + 1;
	let max_y = positions.iter().map(|pos| pos.y).max().expect("missing elements") + 1;
	let max_z = positions.iter().map(|pos| pos.z).max().expect("missing elements") + 1;
	let inside_box = [min_x..=max_x, min_y..=max_y, min_z..=max_z];

	positions
		.par_iter()
		.map(|pos| {
			let mut area = 6;
			for direction in DIRECTIONS {
				let neighbor = *pos + direction;
				if positions.contains(&neighbor)
					|| !has_path_to_freedom(positions, neighbor, inside_box.clone())
				{
					area -= 1;
				}
			}
			area
		})
		.sum()
}

fn has_path_to_freedom(
	positions: &HashSet<Position>,
	from: Position,
	inside: [RangeInclusive<isize>; 3],
) -> bool {
	let mut visited = HashSet::with_capacity(500);
	let mut queue = VecDeque::with_capacity(1000);
	queue.push_back(from);

	while let Some(current) = queue.pop_front() {
		visited.insert(current);

		// Cut-off assuming freedom/outside is reachable in less steps.
		if visited.len() >= 500 {
			return false;
		}

		for direction in DIRECTIONS {
			let pos = current + direction;

			if !inside[0].contains(&pos.x)
				|| !inside[1].contains(&pos.y)
				|| !inside[2].contains(&pos.z)
			{
				return true;
			}

			if !visited.contains(&pos) && !positions.contains(&pos) {
				queue.push_back(pos);
			}
		}
	}

	false
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Position {
	x: isize,
	y: isize,
	z: isize,
}

impl Add<Position> for Position {
	type Output = Position;

	fn add(self, rhs: Position) -> Self::Output {
		Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
	}
}

impl AddAssign<Position> for Position {
	fn add_assign(&mut self, rhs: Position) {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
	}
}

impl Sub<Position> for Position {
	type Output = Position;

	fn sub(self, rhs: Position) -> Self::Output {
		Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
	}
}

impl SubAssign<Position> for Position {
	fn sub_assign(&mut self, rhs: Position) {
		self.x -= rhs.x;
		self.y -= rhs.y;
		self.z -= rhs.z;
	}
}

fn parse_cuboids(input: &str) -> Result<HashSet<Position>> {
	input.lines().map(Position::from_str).collect()
}

impl FromStr for Position {
	type Err = Report;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut numbers = s.split(',');
		let x = numbers.next().ok_or_else(|| eyre!("too few coordinates"))?.parse()?;
		let y = numbers.next().ok_or_else(|| eyre!("too few coordinates"))?.parse()?;
		let z = numbers.next().ok_or_else(|| eyre!("too few coordinates"))?.parse()?;
		Ok(Self { x, y, z })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn example_input() -> &'static str {
		r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#
	}

	#[test]
	fn part1() {
		let cuboids = parse_cuboids(example_input()).expect("parsing");
		let total = total_surface_area_1(&cuboids);
		assert_eq!(total, 64);
	}

	#[test]
	fn part2() {
		let cuboids = parse_cuboids(example_input()).expect("parsing");
		let total = total_surface_area_2(&cuboids);
		assert_eq!(total, 58);
	}
}
