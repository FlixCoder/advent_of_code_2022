//! Day 16.

use std::collections::HashMap;

use lazy_regex::regex_captures;
use petgraph::{algo::dijkstra, Graph, Undirected};
use rayon::prelude::*;

type NodeIndex = petgraph::graph::NodeIndex<u32>;

fn input() -> &'static str {
	include_str!("day_16.txt")
}

pub fn run() {
	let graph = ValveGraph::parse(input());

	println!("Part 1:");
	let pressure_release = graph.optimal_pressure_release_1();
	println!("Optimal pressure release: {pressure_release}");
	println!();

	println!("Part 2:");
	let pressure_release = graph.optimal_pressure_release_2();
	println!("Optimal pressure release: {pressure_release}");
}

struct ValveGraph<'a> {
	graph: Graph<u32, (), Undirected>,
	indices: HashMap<&'a str, NodeIndex>,
}

impl<'a> ValveGraph<'a> {
	fn get_flow(&self, nodes: &[NodeIndex]) -> u64 {
		let mut flow = 0;
		for node in nodes {
			flow += self.graph[*node] as u64;
		}
		flow
	}

	fn shortest_paths(&self) -> HashMap<NodeIndex, HashMap<NodeIndex, usize>> {
		let mut shortest_paths = HashMap::new();
		for start in self.indices.values().copied() {
			let path_lengths = dijkstra(&self.graph, start, None, |_| 1);
			shortest_paths.insert(start, path_lengths);
		}
		shortest_paths
	}

	fn get_path_pressure_release(
		&self,
		shortest_paths: &HashMap<NodeIndex, HashMap<NodeIndex, usize>>,
		path: &[NodeIndex],
		mut time: usize,
	) -> u64 {
		let mut pressure_release = 0;
		let mut active = Vec::new();

		let mut path = path.iter().peekable();
		while let Some(from) = path.next() {
			if let Some(to) = path.peek().copied() {
				let len = shortest_paths[from][to];
				pressure_release += self.get_flow(&active) * (len + 1) as u64;
				time = time - len - 1;
				active.push(*to);
			}
		}

		pressure_release += self.get_flow(&active) * time as u64;
		pressure_release
	}

	fn get_combined_path_pressure_release(
		&self,
		shortest_paths: &HashMap<NodeIndex, HashMap<NodeIndex, usize>>,
		path1: &[NodeIndex],
		path2: &[NodeIndex],
		time: usize,
	) -> u64 {
		let mut pressure_release = 0;
		let mut active = Vec::new();

		let mut next1 = path1.iter().copied().peekable();
		let mut arrives1 = 0;
		let mut next2 = path2.iter().copied().peekable();
		let mut arrives2 = 0;
		for i in 0..time {
			if arrives1 == i {
				if let Some(arrived) = next1.next() {
					if !active.contains(&arrived) {
						active.push(arrived);
					}
					if let Some(next) = next1.peek() {
						arrives1 = i + shortest_paths[&arrived][next] + 1;
					}
				}
			}

			if arrives2 == i {
				if let Some(arrived) = next2.next() {
					if !active.contains(&arrived) {
						active.push(arrived);
					}
					if let Some(next) = next2.peek() {
						arrives2 = i + shortest_paths[&arrived][next] + 1;
					}
				}
			}

			pressure_release += self.get_flow(&active);
		}

		pressure_release
	}

	fn find_all_possible_paths(
		&self,
		shortest_paths: &HashMap<NodeIndex, HashMap<NodeIndex, usize>>,
		start: NodeIndex,
		visited: &[NodeIndex],
		time: usize,
	) -> Vec<Vec<NodeIndex>> {
		let mut new_visited = visited.to_vec();
		new_visited.push(start);

		let mut paths = Vec::new();
		let targets = self
			.indices
			.values()
			.filter(|node| shortest_paths[&start][*node] + 1 < time)
			.filter(|node| self.graph[**node] != 0)
			.filter(|node| !new_visited.contains(*node));
		for target in targets {
			let len = shortest_paths[&start][target];
			let mut new_paths =
				self.find_all_possible_paths(shortest_paths, *target, &new_visited, time - len - 1);
			new_paths.iter_mut().for_each(|path| path.push(start));
			paths.extend(new_paths);
		}

		if paths.is_empty() {
			paths.push(vec![start]);
		}
		paths
	}

	pub fn optimal_pressure_release_1(&self) -> u64 {
		let shortest_paths = self.shortest_paths();
		let mut possible_paths =
			self.find_all_possible_paths(&shortest_paths, self.indices["AA"], &[], 30);
		possible_paths.iter_mut().for_each(|path| path.reverse());

		possible_paths
			.into_iter()
			.map(|path| self.get_path_pressure_release(&shortest_paths, &path, 30))
			.max()
			.expect("There must be one maximum")
	}

	pub fn optimal_pressure_release_2(&self) -> u64 {
		let shortest_paths = self.shortest_paths();
		let mut possible_paths =
			self.find_all_possible_paths(&shortest_paths, self.indices["AA"], &[], 26);
		possible_paths.iter_mut().for_each(|path| path.reverse());

		possible_paths
			.par_iter()
			.map(|path1| {
				let mut max = 0;
				for path2 in possible_paths.iter() {
					let value =
						self.get_combined_path_pressure_release(&shortest_paths, path1, path2, 26);
					max = max.max(value);
				}
				max
			})
			.max()
			.expect("There must be one maximum")
	}

	fn parse(input: &'a str) -> Self {
		let mut nodes = Vec::new();
		let mut edges = Vec::new();
		for line in input.lines() {
			let (_all, node, flow_rate, connections) = regex_captures!(
				r#"Valve (.{2}) has flow rate=(\d+); tunnels? leads? to valves? (.+)"#,
				line
			)
			.expect("applying regex");

			nodes.push((node, flow_rate.parse::<u32>().expect("parsing number")));

			for target in connections.split(", ") {
				edges.push((node, target));
			}
		}

		let mut graph = Graph::default();
		let mut indices = HashMap::new();
		for (node, flow_rate) in nodes {
			let index = graph.add_node(flow_rate);
			indices.insert(node, index);
		}
		for (from, to) in edges {
			graph.update_edge(indices[from], indices[to], ());
		}

		Self { graph, indices }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn example_input() -> &'static str {
		r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#
	}

	#[test]
	fn part1() {
		let graph = ValveGraph::parse(example_input());
		assert_eq!(graph.optimal_pressure_release_1(), 1651);
	}

	#[test]
	fn part1_path_pressure_release() {
		let graph = ValveGraph::parse(example_input());
		let path = [
			graph.indices["AA"],
			graph.indices["DD"],
			graph.indices["BB"],
			graph.indices["JJ"],
			graph.indices["HH"],
			graph.indices["EE"],
			graph.indices["CC"],
		];
		let shortest_paths = graph.shortest_paths();
		assert_eq!(graph.get_path_pressure_release(&shortest_paths, &path, 30), 1651);
	}

	#[test]
	fn part1_all_paths() {
		let graph = ValveGraph::parse(example_input());
		let path = vec![
			graph.indices["AA"],
			graph.indices["DD"],
			graph.indices["BB"],
			graph.indices["JJ"],
			graph.indices["HH"],
			graph.indices["EE"],
			graph.indices["CC"],
		];
		let shortest_paths = graph.shortest_paths();
		let mut all_paths =
			graph.find_all_possible_paths(&shortest_paths, graph.indices["AA"], &[], 30);
		all_paths.iter_mut().for_each(|path| path.reverse());
		assert!(all_paths.contains(&path));
	}

	#[test]
	fn part2() {
		let graph = ValveGraph::parse(example_input());
		assert_eq!(graph.optimal_pressure_release_2(), 1707);
	}

	#[test]
	fn part2_path_pressure_release() {
		let graph = ValveGraph::parse(example_input());
		let shortest_paths = graph.shortest_paths();
		let path1 =
			[graph.indices["AA"], graph.indices["JJ"], graph.indices["BB"], graph.indices["CC"]];
		let path2 =
			[graph.indices["AA"], graph.indices["DD"], graph.indices["HH"], graph.indices["EE"]];
		assert_eq!(
			graph.get_combined_path_pressure_release(&shortest_paths, &path1, &path2, 26),
			1707
		);
	}
}
