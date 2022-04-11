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

use super::extend_step::{ExtendEdge, ExtendStep};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Out = 0,
    Incoming,
}

#[derive(Debug, Clone)]
struct PatternVertex {
    id: u64,
    label: u64,
    index: u64,
    connect_edges: BTreeMap<u64, (u64, Direction)>,
    connect_vertices: BTreeMap<u64, Vec<(u64, Direction)>>,
    out_degree: u64,
    in_degree: u64,
}

#[derive(Debug, Clone, Copy)]
struct PatternEdge {
    id: u64,
    label: u64,
    start_v_id: u64,
    end_v_id: u64,
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
    /// if e1's start vertex's id is less than e2's then e1 < e2, vice versa,
    /// else if equal, move to next comparison step:
    /// if e1's end vertex's id is less than e2's ,then e1 < e2, vice versa,
    /// else if equal, move to next comparison step:
    fn cmp_edges(&self, e1_id: u64, e2_id: u64) -> Ordering {
        let e1 = self.edges.get(&e1_id).unwrap();
        let e2 = self.edges.get(&e2_id).unwrap();
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
        let e1_start_index = self.vertices.get(&e1.start_v_id).unwrap().index;
        let e2_start_index = self.vertices.get(&e2.start_v_id).unwrap().index;
        match e1_start_index.cmp(&e2_start_index) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            _ => (),
        }
        let e1_end_index = self.vertices.get(&e1.end_v_id).unwrap().index;
        let e2_end_index = self.vertices.get(&e2.end_v_id).unwrap().index;
        match e1_end_index.cmp(&e2_end_index) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            _ => (),
        }
        Ordering::Equal
    }

    /// Get a vector of ordered edges's indexes of a Pattern
    /// The comparison is based on the `cmp_edges` method above to get the Order
    fn get_ordered_edges(&self) -> Vec<u64> {
        let mut ordered_edges = Vec::new();
        for (&edge, _) in &self.edges {
            ordered_edges.push(edge);
        }
        ordered_edges.sort_by(|e1_id, e2_id| self.cmp_edges(*e1_id, *e2_id));
        ordered_edges
    }

    /// Get a edge encode unit of a PatternEdge
    /// The unit contains 5 components:
    /// (edge's label, start vertex's label, end vertex's label, start vertex's label, end vertex's label)
    fn get_edge_encode_unit(&self, edge_id: u64) -> (u64, u64, u64, u64, u64) {
        let edge = self.edges.get(&edge_id).unwrap();
        let start_v_index = self
            .vertices
            .get(&edge.start_v_id)
            .unwrap()
            .index;
        let end_v_index = self.vertices.get(&edge.end_v_id).unwrap().index;
        (edge_id, edge.start_v_label, edge.end_v_label, start_v_index, end_v_index)
    }
}

impl Pattern {
    /// Get all the vertices(id) with the same vertex label and vertex index
    /// These vertices are equivalent in the Pattern
    fn get_equivalent_vertices(&self, v_label: u64, v_index: u64) -> Vec<u64> {
        let mut equivalent_vertices = Vec::new();
        if let Some(vs_with_same_label) = self.vertex_label_map.get(&v_label) {
            for v_id in vs_with_same_label {
                if let Some(vertex) = self.vertices.get(v_id) {
                    if vertex.index == v_index {
                        equivalent_vertices.push(*v_id);
                    }
                }
            }
        }
        equivalent_vertices
    }

    /// Get the legal id for the future incoming vertex
    fn get_next_pattern_vertex_id(&self) -> u64 {
        let mut new_vertex_id = self.vertices.len() as u64;
        while self.vertices.contains_key(&new_vertex_id) {
            new_vertex_id += 1;
        }
        new_vertex_id
    }

    /// Get the legal id for the future incoming vertex
    fn get_next_pattern_edge_id(&self) -> u64 {
        let mut new_edge_id = self.edges.len() as u64;
        while self.edges.contains_key(&new_edge_id) {
            new_edge_id += 1;
        }
        new_edge_id
    }

    /// Extend the current Pattern to a new Pattern with the given ExtendStep
    /// If the ExtendStep is not matched with the current Pattern, the function will return None
    /// Else, it will return the new Pattern after the extension
    fn extend(&self, extend_step: ExtendStep) -> Option<Pattern> {
        let mut new_pattern = self.clone();
        let target_v_label = extend_step.target_v_label;
        let mut new_pattern_vertex = PatternVertex {
            id: new_pattern.get_next_pattern_vertex_id(),
            label: target_v_label,
            index: 0,
            connect_edges: BTreeMap::new(),
            connect_vertices: BTreeMap::new(),
            out_degree: 0,
            in_degree: 0,
        };
        for ((v_label, v_index), extend_edges) in extend_step.extend_edges {
            // Get all the vertices which can be used to extend with these extend edges
            let vertices_can_use = self.get_equivalent_vertices(v_label, v_index);
            // There's no enough vertices to extend, just return None
            if vertices_can_use.len() < extend_edges.len() {
                return None;
            }
            // Connect each vertex can be use to each extend edge
            for i in 0..extend_edges.len() {
                // If extend edge's target vertex label does not match with the global target vertex label
                // means the extend step is illegal, just return None
                if extend_edges[i].target_v_label != target_v_label {
                    return None;
                }
                match extend_edges[i].dir {
                    // Case that the extend edge's direciton is Out
                    Direction::Out => {
                        // new pattern edge info
                        let new_pattern_edge = PatternEdge {
                            id: new_pattern.get_next_pattern_edge_id(),
                            label: extend_edges[i].edge_label,
                            start_v_id: vertices_can_use[i],
                            end_v_id: new_pattern_vertex.id,
                            start_v_label: self
                                .vertices
                                .get(&vertices_can_use[i])
                                .unwrap()
                                .label,
                            end_v_label: new_pattern_vertex.label,
                        };
                        // update newly extended pattern vertex's connection info
                        new_pattern_vertex
                            .connect_edges
                            .insert(new_pattern_edge.id, (vertices_can_use[i], Direction::Incoming));
                        new_pattern_vertex
                            .connect_vertices
                            .insert(vertices_can_use[i], vec![(new_pattern_edge.id, Direction::Out)]);
                        new_pattern_vertex.in_degree += 1;
                        // Add he new pattern info to the new Pattern
                        new_pattern
                            .edge_label_map
                            .entry(new_pattern_edge.label)
                            .or_insert(BTreeSet::new())
                            .insert(new_pattern_edge.id);
                        new_pattern
                            .edges
                            .insert(new_pattern_edge.id, new_pattern_edge);
                    }
                    // Case that the extend edge's direction is Incoming
                    Direction::Incoming => {
                        let new_pattern_edge = PatternEdge {
                            id: new_pattern.get_next_pattern_edge_id(),
                            label: extend_edges[i].edge_label,
                            start_v_id: new_pattern_vertex.id,
                            end_v_id: vertices_can_use[i],
                            start_v_label: new_pattern_vertex.label,
                            end_v_label: self
                                .vertices
                                .get(&vertices_can_use[i])
                                .unwrap()
                                .label,
                        };
                        new_pattern_vertex
                            .connect_edges
                            .insert(new_pattern_edge.id, (vertices_can_use[i], Direction::Out));
                        new_pattern_vertex
                            .connect_vertices
                            .insert(vertices_can_use[i], vec![(new_pattern_edge.id, Direction::Incoming)]);
                        new_pattern_vertex.out_degree += 1;
                        new_pattern
                            .edge_label_map
                            .entry(new_pattern_edge.label)
                            .or_insert(BTreeSet::new())
                            .insert(new_pattern_edge.id);
                        new_pattern
                            .edges
                            .insert(new_pattern_edge.id, new_pattern_edge);
                    }
                }
            }
        }
        // Add the newly extended pattern vertex to the new pattern
        new_pattern
            .vertex_label_map
            .entry(new_pattern_vertex.label)
            .or_insert(BTreeSet::new())
            .insert(new_pattern_vertex.id);
        new_pattern
            .vertices
            .insert(new_pattern_vertex.id, new_pattern_vertex);
        new_pattern.reorder_label_vertices(target_v_label);
        Some(new_pattern)
    }
}

// Initialize a Pattern containing only one vertex from hte vertex's id and label
impl From<(u64, u64)> for Pattern {
    fn from((vertex_id, vertex_label): (u64, u64)) -> Pattern {
        let vertex = PatternVertex {
            id: vertex_id,
            label: vertex_label,
            index: 0,
            connect_edges: BTreeMap::new(),
            connect_vertices: BTreeMap::new(),
            out_degree: 0,
            in_degree: 0,
        };
        Pattern::from(vertex)
    }
}

// Initialze a Pattern from just a single Pattern Vertex
impl From<PatternVertex> for Pattern {
    fn from(vertex: PatternVertex) -> Pattern {
        Pattern {
            edges: BTreeMap::new(),
            vertices: BTreeMap::from([(vertex.id, vertex.clone())]),
            edge_label_map: HashMap::new(),
            vertex_label_map: HashMap::from([(vertex.label, BTreeSet::from([vertex.id]))]),
        }
    }
}

// Initialize a Pattern from a vertor of Pattern Edges
impl From<Vec<PatternEdge>> for Pattern {
    fn from(edges: Vec<PatternEdge>) -> Pattern {
        if edges.len() == 0 {
            panic!(
                "There should be at least one pattern edge to get a pattern. 
                   To get a pattern with single vertex, it shoud call from Pattern Vertex"
            )
        }
        let mut new_pattern = Pattern {
            edges: BTreeMap::new(),
            vertices: BTreeMap::new(),
            edge_label_map: HashMap::new(),
            vertex_label_map: HashMap::new(),
        };
        for edge in edges {
            // Add the new Pattern Edge to the new Pattern
            new_pattern.edges.insert(edge.id, edge);
            let edge_set = new_pattern
                .edge_label_map
                .entry(edge.label)
                .or_insert(BTreeSet::new());
            edge_set.insert(edge.id);
            // Add or update the start & end vertex to the new Pattern
            match new_pattern.vertices.get_mut(&edge.start_v_id) {
                // the start vertex existed, just update the connection info
                Some(start_vertex) => {
                    start_vertex
                        .connect_edges
                        .insert(edge.id, (edge.end_v_id, Direction::Out));
                    let start_vertex_connect_vertices_vec = start_vertex
                        .connect_vertices
                        .entry(edge.end_v_id)
                        .or_insert(Vec::new());
                    start_vertex_connect_vertices_vec.push((edge.id, Direction::Out));
                    start_vertex.out_degree += 1;
                }
                // the start vertex not existed, add to the new Pattern
                None => {
                    new_pattern.vertices.insert(
                        edge.start_v_id,
                        PatternVertex {
                            id: edge.start_v_id,
                            label: edge.start_v_label,
                            index: 0,
                            connect_edges: BTreeMap::from([(edge.id, (edge.end_v_id, Direction::Out))]),
                            connect_vertices: BTreeMap::from([(
                                edge.end_v_id,
                                vec![(edge.id, Direction::Out)],
                            )]),
                            out_degree: 1,
                            in_degree: 0,
                        },
                    );
                    let vertex_set = new_pattern
                        .vertex_label_map
                        .entry(edge.start_v_label)
                        .or_insert(BTreeSet::new());
                    vertex_set.insert(edge.start_v_id);
                }
            }
            match new_pattern.vertices.get_mut(&edge.end_v_id) {
                // the end vertex existed, just update the connection info
                Some(end_vertex) => {
                    end_vertex
                        .connect_edges
                        .insert(edge.id, (edge.start_v_id, Direction::Incoming));
                    let end_vertex_connect_vertices_vec = end_vertex
                        .connect_vertices
                        .entry(edge.start_v_id)
                        .or_insert(Vec::new());
                    end_vertex_connect_vertices_vec.push((edge.id, Direction::Incoming));
                    end_vertex.in_degree += 1;
                }
                // the end vertex not existed, add the new Pattern
                None => {
                    new_pattern.vertices.insert(
                        edge.end_v_id,
                        PatternVertex {
                            id: edge.end_v_id,
                            label: edge.end_v_label,
                            index: 0,
                            connect_edges: BTreeMap::from([(
                                edge.id,
                                (edge.start_v_id, Direction::Incoming),
                            )]),
                            connect_vertices: BTreeMap::from([(
                                edge.start_v_id,
                                vec![(edge.id, Direction::Incoming)],
                            )]),
                            out_degree: 0,
                            in_degree: 1,
                        },
                    );
                    let vertex_set = new_pattern
                        .vertex_label_map
                        .entry(edge.end_v_label)
                        .or_insert(BTreeSet::new());
                    vertex_set.insert(edge.end_v_id);
                }
            }
        }
        new_pattern
    }
}

#[cfg(test)]
mod tests {
    use super::Direction;
    use super::Pattern;
    use super::PatternEdge;
    use super::PatternVertex;

    fn build_pattern_case1() -> Pattern {
        let pattern_edge1 =
            PatternEdge { id: 0, label: 0, start_v_id: 0, end_v_id: 1, start_v_label: 0, end_v_label: 0 };
        let pattern_edge2 =
            PatternEdge { id: 1, label: 0, start_v_id: 1, end_v_id: 0, start_v_label: 0, end_v_label: 0 };
        let pattern_vec = vec![pattern_edge1, pattern_edge2];
        Pattern::from(pattern_vec)
    }

    #[test]
    fn test_pattern_case1_structure() {
        let pattern_case1 = build_pattern_case1();
        let edges_num = pattern_case1.edges.len();
        assert_eq!(edges_num, 2);
        let vertices_num = pattern_case1.vertices.len();
        assert_eq!(vertices_num, 2);
        let edges_with_label_0 = pattern_case1.edge_label_map.get(&0).unwrap();
        assert_eq!(edges_with_label_0.len(), 2);
        let mut edges_with_label_0_iter = edges_with_label_0.iter();
        assert_eq!(*edges_with_label_0_iter.next().unwrap(), 0);
        assert_eq!(*edges_with_label_0_iter.next().unwrap(), 1);
        let vertices_with_label_0 = pattern_case1.vertex_label_map.get(&0).unwrap();
        assert_eq!(vertices_with_label_0.len(), 2);
        let mut vertices_with_label_0_iter = vertices_with_label_0.iter();
        assert_eq!(*vertices_with_label_0_iter.next().unwrap(), 0);
        assert_eq!(*vertices_with_label_0_iter.next().unwrap(), 1);
        let edge_0 = pattern_case1.edges.get(&0).unwrap();
        assert_eq!(edge_0.id, 0);
        assert_eq!(edge_0.label, 0);
        assert_eq!(edge_0.start_v_id, 0);
        assert_eq!(edge_0.end_v_id, 1);
        assert_eq!(edge_0.start_v_label, 0);
        assert_eq!(edge_0.end_v_label, 0);
        let edge_1 = pattern_case1.edges.get(&1).unwrap();
        assert_eq!(edge_1.id, 1);
        assert_eq!(edge_1.label, 0);
        assert_eq!(edge_1.start_v_id, 1);
        assert_eq!(edge_1.end_v_id, 0);
        assert_eq!(edge_1.start_v_label, 0);
        assert_eq!(edge_1.end_v_label, 0);
        let vertex_0 = pattern_case1.vertices.get(&0).unwrap();
        assert_eq!(vertex_0.id, 0);
        assert_eq!(vertex_0.label, 0);
        assert_eq!(vertex_0.connect_edges.len(), 2);
        let mut vertex_0_connect_edges_iter = vertex_0.connect_edges.iter();
        let (v0_e0, (v0_v0, v0_d0)) = vertex_0_connect_edges_iter.next().unwrap();
        assert_eq!(*v0_e0, 0);
        assert_eq!(*v0_v0, 1);
        assert_eq!(*v0_d0, Direction::Out);
        let (v0_e1, (v0_v1, v0_d1)) = vertex_0_connect_edges_iter.next().unwrap();
        assert_eq!(*v0_e1, 1);
        assert_eq!(*v0_v1, 1);
        assert_eq!(*v0_d1, Direction::Incoming);
        assert_eq!(vertex_0.connect_vertices.len(), 1);
        let v0_v1_connected_edges = vertex_0.connect_vertices.get(&1).unwrap();
        assert_eq!(v0_v1_connected_edges.len(), 2);
        let mut v0_v1_connected_edges_iter = v0_v1_connected_edges.iter();
        assert_eq!(*v0_v1_connected_edges_iter.next().unwrap(), (0, Direction::Out));
        assert_eq!(*v0_v1_connected_edges_iter.next().unwrap(), (1, Direction::Incoming));
        let vertex_1 = pattern_case1.vertices.get(&1).unwrap();
        assert_eq!(vertex_1.id, 1);
        assert_eq!(vertex_1.label, 0);
        assert_eq!(vertex_1.connect_edges.len(), 2);
        let mut vertex_1_connect_edges_iter = vertex_1.connect_edges.iter();
        let (v1_e0, (v1_v0, v1_d0)) = vertex_1_connect_edges_iter.next().unwrap();
        assert_eq!(*v1_e0, 0);
        assert_eq!(*v1_v0, 0);
        assert_eq!(*v1_d0, Direction::Incoming);
        let (v1_e1, (v1_v1, v1_d1)) = vertex_1_connect_edges_iter.next().unwrap();
        assert_eq!(*v1_e1, 1);
        assert_eq!(*v1_v1, 0);
        assert_eq!(*v1_d1, Direction::Out);
        assert_eq!(vertex_1.connect_vertices.len(), 1);
        let v1_v0_connected_edges = vertex_1.connect_vertices.get(&0).unwrap();
        assert_eq!(v1_v0_connected_edges.len(), 2);
        let mut v1_v0_connected_edges_iter = v1_v0_connected_edges.iter();
        assert_eq!(*v1_v0_connected_edges_iter.next().unwrap(), (0, Direction::Incoming));
        assert_eq!(*v1_v0_connected_edges_iter.next().unwrap(), (1, Direction::Out));
    }
}
