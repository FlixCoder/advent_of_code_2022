//! Day 15.

use std::{ops::RangeInclusive, str::FromStr};

use lazy_regex::regex_captures;

fn input() -> &'static str {
	include_str!("day_15.txt")
}

pub fn run() {
	let sensors: Sensors = input().parse().expect("parsing");

	println!("Part 1:");
	let pos_count = sensors.excluded_position_count(2_000_000);
	println!("Position count that cannot contain a beacon at y=2_000_000: {pos_count}");
	println!();

	println!("Part 2:");
	let pos =
		sensors.find_available_position_in(0..=4_000_000).expect("There must be a free position");
	let tuning_freq = pos.x * 4_000_000 + pos.y;
	println!("Tuning frequency: {tuning_freq}");
}

/// The group of sensors.
struct Sensors(Vec<Sensor>);

impl Sensors {
	/// Return the range of where the sensors can reach.
	fn x_range(&self) -> RangeInclusive<isize> {
		let min = self
			.0
			.iter()
			.map(|sensor| sensor.position.x - sensor.distance_to_beacon() as isize)
			.min()
			.expect("There must be at least one sensor..");
		let max = self
			.0
			.iter()
			.map(|sensor| sensor.position.x + sensor.distance_to_beacon() as isize)
			.max()
			.expect("There must be at least one sensor..");
		min..=max
	}

	/// Return whether the sensors exclude this position.
	fn exclude(&self, pos: Position) -> bool {
		self.0.iter().any(|sensor| sensor.distance_to(pos) <= sensor.distance_to_beacon())
	}

	/// Return whether at the position is a known beacon.
	fn is_beacon(&self, pos: Position) -> bool {
		self.0.iter().any(|sensor| pos == sensor.closest_beacon)
	}

	pub fn excluded_position_count(&self, height: isize) -> usize {
		self.x_range()
			.map(|x| Position { x, y: height })
			.filter(|pos| !self.is_beacon(*pos))
			.filter(|pos| self.exclude(*pos))
			.count()
	}

	pub fn find_available_position_in(&self, area: RangeInclusive<isize>) -> Option<Position> {
		for sensor in &self.0 {
			for i in 0..=sensor.distance_to_beacon() as isize {
				let top = Position {
					x: sensor.position.x + i,
					y: sensor.position.y - sensor.distance_to_beacon() as isize - 1 + i,
				};
				let right = Position {
					x: sensor.position.x + sensor.distance_to_beacon() as isize + 1 - i,
					y: sensor.position.y + i,
				};
				let bottom = Position {
					x: sensor.position.x - i,
					y: sensor.position.y + sensor.distance_to_beacon() as isize + 1 - i,
				};
				let left = Position {
					x: sensor.position.x - sensor.distance_to_beacon() as isize - 1 + i,
					y: sensor.position.y - i,
				};
				let all = [top, right, bottom, left];

				for pos in all {
					if !self.exclude(pos) && area.contains(&pos.x) && area.contains(&pos.y) {
						return Some(pos);
					}
				}
			}
		}
		None
	}
}

impl FromStr for Sensors {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let sensors = s.lines().map(Sensor::from_str).collect::<Result<_, _>>()?;
		Ok(Self(sensors))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
	x: isize,
	y: isize,
}

#[derive(Debug, Clone)]
struct Sensor {
	position: Position,
	closest_beacon: Position,
}

impl Sensor {
	fn distance_to(&self, to: Position) -> usize {
		self.position.x.abs_diff(to.x) + self.position.y.abs_diff(to.y)
	}

	fn distance_to_beacon(&self) -> usize {
		self.distance_to(self.closest_beacon)
	}
}

impl FromStr for Sensor {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (_whole, pos_x, pos_y, beacon_x, beacon_y) = regex_captures!(
			r#"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"#,
			s
		)
		.ok_or_else(|| format!("Regex application failed on `{s}`"))?;

		let x = pos_x.parse().map_err(|err| format!("Failed to parse number: {err}"))?;
		let y = pos_y.parse().map_err(|err| format!("Failed to parse number: {err}"))?;
		let position = Position { x, y };

		let x = beacon_x.parse().map_err(|err| format!("Failed to parse number: {err}"))?;
		let y = beacon_y.parse().map_err(|err| format!("Failed to parse number: {err}"))?;
		let closest_beacon = Position { x, y };

		Ok(Sensor { position, closest_beacon })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn example_input() -> &'static str {
		r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#
	}

	#[test]
	fn part_1() {
		let sensors: Sensors = example_input().parse().expect("parsing");
		assert_eq!(sensors.excluded_position_count(10), 26);
	}

	#[test]
	fn part_2() {
		let sensors: Sensors = example_input().parse().expect("parsing");
		assert_eq!(sensors.find_available_position_in(0..=20), Some(Position { x: 14, y: 11 }))
	}
}
