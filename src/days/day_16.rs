//! Day 16.

use std::collections::HashMap;

use lazy_regex::regex_captures;
use petgraph::{algo::dijkstra, Graph, Undirected};

type NodeIndex = petgraph::graph::NodeIndex<u32>;

fn input() -> &'static str {
	include_str!("day_16.txt")
}

pub fn run() {
	let graph = ValveGraph::parse(input());

	println!("Part 1:");
	let pressure_release = graph.optimal_pressure_release();
	println!("Optimal pressure release: {pressure_release}");
	println!();
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

	fn find_optimal_path(
		&self,
		current: NodeIndex,
		activated: &[NodeIndex],
		time_remaining: usize,
	) -> u64 {
		let paths = dijkstra(&self.graph, current, None, |_| 1);
		paths
			.into_iter()
			.filter(|(_, len)| *len + 1 < time_remaining)
			.filter(|(node, _)| self.graph[*node] != 0)
			.filter(|(node, _)| !activated.contains(node))
			.map(|(node, len)| {
				let mut new_active = activated.to_vec();
				new_active.push(node);
				self.get_flow(activated) * (len + 1) as u64
					+ self.find_optimal_path(node, &new_active, time_remaining - len - 1)
			})
			.max()
			.unwrap_or_else(|| self.get_flow(activated) * time_remaining as u64)
	}

	pub fn optimal_pressure_release(&self) -> u64 {
		self.find_optimal_path(self.indices["AA"], &[], 30)
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
		assert_eq!(graph.optimal_pressure_release(), 1651);
	}
}
