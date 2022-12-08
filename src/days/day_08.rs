//! Day 8.

use std::str::FromStr;

fn input() -> &'static str {
	include_str!("day_08.txt")
}

pub fn run() {
	let grid: Grid<u8> = input().parse().expect("parsing");

	println!("Part 1:");
	let num_visible = grid.visible_tree_grid().count();
	println!("Number of visible trees: {num_visible}");
	println!();
}

/// The grid of trees.
#[derive(Debug)]
struct Grid<T> {
	/// Grid width.
	width: usize,
	/// Heights of the trees in the grid (u8) or whether they are visible
	/// (bool).
	data: Vec<T>,
}

impl<T: Copy> Grid<T> {
	/// Get width.
	pub fn width(&self) -> usize {
		self.width
	}

	/// Get height.
	pub fn height(&self) -> usize {
		self.data.len() / self.width
	}

	/// Get data at position (x, y).
	/// Panics on out of bounds.
	pub fn get(&self, x: usize, y: usize) -> T {
		self.data.get(y * self.width + x).copied().expect("index out of bounds")
	}
}

impl Grid<u8> {
	/// Check whether the tree at position (x, y) is visible.
	/// Panics if out of bounds.
	fn is_visible(&self, x: usize, y: usize) -> bool {
		let value = self.get(x, y);

		let left_visible = (0..x).all(|xx| self.get(xx, y) < value);
		let right_visible = (x + 1..self.width()).all(|xx| self.get(xx, y) < value);
		let top_visible = (0..y).all(|yy| self.get(x, yy) < value);
		let bottom_visible = (y + 1..self.height()).all(|yy| self.get(x, yy) < value);

		left_visible || right_visible || top_visible || bottom_visible
	}

	/// Get a grid of visible trees.
	pub fn visible_tree_grid(&self) -> Grid<bool> {
		let mut visible = Vec::with_capacity(self.data.len());

		for y in 0..self.height() {
			for x in 0..self.width() {
				visible.push(self.is_visible(x, y));
			}
		}

		Grid { width: self.width, data: visible }
	}
}

impl Grid<bool> {
	/// Count the number of visible trees.
	pub fn count(&self) -> usize {
		self.data.iter().copied().filter(|b| *b).count()
	}
}

impl FromStr for Grid<u8> {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut data = Vec::with_capacity(s.chars().count());
		let mut width = 0;
		for line in s.lines() {
			if width == 0 {
				width = line.chars().count();
			} else if width != line.chars().count() {
				return Err("Grid lines have different lengths!".to_owned());
			}

			for c in line.chars() {
				let value = c as u8 - b'0';
				data.push(value);
			}
		}

		Ok(Self { width, data })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE_GRID: &str = r#"30373
25512
65332
33549
35390"#;

	#[test]
	fn test_example() {
		let grid: Grid<u8> = EXAMPLE_GRID.parse().expect("parsing");
		assert!(!grid.is_visible(3, 1));

		let num_visible = grid.visible_tree_grid().count();
		assert_eq!(num_visible, 21);
	}
}
