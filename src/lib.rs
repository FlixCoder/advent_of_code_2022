//! Functionality for AoC22.

mod days;

use clap::Parser;

/// CLI command.
#[derive(Debug, Parser)]
pub struct Cli {
	/// Advent of Code day.
	#[arg(value_parser = clap::value_parser!(u8).range(1..24))]
	day: u8,
}

impl Cli {
	/// Run the advent of code day.
	pub fn run(self) {
		days::run(self.day);
	}
}

#[cfg(test)]
mod tests {
	use clap::CommandFactory;

	use super::Cli;

	#[test]
	fn verify_cli() {
		Cli::command().debug_assert();
	}
}
