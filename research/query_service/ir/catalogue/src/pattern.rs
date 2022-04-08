//
//! Copyright 2020 Alibaba Group Holding Limited.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//! http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};

#[derive(Debug, Clone)]
enum Direction {
    Out = 0,
    Incoming,
}

#[derive(Debug, Clone)]
struct PatternVertex {
    index: u64,
    label: u64,
    order: u64,
    connect_edges: BTreeMap<u64, (u64, Direction)>,
    connect_vertices: BTreeMap<u64, (u64, Direction)>,
    out_degree: u64,
    in_degree: u64,
}

#[derive(Debug, Clone, Copy)]
struct PatternEdge {
    index: u64,
    label: u64,
    start_v_index: u64,
    end_v_index: u64,
    start_v_label: u64,
    end_v_label: u64,
}

#[derive(Debug, Clone)]
struct Pattern {
    edges: BTreeMap<u64, PatternEdge>,
    vertices: BTreeMap<u64, PatternVertex>,
    edge_label_map: HashMap<u64, BTreeSet<u64>>,
    vertex_label_map: HashMap<u64, BTreeSet<u64>>,
}

impl Pattern {
    fn reorder_label_vertices(&mut self, v_label: u64) {}

    fn reorder_vertices(&mut self) {
        let mut v_labels = Vec::with_capacity(self.vertex_label_map.len());
        for (v_label, _) in &self.vertex_label_map {
            v_labels.push(*v_label)
        }
        for v_label in v_labels {
            self.reorder_label_vertices(v_label)
        }
    }

    /// Get the Order of two PatternEdges of a Pattern
    /// The comparison logics are:
    /// if e1's label is less than e2's, then e1 < e2, vice versa,
    /// else if equal, move to next comparison step:
    /// if e1's start vertex label is less than e2's then e1 < e2, vice versa,
    /// else if equal, move to next comparison step:
    /// if e1's end vertex label is less than e2's ,then e1 < e2, vice versa,
    /// else if equal, move to next comparison step:
    /// if e1's start vertex's order is less than e2's then e1 < e2, vice versa,
    /// else if equal, move to next comparison step:
    /// if e1's end vertex's order is less than e2's ,then e1 < e2, vice versa,
    /// else if equal, move to next comparison step:
    fn cmp_edges(&self, e1_index: u64, e2_index: u64) -> Ordering {
        let e1 = self.edges.get(&e1_index).unwrap();
        let e2 = self.edges.get(&e2_index).unwrap();
        match e1.label.cmp(&e2.label) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            _ => (),
        }
        match e1.start_v_label.cmp(&e2.start_v_label) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            _ => (),
        }
        match e1.end_v_label.cmp(&e2.end_v_label) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            _ => (),
        }
        let e1_start_order = self
            .vertices
            .get(&e1.start_v_index)
            .unwrap()
            .order;
        let e2_start_order = self
            .vertices
            .get(&e2.start_v_index)
            .unwrap()
            .order;
        match e1_start_order.cmp(&e2_start_order) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            _ => (),
        }
        let e1_end_order = self
            .vertices
            .get(&e1.end_v_index)
            .unwrap()
            .order;
        let e2_end_order = self
            .vertices
            .get(&e2.end_v_index)
            .unwrap()
            .order;
        match e1_end_order.cmp(&e2_end_order) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            _ => (),
        }
        Ordering::Equal
    }

    /// Get a vector of ordered edges's indexes of a Pattern
    /// The comparison is based on the `cmp_edges` method above to get the order
    fn get_ordered_edges(&self) -> Vec<u64> {
        let mut ordered_edges = Vec::new();
        for (&edge, _) in &self.edges {
            ordered_edges.push(edge);
        }
        ordered_edges.sort_by(|e1_index, e2_index| self.cmp_edges(*e1_index, *e2_index));
        ordered_edges
    }

    /// Get a edge encode unit of a PatternEdge
    /// The unit contains 5 components:
    /// (edge's label, start vertex's label, end vertex's label, start vertex's label, end vertex's label)
    fn get_edge_encode_unit(&self, edge_index: u64) -> (u64, u64, u64, u64, u64) {
        let edge = self.edges.get(&edge_index).unwrap();
        let start_v_order = self
            .vertices
            .get(&edge.start_v_index)
            .unwrap()
            .order;
        let end_v_order = self
            .vertices
            .get(&edge.end_v_index)
            .unwrap()
            .order;
        (edge_index, edge.start_v_label, edge.end_v_label, start_v_order, end_v_order)
    }
}

impl From<Vec<PatternEdge>> for Pattern {
    fn from(edges: Vec<PatternEdge>) -> Pattern {
        let mut new_pattern = Pattern {
            edges: BTreeMap::new(),
            vertices: BTreeMap::new(),
            edge_label_map: HashMap::new(),
            vertex_label_map: HashMap::new(),
        };
        for edge in edges {
            // Add the new Pattern Edge to the new Pattern
            new_pattern.edges.insert(edge.index, edge);
            let edge_set = new_pattern
                .edge_label_map
                .entry(edge.label)
                .or_insert(BTreeSet::new());
            edge_set.insert(edge.index);
            // Add or update the start & end vertex to the new Pattern
            match new_pattern
                .vertices
                .get_mut(&edge.start_v_index)
            {
                // the start vertex existed, just update the connection info
                Some(start_vertex) => {
                    start_vertex
                        .connect_edges
                        .insert(edge.index, (edge.end_v_index, Direction::Out));
                    start_vertex
                        .connect_vertices
                        .insert(edge.end_v_index, (edge.index, Direction::Out));
                    start_vertex.out_degree += 1;
                }
                // the start vertex not existed, add to the new Pattern
                None => {
                    new_pattern.vertices.insert(
                        edge.start_v_index,
                        PatternVertex {
                            index: edge.start_v_index,
                            label: edge.start_v_label,
                            order: 0,
                            connect_edges: BTreeMap::from([(
                                edge.index,
                                (edge.end_v_index, Direction::Out),
                            )]),
                            connect_vertices: BTreeMap::from([(
                                edge.end_v_index,
                                (edge.index, Direction::Out),
                            )]),
                            out_degree: 1,
                            in_degree: 0,
                        },
                    );
                }
            }
            match new_pattern.vertices.get_mut(&edge.end_v_index) {
                // the end vertex existed, just update the connection info
                Some(end_vertex) => {
                    end_vertex
                        .connect_edges
                        .insert(edge.index, (edge.start_v_index, Direction::Incoming));
                    end_vertex
                        .connect_vertices
                        .insert(edge.start_v_index, (edge.index, Direction::Incoming));
                    end_vertex.in_degree += 1;
                }
                // the end vertex not existed, add the new Pattern
                None => {
                    new_pattern.vertices.insert(
                        edge.end_v_index,
                        PatternVertex {
                            index: edge.end_v_index,
                            label: edge.end_v_label,
                            order: 0,
                            connect_edges: BTreeMap::from([(
                                edge.index,
                                (edge.start_v_index, Direction::Incoming),
                            )]),
                            connect_vertices: BTreeMap::from([(
                                edge.start_v_index,
                                (edge.index, Direction::Incoming),
                            )]),
                            out_degree: 0,
                            in_degree: 1,
                        },
                    );
                }
            }
        }
        new_pattern
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
