//! Day 13.

use std::{cmp::Ordering, str::FromStr};

use nom::{
	branch::alt,
	character::complete::{char as nom_char, digit1, multispace1},
	combinator::{map, map_res},
	error::{convert_error, VerboseError},
	multi::{separated_list0, separated_list1},
	Finish,
};

type MyIResult<'a, O> = nom::IResult<&'a str, O, VerboseError<&'a str>>;

fn input() -> &'static str {
	include_str!("day_13.txt")
}

pub fn run() {
	println!("Part 1:");
	let pairs = parse_pairs(input());
	let sum_of_indices: usize =
		pairs.iter().enumerate().filter(|(_, pair)| pair.is_in_order()).map(|(i, _)| i + 1).sum();
	println!("Sum of indices that are already in order: {sum_of_indices}");
	println!();

	println!("Part 2:");
	let mut lists = parse_lists(input());
	let divider_1 = List { items: vec![ListItem::List(List { items: vec![ListItem::Number(2)] })] };
	let divider_2 = List { items: vec![ListItem::List(List { items: vec![ListItem::Number(6)] })] };
	lists.push(divider_1.clone());
	lists.push(divider_2.clone());
	lists.sort();
	let product_of_divider_indices: usize = lists
		.iter()
		.enumerate()
		.filter(|(_, list)| **list == divider_1 || **list == divider_2)
		.map(|(i, _)| i + 1)
		.product();
	println!("Product of divider indices: {product_of_divider_indices}");
}

#[derive(Debug, PartialEq, Eq)]
struct Pair {
	a: List,
	b: List,
}

impl Pair {
	pub fn is_in_order(&self) -> bool {
		self.a < self.b
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct List {
	items: Vec<ListItem>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ListItem {
	Number(u32),
	List(List),
}

impl PartialOrd for List {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		for (item_a, item_b) in self.items.iter().zip(other.items.iter()) {
			match item_a.cmp(item_b) {
				Ordering::Less => return Some(Ordering::Less),
				Ordering::Greater => return Some(Ordering::Greater),
				_ => continue,
			}
		}
		Some(self.items.len().cmp(&other.items.len()))
	}
}

impl Ord for List {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

impl PartialOrd for ListItem {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self, other) {
			(Self::Number(a), Self::Number(b)) => a.partial_cmp(b),
			(Self::List(a), Self::List(b)) => a.partial_cmp(b),
			(num_a, Self::List(b)) => {
				let a = List { items: vec![num_a.clone()] };
				a.partial_cmp(b)
			}
			(Self::List(a), num_b) => {
				let b = List { items: vec![num_b.clone()] };
				a.partial_cmp(&b)
			}
		}
	}
}

impl Ord for ListItem {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

fn parse_pairs(input: &str) -> Vec<Pair> {
	let (_rest, pairs) = nom_pairs(input)
		.finish()
		.map_err(|err| convert_error(input, err))
		.map_err(|err| {
			eprintln!("{err}");
			err
		})
		.expect("parsing");
	pairs
}

fn parse_lists(input: &str) -> Vec<List> {
	let (_rest, lists) = nom_lists(input)
		.finish()
		.map_err(|err| convert_error(input, err))
		.map_err(|err| {
			eprintln!("{err}");
			err
		})
		.expect("parsing");
	lists
}

fn nom_pairs(input: &str) -> MyIResult<Vec<Pair>> {
	separated_list1(multispace1, nom_pair)(input)
}

fn nom_pair(input: &str) -> MyIResult<Pair> {
	let (input, a) = nom_list(input)?;
	let (input, _) = multispace1(input)?;
	let (output, b) = nom_list(input)?;
	Ok((output, Pair { a, b }))
}

fn nom_lists(input: &str) -> MyIResult<Vec<List>> {
	separated_list1(multispace1, nom_list)(input)
}

fn nom_list(input: &str) -> MyIResult<List> {
	let (input, _) = nom_char('[')(input)?;
	let (input, items) = separated_list0(nom_char(','), nom_list_item)(input)?;
	let (output, _) = nom_char(']')(input)?;
	Ok((output, List { items }))
}

fn nom_list_item(input: &str) -> MyIResult<ListItem> {
	let number = map(nom_number, ListItem::Number);
	let list = map(nom_list, ListItem::List);
	alt((number, list))(input)
}

fn nom_number<N: FromStr>(input: &str) -> MyIResult<N> {
	map_res(digit1, |digit: &str| digit.parse())(input)
}
