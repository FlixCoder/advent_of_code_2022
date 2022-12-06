//! Day 6.

use std::collections::VecDeque;

fn input() -> &'static str {
	include_str!("day_06.txt")
}

pub fn run() {
	println!("Part 1:");
	let start_index = find_start_of_message(input(), 4).expect("finding start of message");
	println!("Number of characters until first start-of-packet: {start_index}");
	println!();

	println!("Part 2:");
	let start_index = find_start_of_message(input(), 14).expect("finding start of message");
	println!("Number of characters until first start-of-message: {start_index}");
}

/// Returns the byte position after the first time there were `in_a_row`
/// different characters in a row.
fn find_start_of_message(msg: &str, in_a_row: usize) -> Option<usize> {
	let mut current_chars = VecDeque::with_capacity(in_a_row);
	for (index, c) in msg.char_indices() {
		current_chars.push_back(c);
		if current_chars.len() == in_a_row {
			if are_unique(current_chars.make_contiguous()) {
				return Some(index + c.len_utf8());
			}
			current_chars.pop_front();
		}
	}
	None
}

fn are_unique(characters: &[char]) -> bool {
	for (i, c) in characters.iter().enumerate() {
		if characters.split_at(i + 1).1.contains(c) {
			return false;
		}
	}
	true
}
