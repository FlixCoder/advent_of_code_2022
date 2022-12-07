//! Day 7.

use std::collections::HashMap;

fn input() -> &'static str {
	include_str!("day_07.txt")
}

pub fn run() {
	let file_system = Directory::from_shell_output(input());

	println!("Part 1:");
	let sum_of_sizes: usize = file_system
		.all_directories()
		.into_iter()
		.map(Directory::size)
		.filter(|size| *size <= 100_000)
		.sum();
	println!("Sum of directories below a size of 100k: {sum_of_sizes}");
	println!();

	println!("Part 2:");
	let required_to_free = file_system.size() - 40_000_000;
	let smallest_large_enough = file_system
		.all_directories()
		.into_iter()
		.map(Directory::size)
		.filter(|size| *size >= required_to_free)
		.min()
		.expect("no such folder found");
	println!("Smallest folder size that frees up enough space: {smallest_large_enough}");
}

/// A filesystem item.
#[derive(Debug)]
enum FileSystemItem {
	/// A File.
	File(File),
	/// A directory.
	Directory(Directory),
}

impl FileSystemItem {
	/// Return the size of this file or directory.
	pub fn size(&self) -> usize {
		match self {
			Self::File(file) => file.size(),
			Self::Directory(dir) => dir.size(),
		}
	}

	/// Return the inner directory.
	pub fn as_directory(&self) -> Option<&Directory> {
		match self {
			Self::Directory(dir) => Some(dir),
			_ => None,
		}
	}

	/// Return the inner directory.
	pub fn as_directory_mut(&mut self) -> Option<&mut Directory> {
		match self {
			Self::Directory(dir) => Some(dir),
			_ => None,
		}
	}
}

/// A file in the filesystem.
#[derive(Debug, Default)]
struct File {
	/// File size.
	size: usize,
}

impl File {
	/// Get the size.
	pub fn size(&self) -> usize {
		self.size
	}
}

/// A directory in the filesystem.
#[derive(Debug, Default)]
struct Directory {
	/// A map from names to the item.
	items: HashMap<String, FileSystemItem>,
}

impl Directory {
	/// Get the directories total recursive size.
	pub fn size(&self) -> usize {
		self.items.values().map(FileSystemItem::size).sum()
	}

	/// Gets or creates the directory at the specified position.
	pub fn get_or_create_directory_mut(&mut self, names: &[&str]) -> Option<&mut Self> {
		let mut current = self;
		for name in names {
			current = current
				.items
				.entry(name.to_string())
				.or_insert_with(|| FileSystemItem::Directory(Directory::default()))
				.as_directory_mut()?;
		}
		Some(current)
	}

	/// Build up a directory from AoC command output.
	pub fn from_shell_output(shell: &str) -> Self {
		let mut lines = shell.lines().peekable();
		let mut root = Self::default();

		let mut current_folder = Vec::new();
		while let Some(command) = lines.next() {
			if command.starts_with("$ cd") {
				let argument = &command[5..];
				match argument {
					"/" => current_folder.clear(),
					".." => {
						current_folder.pop();
					}
					name => {
						current_folder.push(name);
					}
				}
			} else if command.starts_with("$ ls") {
				let current = root
					.get_or_create_directory_mut(&current_folder)
					.expect("Directories must not be files");
				while lines.peek().map_or(false, |s| !s.starts_with('$')) {
					let (left, right) = lines
						.next()
						.unwrap()
						.split_once(' ')
						.expect("List info must include space");
					if left == "dir" {
						current
							.items
							.entry(right.to_owned())
							.or_insert_with(|| FileSystemItem::Directory(Directory::default()));
					} else {
						let size = left.parse().expect("number parsing");
						current.items.insert(right.to_owned(), FileSystemItem::File(File { size }));
					}
				}
			} else {
				panic!("Unrecognized command: {command}");
			}
		}

		root
	}

	/// Return a list of all directories.
	pub fn all_directories(&self) -> Vec<&Self> {
		let mut directories = vec![self];

		for item in self.items.values() {
			if let Some(dir) = item.as_directory() {
				directories.extend(dir.all_directories());
			}
		}

		directories
	}
}
