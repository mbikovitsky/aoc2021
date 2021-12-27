use std::ops::Add;

use itertools::Itertools;
use num::Integer;
use petgraph::{
    algo::toposort,
    graph::NodeIndex,
    stable_graph::StableDiGraph,
    visit::{depth_first_search, Control, DfsEvent, DfsPostOrder, NodeIndexable},
    EdgeDirection::Incoming,
};

#[derive(Debug)]
pub enum Element {
    Number(u32),
    Pair(Box<Element>, Box<Element>),
}

type SnailfishGraph = StableDiGraph<Option<u32>, ()>;

#[derive(Debug, Clone)]
pub struct SnailfishNum {
    graph: SnailfishGraph,
}

impl SnailfishNum {
    pub fn new(left: Box<Element>, right: Box<Element>) -> Self {
        let mut graph = SnailfishGraph::new();

        let root = graph.add_node(None);

        let mut stack: Vec<(NodeIndex, Element)> = vec![(root, Element::Pair(left, right))];

        while let Some((node, element)) = stack.pop() {
            match element {
                Element::Number(value) => {
                    graph[node] = Some(value);
                }
                Element::Pair(a, b) => {
                    let (left, right) = create_children(&mut graph, node);

                    stack.push((right, *b));
                    stack.push((left, *a));
                }
            }
        }

        Self { graph }
    }

    pub fn reduce(mut self) -> Self {
        loop {
            if explode(&mut self.graph) {
                continue;
            }

            if split(&mut self.graph) {
                continue;
            }

            break;
        }

        self
    }

    pub fn magnitude(&self) -> u32 {
        let mut magnitudes = vec![None; self.graph.node_bound()];

        let root = self.graph.externals(Incoming).exactly_one().unwrap();

        let mut dfs = DfsPostOrder::new(&self.graph, root);
        while let Some(node) = dfs.next(&self.graph) {
            match self.graph[node] {
                Some(value) => {
                    magnitudes[node.index()] = Some(value);
                }
                None => {
                    let (left, right) = self.graph.neighbors(node).collect_tuple().unwrap();

                    let left_mag = magnitudes[left.index()].unwrap();
                    let right_mag = magnitudes[right.index()].unwrap();

                    magnitudes[node.index()] = Some(3 * left_mag + 2 * right_mag);
                }
            }
        }

        magnitudes[root.index()].unwrap()
    }
}

impl Add for SnailfishNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl<'a> Add for &'a SnailfishNum {
    type Output = SnailfishNum;

    fn add(self, rhs: Self) -> Self::Output {
        let mut graph = SnailfishGraph::new();

        let root = graph.add_node(None);

        let (left, right) = create_children(&mut graph, root);

        let lhs_root = self.graph.externals(Incoming).exactly_one().unwrap();
        let rhs_root = rhs.graph.externals(Incoming).exactly_one().unwrap();

        let mut stack: Vec<(NodeIndex, &SnailfishGraph, NodeIndex)> =
            vec![(right, &rhs.graph, rhs_root), (left, &self.graph, lhs_root)];

        while let Some((target_node, source_graph, source_node)) = stack.pop() {
            match source_graph[source_node] {
                Some(value) => {
                    graph[target_node] = Some(value);
                }
                None => {
                    let (source_left, source_right) =
                        source_graph.neighbors(source_node).collect_tuple().unwrap();

                    let (left, right) = create_children(&mut graph, target_node);

                    stack.push((right, source_graph, source_right));
                    stack.push((left, source_graph, source_left));
                }
            }
        }

        SnailfishNum { graph }.reduce()
    }
}

fn create_children(graph: &mut SnailfishGraph, parent: NodeIndex) -> (NodeIndex, NodeIndex) {
    let left = graph.add_node(None);
    let right = graph.add_node(None);

    graph.add_edge(parent, right, ());
    graph.add_edge(parent, left, ());

    (left, right)
}

fn explode(graph: &mut SnailfishGraph) -> bool {
    let root = graph.externals(Incoming).exactly_one().unwrap();

    let mut depth = vec![0usize; graph.node_bound()];
    depth[root.index()] = 0;

    let control = depth_first_search(&*graph, Some(root), |event| {
        if let DfsEvent::TreeEdge(u, v) = event {
            depth[v.index()] = depth[u.index()] + 1;

            if depth[v.index()] == 4 && graph[v].is_none() {
                return Control::Break(v);
            }
        }

        Control::Continue
    });

    if let Control::Break(explode) = control {
        assert!(graph[explode].is_none());

        let (left, right) = graph.neighbors(explode).collect_tuple().unwrap();
        assert!(graph[left].is_some());
        assert!(graph[right].is_some());

        let mut sorted = toposort(&*graph, None).unwrap();
        sorted.retain(|&index| index == explode || graph[index].is_some());

        let position = sorted
            .iter()
            .find_position(|&&index| index == explode)
            .unwrap()
            .0;

        if position >= 1 {
            let previous = sorted[position - 1];
            *graph[previous].as_mut().unwrap() += graph[left].unwrap();
        }

        if position.checked_add(3).unwrap() < sorted.len() {
            let next = sorted[position + 3];
            *graph[next].as_mut().unwrap() += graph[right].unwrap();
        }

        graph[explode] = Some(0);
        graph.remove_node(left);
        graph.remove_node(right);

        return true;
    }

    false
}

fn split(graph: &mut SnailfishGraph) -> bool {
    let root = graph.externals(Incoming).exactly_one().unwrap();

    let control = depth_first_search(&*graph, Some(root), |event| {
        if let DfsEvent::Discover(u, _) = event {
            if let Some(value) = graph[u] {
                if value >= 10 {
                    return Control::Break(u);
                }
            }
        }

        Control::Continue
    });

    if let Control::Break(index) = control {
        let value = graph[index].unwrap();

        let left = graph.add_node(Some(value.div_floor(&2)));
        let right = graph.add_node(Some(value.div_ceil(&2)));

        graph[index] = None;
        graph.add_edge(index, right, ());
        graph.add_edge(index, left, ());

        return true;
    }

    false
}
