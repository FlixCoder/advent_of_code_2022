//! Day 3.

use std::collections::HashSet;

fn input() -> &'static str {
	include_str!("day_03.txt")
}

pub fn run() {
	println!("Part 1:");
	let prio_items_in_both: u32 = input()
		.lines()
		.map(|line| line.split_at(line.len() / 2))
		.map(|(left, right)| {
			(left.chars().collect::<HashSet<char>>(), right.chars().collect::<HashSet<char>>())
		})
		.flat_map(|(left, right)| left.intersection(&right).copied().collect::<HashSet<char>>())
		.map(item_priority)
		.sum();
	println!("Sum of priorities: {prio_items_in_both}");
	println!();

	println!("Part 2:");
	let rucksacks = input().lines().collect::<Vec<_>>();
	let sum_of_group_prios: u32 = rucksacks
		.chunks_exact(3)
		.map(items_in_all_rucksacks)
		.inspect(|in_all| assert_eq!(in_all.len(), 1))
		.flatten()
		.map(item_priority)
		.sum();
	println!("Sum of priorities: {sum_of_group_prios}");
}

fn item_priority(item: char) -> u32 {
	if ('a'..='z').contains(&item) {
		u32::from(item as u8 - b'a') + 1
	} else if ('A'..='Z').contains(&item) {
		u32::from(item as u8 - b'A') + 27
	} else {
		panic!("Invalid item");
	}
}

fn items_in_all_rucksacks(rucksacks: &[&str]) -> HashSet<char> {
	rucksacks
		.iter()
		.copied()
		.map(|sack| sack.chars().collect::<HashSet<char>>())
		.reduce(|a, b| a.intersection(&b).copied().collect())
		.expect("reducing")
}
