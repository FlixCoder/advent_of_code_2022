//! Day 10.

use std::str::FromStr;

fn input() -> &'static str {
	include_str!("day_10.txt")
}

pub fn run() {
	let commands: Vec<Command> =
		input().lines().map(Command::from_str).collect::<Result<_, _>>().expect("parsing");
	let mut cpu = Cpu::default();
	let states = cpu.run_program(&commands);

	println!("Part 1:");
	let mut sum_of_signal_strengths = 0_i64;
	for cycle in (20..=220).step_by(40) {
		sum_of_signal_strengths += states[cycle - 1].register_x * cycle as i64;
	}
	println!("Sum of signal strengths: {sum_of_signal_strengths}");
	println!();

	println!("Part 2:");
	for y in 0..6 {
		let mut line = String::new();
		for x in 0..40 {
			let cycle = y * 40 + x;
			let c = if (x as i64 - states[cycle].register_x).abs() <= 1 { '#' } else { ' ' };
			line.push(c);
		}
		println!("{line}");
	}
}

/// The CPU in its current state.
#[derive(Debug, Clone, Copy)]
struct Cpu {
	/// The X register.
	register_x: i64,
}

impl Default for Cpu {
	fn default() -> Self {
		Self { register_x: 1 }
	}
}

impl Cpu {
	/// Execute the given command.
	pub fn execute(&mut self, command: &Command) {
		match command {
			Command::Noop => (),
			Command::AddX(val) => self.register_x += val,
		}
	}

	/// Run a program, returning the state at every cycle.
	pub fn run_program(&mut self, commands: &[Command]) -> Vec<Self> {
		let mut states = vec![*self];
		for command in commands {
			for _ in 1..command.cycles() {
				states.push(*self);
			}
			self.execute(command);
			states.push(*self);
		}
		states
	}
}

/// Assembly CPU command.
enum Command {
	/// No operation.
	Noop,
	/// Add to the X register.
	AddX(i64),
}

impl Command {
	/// Get the number of cycles this command takes.
	pub fn cycles(&self) -> usize {
		match self {
			Self::Noop => 1,
			Self::AddX(_) => 2,
		}
	}
}

impl FromStr for Command {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.split_once(' ') {
			Some((left, right)) if left == "addx" => Ok(Self::AddX(
				right.parse().map_err(|err| format!("Invalid number: {right} ({err})"))?,
			)),
			None if s == "noop" => Ok(Self::Noop),
			_ => Err(format!("Invalid input: {s}")),
		}
	}
}
