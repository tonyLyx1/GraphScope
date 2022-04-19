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

use std::cmp::{max, Ordering};
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use fast_math::log2;

use super::extend_step::{ExtendEdge, ExtendStep};
use super::pattern_meta::PatternMeta;
use super::codec::PatternEdgeEncodeUnit;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Out = 0,
    Incoming,
}

#[derive(Debug, Clone)]
pub struct PatternVertex {
    id: i32,
    label: i32,
    index: i32,
    connect_edges: BTreeMap<i32, (i32, Direction)>,
    connect_vertices: BTreeMap<i32, Vec<(i32, Direction)>>,
    out_degree: i32,
    in_degree: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct PatternEdge {
    pub id: i32,
    pub label: i32,
    pub start_v_id: i32,
    pub end_v_id: i32,
    pub start_v_label: i32,
    pub end_v_label: i32,
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub edges: BTreeMap<i32, PatternEdge>,
    pub vertices: BTreeMap<i32, PatternVertex>,
    pub edge_label_map: HashMap<i32, BTreeSet<i32>>,
    pub vertex_label_map: HashMap<i32, BTreeSet<i32>>,
}

/// Public Functions of Pattern
impl Pattern {
    /// ### Get Edges References
    pub fn get_edges(&self) -> &BTreeMap<i32, PatternEdge> {
        &self.edges
    }

    /// ### Get Vertices References
    pub fn get_vertices(&self) -> &BTreeMap<i32, PatternVertex> {
        &self.vertices
    }

    /// ### Get PatternEdge Reference from Edge ID
    pub fn get_edge_from_id(&self, edge_id: i32) -> &PatternEdge {
        self.edges.get(&edge_id).unwrap()
    }

    /// ### Get PatternVertex Reference from Vertex ID
    pub fn get_vertex_from_id(&self, vertex_id: i32) -> &PatternVertex {
        self.vertices.get(&vertex_id).unwrap()
    }

    /// ### [Public] Get the order of both start and end vertices of an edge
    pub fn get_edge_vertices_order(&self, edge_index: i32) -> (i32, i32) {
        let edge = self.get_edge_from_id(edge_index);
        let start_v_order = self.get_vertex_index(&edge.start_v_id);
        let end_v_order = self.get_vertex_index(&edge.end_v_id);
        (start_v_order, end_v_order)
    }

    /// ### Get the total number of edges in the pattern
    pub fn get_edge_num(&self) -> usize {
        self.edges.len()
    }

    /// ### Get the total number of vertices in the pattern
    pub fn get_vertex_num(&self) -> usize {
        println!("Vertex Num: {}", self.vertices.len());
        self.vertices.len()
    }

    /// ### Get the total number of edge labels in the pattern
    pub fn get_edge_label_num(&self) -> usize {
        self.edge_label_map.len()
    }

    /// ### Compute at least how many bits are needed to represent edge labels
    /// At least 1 bit
    pub fn get_min_edge_label_bit_num(&self) -> u8 {
        max(1, log2(self.get_edge_num() as f32).ceil() as u8)
    }

    /// ### Get the total number of vertex labels in the pattern
    pub fn get_vertex_label_num(&self) -> usize {
        self.vertex_label_map.len()
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
    fn reorder_label_vertices(&mut self, _v_label: i32) {}

    pub fn reorder_vertices(&mut self) {
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
    fn cmp_edges(&self, e1_id: i32, e2_id: i32) -> Ordering {
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
    pub fn get_ordered_edges(&self) -> Vec<i32> {
        let mut ordered_edges = Vec::new();
        for (&edge, _) in &self.edges {
            ordered_edges.push(edge);
        }
        ordered_edges.sort_by(|e1_id, e2_id| self.cmp_edges(*e1_id, *e2_id));
        ordered_edges
    }

    /// ### Get Vertex Order from Vertex Index Reference
    fn get_vertex_index(&self, vertex_index: &i32) -> i32 {
        self.vertices.get(vertex_index).unwrap().index
    }

    pub fn get_edge_encode_unit_by_id(&self, edge_id: i32) -> PatternEdgeEncodeUnit {
        let edge = self.get_edge_from_id(edge_id);
        let start_v_label = edge.start_v_label;
        let end_v_label = edge.end_v_label;
        let (start_v_index, end_v_index) = self.get_edge_vertices_order(edge_id);

        PatternEdgeEncodeUnit::new(
            edge.label,
            start_v_label,
            end_v_label,
            Direction::Out,
            start_v_index,
            end_v_index,
        )
    }
}

impl Pattern {
    /// Get all the vertices(id) with the same vertex label and vertex index
    /// These vertices are equivalent in the Pattern
    fn get_equivalent_vertices(&self, v_label: i32, v_index: i32) -> Vec<i32> {
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
    fn get_next_pattern_vertex_id(&self) -> i32 {
        let mut new_vertex_id = self.vertices.len() as i32;
        while self.vertices.contains_key(&new_vertex_id) {
            new_vertex_id += 1;
        }
        new_vertex_id
    }

    /// Get the legal id for the future incoming vertex
    fn get_next_pattern_edge_id(&self) -> i32 {
        let mut new_edge_id = self.edges.len() as i32;
        while self.edges.contains_key(&new_edge_id) {
            new_edge_id += 1;
        }
        new_edge_id
    }

    /// Extend the current Pattern to a new Pattern with the given ExtendStep
    /// If the ExtendStep is not matched with the current Pattern, the function will return None
    /// Else, it will return the new Pattern after the extension
    pub fn extend(&self, extend_step: ExtendStep) -> Option<Pattern> {
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

    pub fn get_extend_steps(&self, pattern_meta: &PatternMeta) -> Vec<ExtendStep> {
        let mut extend_steps = Vec::new();
        let target_vertices = pattern_meta.get_all_vertex_ids();
        for target_v_label in target_vertices {
            let mut extend_edgess = Vec::new();
            let mut extend_edges_with_src_id = Vec::new();
            for (_, src_vertex) in &self.vertices {
                let connect_edges =
                    pattern_meta.get_edges_between_vertices(src_vertex.label, target_v_label);
                for connect_edge in connect_edges {
                    let extend_edge = ExtendEdge {
                        start_v_label: src_vertex.label,
                        start_v_index: src_vertex.index,
                        edge_label: connect_edge.0,
                        dir: connect_edge.1,
                    };
                    extend_edges_with_src_id.push((extend_edge, src_vertex.id));
                }
            }
            let mut queue = VecDeque::new();
            for (i, extend_edge) in extend_edges_with_src_id.iter().enumerate() {
                queue.push_back((vec![extend_edge.clone()], i + 1));
            }
            while queue.len() > 0 {
                let (extend_edges_combinations, max_index) = queue.pop_front().unwrap();
                let mut extend_edges = Vec::with_capacity(extend_edges_combinations.len());
                for (extend_edge, _) in &extend_edges_combinations {
                    extend_edges.push(*extend_edge);
                }
                extend_edgess.push(extend_edges);
                'outer: for i in max_index..extend_edges_with_src_id.len() {
                    for (_, src_id) in &extend_edges_combinations {
                        if *src_id == extend_edges_with_src_id[i].1 {
                            continue 'outer;
                        }
                    }
                    let mut new_extend_edges_combinations = extend_edges_combinations.clone();
                    new_extend_edges_combinations.push(extend_edges_with_src_id[i]);
                    queue.push_back((new_extend_edges_combinations, i + 1));
                }
            }
            for extend_edges in extend_edgess {
                let extend_step = ExtendStep::from((target_v_label, extend_edges));
                extend_steps.push(extend_step);
            }
        }
        extend_steps
    }
}

// Initialize a Pattern containing only one vertex from hte vertex's id and label
impl From<(i32, i32)> for Pattern {
    fn from((vertex_id, vertex_label): (i32, i32)) -> Pattern {
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
    use std::fs::File;

    use ir_core::{plan::meta::Schema, JsonIO};

    use super::Direction;
    use super::Pattern;
    use super::PatternEdge;
    use super::PatternMeta;
    use super::{ExtendEdge, ExtendStep};

    /// The pattern looks like:
    /// A <-> A
    /// where <-> means two edges
    /// A's label's id is 0
    /// The edges's labels' id are both 0
    /// The left A has id 0
    /// The right A has id 1
    fn build_pattern_case1() -> Pattern {
        let pattern_edge1 =
            PatternEdge { id: 0, label: 0, start_v_id: 0, end_v_id: 1, start_v_label: 0, end_v_label: 0 };
        let pattern_edge2 =
            PatternEdge { id: 1, label: 0, start_v_id: 1, end_v_id: 0, start_v_label: 0, end_v_label: 0 };
        let pattern_vec = vec![pattern_edge1, pattern_edge2];
        Pattern::from(pattern_vec)
    }

    /// The pattern looks like:
    ///         B
    ///       /   \
    ///      A <-> A
    /// where <-> means two edges
    /// A's label id is 0, B's label id is 1
    /// The edges between two As have label id 0
    /// The edges between A and B have label id 1
    /// The left A has id 0
    /// The right A has id 1
    /// B has id 2
    fn build_pattern_case2() -> Pattern {
        let pattern_edge1 =
            PatternEdge { id: 0, label: 0, start_v_id: 0, end_v_id: 1, start_v_label: 0, end_v_label: 0 };
        let pattern_edge2 =
            PatternEdge { id: 1, label: 0, start_v_id: 1, end_v_id: 0, start_v_label: 0, end_v_label: 0 };
        let pattern_edge3 =
            PatternEdge { id: 2, label: 1, start_v_id: 0, end_v_id: 2, start_v_label: 0, end_v_label: 1 };
        let pattern_edge4 =
            PatternEdge { id: 3, label: 1, start_v_id: 1, end_v_id: 2, start_v_label: 0, end_v_label: 1 };
        let pattern_vec = vec![pattern_edge1, pattern_edge2, pattern_edge3, pattern_edge4];
        Pattern::from(pattern_vec)
    }

    /// The extend step looks like:
    ///         B
    ///       /   \
    ///      A     A
    /// The left A has label id 0 and index 0
    /// The right A also has label id 0 and index 0, the two A's are equivalent
    /// The target vertex is B with label id 1
    /// The two extend edges are both with edge id 1
    /// pattern_case1 + extend_step_case1 = pattern_case2
    fn build_extend_step_case1() -> ExtendStep {
        let extend_edge1 =
            ExtendEdge { start_v_label: 0, start_v_index: 0, edge_label: 1, dir: Direction::Out };
        let extend_edge2 = extend_edge1.clone();
        ExtendStep::from((1, vec![extend_edge1, extend_edge2]))
    }

    fn get_modern_pattern_meta() -> PatternMeta {
        let modern_schema_file = match File::open("resource/modern_schema.json") {
            Ok(file) => file,
            Err(_) => File::open("catalogue/resource/modern_schema.json").unwrap(),
        };
        let modern_schema = Schema::from_json(modern_schema_file).unwrap();
        PatternMeta::from(modern_schema)
    }

    /// Pattern from modern schema file
    /// Person only Pattern
    fn build_modern_pattern_case1() -> Pattern {
        Pattern::from((0, 0))
    }

    /// Software only Pattern
    fn build_modern_pattern_case2() -> Pattern {
        Pattern::from((0, 1))
    }

    /// Person -> knows -> Person
    fn build_modern_pattern_case3() -> Pattern {
        let pattern_edge =
            PatternEdge { id: 0, label: 0, start_v_id: 0, end_v_id: 1, start_v_label: 0, end_v_label: 0 };
        let mut pattern = Pattern::from(vec![pattern_edge]);
        pattern.vertices.get_mut(&1).unwrap().index = 1;
        pattern
    }

    /// Person -> created -> Software
    fn build_modern_pattern_case4() -> Pattern {
        let pattern_edge =
            PatternEdge { id: 0, label: 1, start_v_id: 0, end_v_id: 1, start_v_label: 0, end_v_label: 1 };
        Pattern::from(vec![pattern_edge])
    }

    fn get_ldbc_pattern_meta() -> PatternMeta {
        let ldbc_schema_file = match File::open("resource/ldbc_schema.json") {
            Ok(file) => file,
            Err(_) => File::open("catalogue/resource/ldbc_schema.json").unwrap(),
        };
        let ldbc_schema = Schema::from_json(ldbc_schema_file).unwrap();
        PatternMeta::from(ldbc_schema)
    }

    /// Pattern from ldbc schema file
    /// Person -> knows -> Person
    fn build_ldbc_pattern_case1() -> Pattern {
        let pattern_edge =
            PatternEdge { id: 0, label: 12, start_v_id: 0, end_v_id: 1, start_v_label: 1, end_v_label: 1 };
        let mut pattern = Pattern::from(vec![pattern_edge]);
        pattern.vertices.get_mut(&1).unwrap().index = 1;
        pattern
    }

    /// Test whether the structure of pattern_case1 is the same as our previous description
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

    /// Test whether pattern_case1 + extend_step_case1 = pattern_case2
    #[test]
    fn test_pattern_case1_extend() {
        let pattern_case1 = build_pattern_case1();
        let extend_step = build_extend_step_case1();
        let pattern_after_extend = pattern_case1.extend(extend_step).unwrap();
        assert_eq!(pattern_after_extend.edges.len(), 4);
        assert_eq!(pattern_after_extend.vertices.len(), 3);
        // Pattern after extend should be exactly the same as pattern case2
        let pattern_case2 = build_pattern_case2();
        assert_eq!(pattern_after_extend.edges.len(), pattern_case2.edges.len());
        for i in 0..pattern_after_extend.edges.len() as i32 {
            let edge1 = pattern_after_extend.edges.get(&i).unwrap();
            let edge2 = pattern_case2.edges.get(&i).unwrap();
            assert_eq!(edge1.id, edge2.id);
            assert_eq!(edge1.label, edge2.label);
            assert_eq!(edge1.start_v_id, edge2.start_v_id);
            assert_eq!(edge1.start_v_label, edge2.start_v_label);
            assert_eq!(edge1.end_v_id, edge2.end_v_id);
            assert_eq!(edge1.end_v_label, edge2.end_v_label);
        }
        assert_eq!(pattern_after_extend.edges.len(), pattern_case2.edges.len());
        for i in 0..pattern_after_extend.vertices.len() as i32 {
            let vertex1 = pattern_after_extend.vertices.get(&i).unwrap();
            let vertex2 = pattern_after_extend.vertices.get(&i).unwrap();
            assert_eq!(vertex1.id, vertex2.id);
            assert_eq!(vertex1.label, vertex2.label);
            assert_eq!(vertex1.index, vertex2.index);
            assert_eq!(vertex1.in_degree, vertex2.in_degree);
            assert_eq!(vertex1.out_degree, vertex2.out_degree);
            assert_eq!(vertex1.connect_edges.len(), vertex2.connect_edges.len());
            assert_eq!(vertex1.connect_vertices.len(), vertex2.connect_vertices.len());
            for (connect_edge1_id, (connect_vertex1_id, dir1)) in &vertex1.connect_edges {
                let (connect_vertex2_id, dir2) = vertex2
                    .connect_edges
                    .get(connect_edge1_id)
                    .unwrap();
                assert_eq!(*connect_vertex1_id, *connect_vertex2_id);
                assert_eq!(*dir1, *dir2);
            }
            for (connect_vertex1_id, edge_connections1) in &vertex1.connect_vertices {
                let edge_connections2 = vertex2
                    .connect_vertices
                    .get(connect_vertex1_id)
                    .unwrap();
                let (connect_edge1_id, dir1) = edge_connections1[0];
                let (connect_edge2_id, dir2) = edge_connections2[0];
                assert_eq!(connect_edge1_id, connect_edge2_id);
                assert_eq!(dir1, dir2);
            }
        }
        assert_eq!(pattern_after_extend.edge_label_map.len(), pattern_case2.edge_label_map.len());
        for i in 0..=1 {
            let edges_with_labeli_1 = pattern_after_extend
                .edge_label_map
                .get(&i)
                .unwrap();
            let edges_with_labeli_2 = pattern_case2.edge_label_map.get(&i).unwrap();
            assert_eq!(edges_with_labeli_1.len(), edges_with_labeli_2.len());
            let mut edges_with_labeli_1_iter = edges_with_labeli_1.iter();
            let mut edges_with_labeli_2_iter = edges_with_labeli_2.iter();
            let mut edges_with_labeli_1_element = edges_with_labeli_1_iter.next();
            let mut edges_with_labeli_2_element = edges_with_labeli_2_iter.next();
            while edges_with_labeli_1_element.is_some() {
                assert_eq!(*edges_with_labeli_1_element.unwrap(), *edges_with_labeli_2_element.unwrap());
                edges_with_labeli_1_element = edges_with_labeli_1_iter.next();
                edges_with_labeli_2_element = edges_with_labeli_2_iter.next();
            }
        }
        assert_eq!(pattern_after_extend.vertex_label_map.len(), pattern_case2.vertex_label_map.len());
        for i in 0..=1 {
            let vertices_with_labeli_1 = pattern_after_extend
                .vertex_label_map
                .get(&i)
                .unwrap();
            let vertices_with_labeli_2 = pattern_case2.vertex_label_map.get(&i).unwrap();
            assert_eq!(vertices_with_labeli_1.len(), vertices_with_labeli_2.len());
            let mut vertices_with_labeli_1_iter = vertices_with_labeli_1.iter();
            let mut vertices_with_labeli_2_iter = vertices_with_labeli_2.iter();
            let mut vertices_with_labeli_1_element = vertices_with_labeli_1_iter.next();
            let mut vertices_with_labeli_2_element = vertices_with_labeli_2_iter.next();
            while vertices_with_labeli_1_element.is_some() {
                assert_eq!(
                    *vertices_with_labeli_1_element.unwrap(),
                    *vertices_with_labeli_2_element.unwrap()
                );
                vertices_with_labeli_1_element = vertices_with_labeli_1_iter.next();
                vertices_with_labeli_2_element = vertices_with_labeli_2_iter.next();
            }
        }
    }

    #[test]
    fn test_get_extend_steps_of_modern_case1() {
        let modern_pattern_meta = get_modern_pattern_meta();
        let person_only_pattern = build_modern_pattern_case1();
        let all_extend_steps = person_only_pattern.get_extend_steps(&modern_pattern_meta);
        assert_eq!(all_extend_steps.len(), 3);
        let mut out_0_0_0 = 0;
        let mut incoming_0_0_0 = 0;
        let mut out_0_0_1 = 0;
        for extend_step in all_extend_steps {
            let extend_edges = extend_step.extend_edges.get(&(0, 0)).unwrap();
            assert_eq!(extend_edges.len(), 1);
            let extend_edge = extend_edges[0];
            assert_eq!(extend_edge.start_v_label, 0);
            assert_eq!(extend_edge.start_v_index, 0);
            if extend_step.target_v_label == 0 {
                if extend_edge.dir == Direction::Out {
                    out_0_0_0 += 1;
                }
                if extend_edge.dir == Direction::Incoming {
                    incoming_0_0_0 += 1;
                }
            }
            if extend_step.target_v_label == 1 && extend_edge.dir == Direction::Out {
                out_0_0_1 += 1;
            }
        }
        assert_eq!(out_0_0_0, 1);
        assert_eq!(incoming_0_0_0, 1);
        assert_eq!(out_0_0_1, 1);
    }

    #[test]
    fn test_get_extend_steps_of_modern_case2() {
        let modern_pattern_meta = get_modern_pattern_meta();
        let person_only_pattern = build_modern_pattern_case2();
        let all_extend_steps = person_only_pattern.get_extend_steps(&modern_pattern_meta);
        assert_eq!(all_extend_steps.len(), 1);
        assert_eq!(all_extend_steps[0].target_v_label, 0);
        assert_eq!(all_extend_steps[0].extend_edges.len(), 1);
        let extend_edge = all_extend_steps[0]
            .extend_edges
            .get(&(1, 0))
            .unwrap()[0];
        assert_eq!(extend_edge.start_v_label, 1);
        assert_eq!(extend_edge.start_v_index, 0);
        assert_eq!(extend_edge.edge_label, 1);
        assert_eq!(extend_edge.dir, Direction::Incoming);
    }

    #[test]
    fn test_get_extend_steps_of_modern_case3() {
        let modern_pattern_meta = get_modern_pattern_meta();
        let person_knows_person = build_modern_pattern_case3();
        let all_extend_steps = person_knows_person.get_extend_steps(&modern_pattern_meta);
        assert_eq!(all_extend_steps.len(), 11);
        let mut extend_steps_with_label_0_count = 0;
        let mut extend_steps_with_label_1_count = 0;
        let mut out_0_0_0_count = 0;
        let mut incoming_0_0_0_count = 0;
        let mut out_0_1_0_count = 0;
        let mut incoming_0_1_0_count = 0;
        let mut out_0_0_1_count = 0;
        let mut out_0_1_1_count = 0;
        let mut out_0_0_0_out_0_1_0_count = 0;
        let mut out_0_0_0_incoming_0_1_0_count = 0;
        let mut incoming_0_0_0_out_0_1_0_count = 0;
        let mut incoming_0_0_0_incoming_0_1_0_count = 0;
        let mut out_0_0_1_out_0_1_1_count = 0;
        for extend_step in all_extend_steps {
            if extend_step.target_v_label == 0 {
                extend_steps_with_label_0_count += 1;
                if extend_step.extend_edges.len() == 1 {
                    if extend_step.extend_edges.contains_key(&(0, 0)) {
                        let extend_edges = extend_step.extend_edges.get(&(0, 0)).unwrap();
                        for extend_edge in extend_edges {
                            assert_eq!(extend_edge.start_v_label, 0);
                            assert_eq!(extend_edge.start_v_index, 0);
                            if extend_edge.dir == Direction::Out && extend_edge.edge_label == 0 {
                                out_0_0_0_count += 1;
                            }
                            if extend_edge.dir == Direction::Incoming && extend_edge.edge_label == 0 {
                                incoming_0_0_0_count += 1
                            }
                            if extend_edge.dir == Direction::Out && extend_edge.edge_label == 1 {
                                out_0_0_1_count += 1;
                            }
                            if extend_edge.dir == Direction::Out && extend_edge.edge_label == 1 {
                                out_0_1_1_count += 1;
                            }
                        }
                    } else if extend_step.extend_edges.contains_key(&(0, 1)) {
                        let extend_edges = extend_step.extend_edges.get(&(0, 1)).unwrap();
                        for extend_edge in extend_edges {
                            assert_eq!(extend_edge.start_v_label, 0);
                            assert_eq!(extend_edge.start_v_index, 1);
                            if extend_edge.dir == Direction::Incoming && extend_edge.edge_label == 0 {
                                out_0_1_0_count += 1;
                            }
                            if extend_edge.dir == Direction::Incoming && extend_edge.edge_label == 0 {
                                incoming_0_1_0_count += 1;
                            }
                            if extend_edge.dir == Direction::Out && extend_edge.edge_label == 1 {
                                out_0_0_1_count += 1;
                            }
                            if extend_edge.dir == Direction::Out && extend_edge.edge_label == 1 {
                                out_0_1_1_count += 1;
                            }
                        }
                    }
                } else if extend_step.extend_edges.len() == 2 {
                    let mut found_out_0_0_0 = false;
                    let mut found_incoming_0_0_0 = false;
                    let mut found_out_0_1_0 = false;
                    let mut found_incoming_0_1_0 = false;
                    if extend_step.extend_edges.contains_key(&(0, 0)) {
                        let extend_edges = extend_step.extend_edges.get(&(0, 0)).unwrap();
                        for extend_edge in extend_edges {
                            assert_eq!(extend_edge.start_v_label, 0);
                            assert_eq!(extend_edge.start_v_index, 0);
                            if extend_edge.dir == Direction::Out && extend_edge.edge_label == 0 {
                                found_out_0_0_0 = true;
                            } else if extend_edge.dir == Direction::Incoming && extend_edge.edge_label == 0
                            {
                                found_incoming_0_0_0 = true;
                            }
                        }
                    }
                    if extend_step.extend_edges.contains_key(&(0, 1)) {
                        let extend_edges = extend_step.extend_edges.get(&(0, 1)).unwrap();
                        for extend_edge in extend_edges {
                            assert_eq!(extend_edge.start_v_label, 0);
                            assert_eq!(extend_edge.start_v_index, 1);
                            if extend_edge.dir == Direction::Out && extend_edge.edge_label == 0 {
                                found_out_0_1_0 = true;
                            } else if extend_edge.dir == Direction::Incoming && extend_edge.edge_label == 0
                            {
                                found_incoming_0_1_0 = true;
                            }
                        }
                    }
                    if found_out_0_0_0 && found_out_0_1_0 {
                        out_0_0_0_out_0_1_0_count += 1;
                    } else if found_out_0_0_0 && found_incoming_0_1_0 {
                        out_0_0_0_incoming_0_1_0_count += 1;
                    } else if found_incoming_0_0_0 && found_out_0_1_0 {
                        incoming_0_0_0_out_0_1_0_count += 1;
                    } else if found_incoming_0_0_0 && found_incoming_0_1_0 {
                        incoming_0_0_0_incoming_0_1_0_count += 1;
                    }
                }
            } else if extend_step.target_v_label == 1 {
                extend_steps_with_label_1_count += 1;
                if extend_step.extend_edges.len() == 1 {
                    if extend_step.extend_edges.contains_key(&(0, 0)) {
                        let extend_edges = extend_step.extend_edges.get(&(0, 0)).unwrap();
                        for extend_edge in extend_edges {
                            assert_eq!(extend_edge.start_v_label, 0);
                            assert_eq!(extend_edge.start_v_index, 0);
                            if extend_edge.dir == Direction::Out && extend_edge.edge_label == 1 {
                                out_0_0_1_count += 1;
                            }
                            if extend_edge.dir == Direction::Out && extend_edge.edge_label == 0 {
                                out_0_0_0_count += 1;
                            }
                            if extend_edge.dir == Direction::Incoming && extend_edge.edge_label == 0 {
                                incoming_0_0_0_count += 1
                            }
                        }
                    } else if extend_step.extend_edges.contains_key(&(0, 1)) {
                        let extend_edges = extend_step.extend_edges.get(&(0, 1)).unwrap();
                        for extend_edge in extend_edges {
                            assert_eq!(extend_edge.start_v_label, 0);
                            assert_eq!(extend_edge.start_v_index, 1);
                            if extend_edge.dir == Direction::Out && extend_edge.edge_label == 1 {
                                out_0_1_1_count += 1;
                            }
                            if extend_edge.dir == Direction::Out && extend_edge.edge_label == 0 {
                                out_0_0_0_count += 1;
                            }
                            if extend_edge.dir == Direction::Incoming && extend_edge.edge_label == 0 {
                                incoming_0_0_0_count += 1
                            }
                        }
                    }
                } else if extend_step.extend_edges.len() == 2 {
                    let mut found_out_0_0_1 = false;
                    let mut found_out_0_1_1 = false;
                    if extend_step.extend_edges.contains_key(&(0, 0)) {
                        let extend_edges = extend_step.extend_edges.get(&(0, 0)).unwrap();
                        for extend_edge in extend_edges {
                            assert_eq!(extend_edge.start_v_label, 0);
                            assert_eq!(extend_edge.start_v_index, 0);
                            if extend_edge.dir == Direction::Out && extend_edge.edge_label == 1 {
                                found_out_0_0_1 = true;
                            }
                        }
                    }
                    if extend_step.extend_edges.contains_key(&(0, 1)) {
                        let extend_edges = extend_step.extend_edges.get(&(0, 1)).unwrap();
                        for extend_edge in extend_edges {
                            assert_eq!(extend_edge.start_v_label, 0);
                            assert_eq!(extend_edge.start_v_index, 1);
                            if extend_edge.dir == Direction::Out && extend_edge.edge_label == 1 {
                                found_out_0_1_1 = true;
                            }
                        }
                    }
                    if found_out_0_0_1 && found_out_0_1_1 {
                        out_0_0_1_out_0_1_1_count += 1;
                    }
                }
            }
        }
        assert_eq!(extend_steps_with_label_0_count, 8);
        assert_eq!(extend_steps_with_label_1_count, 3);
        assert_eq!(out_0_0_0_count, 1);
        assert_eq!(incoming_0_0_0_count, 1);
        assert_eq!(out_0_1_0_count, 1);
        assert_eq!(incoming_0_1_0_count, 1);
        assert_eq!(out_0_0_1_count, 1);
        assert_eq!(out_0_1_1_count, 1);
        assert_eq!(out_0_0_0_out_0_1_0_count, 1);
        assert_eq!(out_0_0_0_incoming_0_1_0_count, 1);
        assert_eq!(incoming_0_0_0_out_0_1_0_count, 1);
        assert_eq!(incoming_0_0_0_incoming_0_1_0_count, 1);
        assert_eq!(out_0_0_1_out_0_1_1_count, 1);
    }

    #[test]
    fn test_get_extend_steps_of_modern_case4() {
        let modern_pattern_meta = get_modern_pattern_meta();
        let person_created_software = build_modern_pattern_case4();
        let all_extend_steps = person_created_software.get_extend_steps(&modern_pattern_meta);
        assert_eq!(all_extend_steps.len(), 6);
        let mut extend_steps_with_label_0_count = 0;
        let mut extend_steps_with_label_1_count = 0;
        let mut out_0_0_0_count = 0;
        let mut incoming_0_0_0_count = 0;
        let mut incoming_1_0_1_count = 0;
        let mut out_0_0_0_incoming_1_0_1_count = 0;
        let mut incoming_0_0_0_incoming_1_0_1_count = 0;
        for extend_step in all_extend_steps {
            if extend_step.target_v_label == 0 {
                extend_steps_with_label_0_count += 1;
                if extend_step.extend_edges.len() == 1 {
                    if extend_step.extend_edges.contains_key(&(0, 0)) {
                        let extend_edges = extend_step.extend_edges.get(&(0, 0)).unwrap();
                        for extend_edge in extend_edges {
                            assert_eq!(extend_edge.start_v_label, 0);
                            assert_eq!(extend_edge.start_v_index, 0);
                            if extend_edge.dir == Direction::Out && extend_edge.edge_label == 0 {
                                out_0_0_0_count += 1;
                            }
                            if extend_edge.dir == Direction::Incoming && extend_edge.edge_label == 0 {
                                incoming_0_0_0_count += 1;
                            }
                            if extend_edge.dir == Direction::Incoming && extend_edge.edge_label == 1 {
                                incoming_1_0_1_count += 1;
                            }
                        }
                    } else if extend_step.extend_edges.contains_key(&(1, 0)) {
                        let extend_edges = extend_step.extend_edges.get(&(1, 0)).unwrap();
                        for extend_edge in extend_edges {
                            assert_eq!(extend_edge.start_v_label, 1);
                            assert_eq!(extend_edge.start_v_index, 0);
                            if extend_edge.dir == Direction::Out && extend_edge.edge_label == 0 {
                                out_0_0_0_count += 1;
                            }
                            if extend_edge.dir == Direction::Incoming && extend_edge.edge_label == 0 {
                                incoming_0_0_0_count += 1;
                            }
                            if extend_edge.dir == Direction::Incoming && extend_edge.edge_label == 1 {
                                incoming_1_0_1_count += 1;
                            }
                        }
                    }
                } else if extend_step.extend_edges.len() == 2 {
                    let mut found_out_0_0_0 = false;
                    let mut found_incoming_1_0_1 = false;
                    let mut found_incoming_0_0_0 = false;
                    if extend_step.extend_edges.contains_key(&(0, 0)) {
                        let extend_edges = extend_step.extend_edges.get(&(0, 0)).unwrap();
                        for extend_edge in extend_edges {
                            assert_eq!(extend_edge.start_v_label, 0);
                            assert_eq!(extend_edge.start_v_index, 0);
                            if extend_edge.dir == Direction::Out && extend_edge.edge_label == 0 {
                                found_out_0_0_0 = true;
                            } else if extend_edge.dir == Direction::Incoming && extend_edge.edge_label == 0
                            {
                                found_incoming_0_0_0 = true;
                            }
                        }
                    }
                    if extend_step.extend_edges.contains_key(&(1, 0)) {
                        let extend_edges = extend_step.extend_edges.get(&(1, 0)).unwrap();
                        for extend_edge in extend_edges {
                            assert_eq!(extend_edge.start_v_label, 1);
                            assert_eq!(extend_edge.start_v_index, 0);
                            if extend_edge.dir == Direction::Incoming && extend_edge.edge_label == 1 {
                                found_incoming_1_0_1 = true;
                            }
                        }
                    }
                    if found_out_0_0_0 && found_incoming_1_0_1 {
                        out_0_0_0_incoming_1_0_1_count += 1;
                    } else if found_incoming_0_0_0 && found_incoming_1_0_1 {
                        incoming_0_0_0_incoming_1_0_1_count += 1;
                    }
                }
            } else if extend_step.target_v_label == 1 {
                extend_steps_with_label_1_count += 1;
            }
        }
        assert_eq!(extend_steps_with_label_0_count, 5);
        assert_eq!(extend_steps_with_label_1_count, 1);
        assert_eq!(out_0_0_0_count, 1);
        assert_eq!(incoming_0_0_0_count, 1);
        assert_eq!(incoming_1_0_1_count, 1);
        assert_eq!(out_0_0_0_incoming_1_0_1_count, 1);
        assert_eq!(incoming_0_0_0_incoming_1_0_1_count, 1);
    }

    #[test]
    fn test_get_extend_steps_of_ldbc_case1() {
        let ldbc_pattern_meta = get_ldbc_pattern_meta();
        let person_knows_person = build_ldbc_pattern_case1();
        let all_extend_steps = person_knows_person.get_extend_steps(&ldbc_pattern_meta);
        println!("{:?}", all_extend_steps.len());
        println!("{:?}", all_extend_steps);
    }
}
