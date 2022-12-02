//! Day 2.

fn input() -> &'static str {
	include_str!("day_01.txt")
}

pub fn run() {
	println!("Part 1:");
	let max_elf_calories = input()
		.split("\n\n")
		.map(|elve| elve.lines().map(|item| item.parse::<u32>().expect("parsing")).sum::<u32>())
		.max()
		.expect("getting maximum");
	println!("Maximum elf calories: {max_elf_calories}");
	println!();

	println!("Part 2:");
	let mut elf_calories: Vec<u32> = input()
		.split("\n\n")
		.map(|elve| elve.lines().map(|item| item.parse::<u32>().expect("parsing")).sum::<u32>())
		.collect();
	elf_calories.sort();
	let top_3_sum: u32 = elf_calories.windows(3).last().expect("get last").iter().sum();
	println!("Top 3 elf calorie sum: {top_3_sum}");
}
