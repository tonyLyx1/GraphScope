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

use std::cmp::{Ordering, min, max};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use crate::extend_step::{ExtendEdge, ExtendStep};
use crate::encoder::*;
use crate::codec::{Encode, Decode};
use crate::pattern_vertex::*;
use crate::pattern_edge::*;
use fast_math::{log2};

/// 边的方向：正向，反向或双向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Out = 0,
    Incoming = 1,
    Bothway = 2,
}

impl Direction {
    pub fn into_u8(&self) -> u8 {
        match *self {
            Direction::Out => 0,
            Direction::Incoming => 1,
            Direction::Bothway => 2,
            _ => panic!("Error in Converting Direction Enum Type to U8")
        }
    }
}

/// Pattern的全部信息，包含所有的点，边信息
/// 
/// edge_label_map: 拥有相同label的边的id集合
/// 
/// vertex_label_map: 拥有相同label的点的id集合
#[derive(Debug, Clone)]
pub struct Pattern {
    edges: BTreeMap<u64, PatternEdge>,
    vertices: BTreeMap<u64, PatternVertex>,
    edge_label_map: HashMap<u64, BTreeSet<u64>>,
    vertex_label_map: HashMap<u64, BTreeSet<u64>>,
}

/// Other Functions
impl Pattern {
    pub fn to_encode_unit_by_edge_id(&self, edge_id: u64) -> EncodeUnit {
        let edge = self.get_edge_from_id(edge_id);
        let (start_v_label, end_v_label) = edge.get_edge_vertices_label();
        let (start_v_index, end_v_index) = self.get_edge_vertices_order(edge_id);

        EncodeUnit::new(
            edge.get_edge_label(),
            start_v_label,
            end_v_label,
            Direction::Out,
            start_v_index,
            end_v_index,
        )
    }

    pub fn cmp_edges(&self, e1: &PatternEdge, e2: &PatternEdge) -> Ordering {
        match e1.cmp(e2) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            _ => ()
        }

        // Get orders for starting vertex
        let (e1_start_v_order, e1_end_v_order) = self.get_edge_vertices_order(e1.get_edge_id());
        let (e2_start_v_order, e2_end_v_order) = self.get_edge_vertices_order(e2.get_edge_id());
        // Compare the order of the starting vertex
        match e1_start_v_order.cmp(&e2_start_v_order) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            _ => (),
        }
        // Compare the order of ending vertex
        match e1_end_v_order.cmp(&e2_end_v_order) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            _ => (),
        }

        // Return as equal if still cannot distinguish
        Ordering::Equal
    }
}

/// Index Ranking
impl Pattern {
    fn reorder_label_vertices(&mut self, v_label: u64) {
        // To Be Completed
    }

    fn reorder_vertices(&mut self) {
        let mut v_labels = Vec::with_capacity(self.vertex_label_map.len());
        for (v_label, _) in &self.vertex_label_map {
            v_labels.push(*v_label)
        }
        for v_label in v_labels {
            self.reorder_label_vertices(v_label)
        }
    }

    /// ### Set Initial Vertex Index Based on Comparison of Labels and In/Out Degrees
    fn set_initial_index(&mut self) {
        for (_, vertex_set) in self.vertex_label_map.iter() {
            let mut vertex_vec = Vec::<u64>::with_capacity(vertex_set.len());
            let mut vertex_set_iter = vertex_set.iter();
            loop {
                match vertex_set_iter.next() {
                    Some(v_id) => {
                        vertex_vec.push(*v_id);
                        println!("v_id: {}", v_id);
                    },
                    None => break
                }
            }

            // vertex_vec.sort_by(|v1_id, v2_id| self.cmp_vertices(*v1_id, *v2_id));
            vertex_vec.sort_by(|v1_id, v2_id| self.get_vertex_from_id(*v1_id).cmp(self.get_vertex_from_id(*v2_id)));
            let mut vertex_index = 0;
            for v_id in vertex_vec.iter() {
                let vertex = self.vertices.get_mut(v_id).unwrap();
                vertex.set_vertex_index(vertex_index);
                vertex_index += 1;
            }
        }
    }

    /// ### Get a vector of ordered edges's indexes of a Pattern
    /// Get a vector of ordered edges's indexes of a Pattern
    /// The comparison is based on the `cmp_edges` method above to get the Order
    fn get_ordered_edges(&self) -> Vec<u64> {
        let mut ordered_edges = Vec::new();
        for (&edge, _) in &self.edges {
            ordered_edges.push(edge);
        }
        ordered_edges.sort_by(|e1_id, e2_id| self.cmp_edges(self.get_edge_from_id(*e1_id), self.get_edge_from_id(*e2_id)));
        ordered_edges
    }

    /// ### Get Vertex Order from Vertex Index Reference
    fn get_vertex_index(&self, vertex_index: &u64) -> u64 {
        let order = self
			.vertices
			.get(vertex_index)
			.unwrap()
			.index;
        order
    }
}

/// Public Functions of Pattern
impl Pattern {
    /// ### Get Edges References
    pub fn get_edges(&self) -> &BTreeMap<u64, PatternEdge> {
        &self.edges
    }

    /// ### Get Vertices References
    pub fn get_vertices(&self) -> &BTreeMap<u64, PatternVertex> {
        &self.vertices
    }

    /// ### Get PatternEdge Reference from Edge ID
    pub fn get_edge_from_id(&self, edge_id: u64) -> &PatternEdge {
        self.edges.get(&edge_id).unwrap()
    }

    /// ### Get PatternVertex Reference from Vertex ID
    pub fn get_vertex_from_id(&self, vertex_id: u64) -> &PatternVertex {
        self.vertices.get(&vertex_id).unwrap()
    }

    /// ### [Public] Get the order of both start and end vertices of an edge
    pub fn get_edge_vertices_order(&self, edge_index: u64) -> (u64, u64) {
        let edge = self.get_edge_from_id(edge_index);
        let start_v_order = self.get_vertex_index(&edge.start_v_id);
        let end_v_order = self.get_vertex_index(&edge.end_v_id);
        (start_v_order, end_v_order)
    }

    /// ### Get the total number of edges in the pattern
    pub fn get_edge_num(&self) -> u64 {
        self.edges.len() as u64
    }

    /// ### Get the total number of vertices in the pattern
    pub fn get_vertex_num(&self) -> u64 {
        println!("Vertex Num: {}", self.vertices.len());
        self.vertices.len() as u64
    }

    /// ### Get the total number of edge labels in the pattern
    pub fn get_edge_label_num(&self) -> u64 {
        self.edge_label_map.len() as u64
    }

    /// ### Compute at least how many bits are needed to represent edge labels
    /// At least 1 bit
    pub fn get_min_edge_label_bit_num(&self) -> u8 {
        max(1, log2(self.get_edge_num() as f32).ceil() as u8)
    }

    /// ### Get the total number of vertex labels in the pattern
    pub fn get_vertex_label_num(&self) -> u64 {
        self.vertex_label_map.len() as u64
    }

    /// ### Compute at least how many bits are needed to represent vertex labels
    /// At least 1 bit
    pub fn get_min_vertex_label_bit_num(&self) -> u8 {
        max(1, log2(self.get_vertex_num() as f32).ceil() as u8)
    }

    /// ### Compute at least how many bits are needed to represent vertices with the same label
    /// At least 1 bit
    pub fn get_min_vertex_index_bit_num(&self) -> u8 {
        // iterate through the hashmap and compute how many vertices have the same label in one set
        let mut min_index_bit_num: u8 = 1;
        for (_, value) in self.vertex_label_map.iter() {
            let same_label_vertex_num = value.len() as u64;
            let index_bit_num: u8 = log2(same_label_vertex_num as f32).ceil() as u8;
            if index_bit_num > min_index_bit_num {
                min_index_bit_num = index_bit_num;
            }
        }

        println!("Min Index Bit Num: {}", min_index_bit_num);
        min_index_bit_num
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
        let target_v_label = extend_step.get_target_v_label();
        let mut new_pattern_vertex = PatternVertex {
            id: new_pattern.get_next_pattern_vertex_id(),
            label: target_v_label,
            index: 0,
            connect_edges: BTreeMap::new(),
            connect_vertices: BTreeMap::new(),
            out_degree: 0,
            in_degree: 0,
        };
        for ((v_label, v_index), extend_edges) in extend_step.get_extend_edges() {
            // Get all the vertices which can be used to extend with these extend edges
            let vertices_can_use = self.get_equivalent_vertices(*v_label, *v_index);
            // There's no enough vertices to extend, just return None
            if vertices_can_use.len() < extend_edges.len() {
                return None;
            }
            // Connect each vertex can be use to each extend edge
            for i in 0..extend_edges.len() {
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
                        // Add the new pattern edge info to the new Pattern
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
                    Direction::Bothway => {
                        // To Be Completed
                    }
                    _ => {
                        panic!("Error in extend step: invalid Direction Enum Value");
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
            match new_pattern
                .vertices
                .get_mut(&edge.start_v_id)
            {
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

            // Add or update the end vertex to the new Pattern
            match new_pattern
                .vertices
                .get_mut(&edge.end_v_id)
            {
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
#[path = "./tests/pattern.rs"]
mod unit_test;
