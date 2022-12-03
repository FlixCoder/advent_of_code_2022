//! Advent of code days.

mod day_01;
mod day_02;
mod day_03;

/// Run specific AoC day.
pub fn run(day: u8) {
	match day {
		1 => day_01::run(),
		2 => day_02::run(),
		3 => day_03::run(),
		_ => unimplemented!(),
	}
}
