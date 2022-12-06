//! Advent of code days.

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

/// Run specific AoC day.
pub fn run(day: u8) {
	match day {
		1 => day_01::run(),
		2 => day_02::run(),
		3 => day_03::run(),
		4 => day_04::run(),
		5 => day_05::run(),
		6 => day_06::run(),
		_ => unimplemented!(),
	}
}
