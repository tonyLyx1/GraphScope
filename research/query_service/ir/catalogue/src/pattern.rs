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
use crate::extend_step::{ExtendEdge, ExtendStep};

/// 边的方向：正向，反向或双向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Out = 0,
    Incoming = 1,
    Bothway = 2,
}

impl Direction {
    /// ### Convert a Direction Reference to u8
    pub fn to_u8(input: &Direction) -> u8 {
        match input {
            Direction::Out => 0,
            Direction::Incoming => 1,
            Direction::Bothway => 2,
            _ => panic!("Error in Converting Direction Enum Type to U8")
        }
    } 
}

/// 单个点的信息
/// 
/// Remark: 由于完全对称的点拥有相同的index，需使用order字段加以区分
#[derive(Debug, Clone)]
pub struct PatternVertex {
    id: u64,
    label: u64,
    index: u64,
    connect_edges: BTreeMap<u64, (u64, Direction)>,
    connect_vertices: BTreeMap<u64, Vec<(u64, Direction)>>,
    out_degree: u64,
    in_degree: u64,
}

impl PatternVertex {
    /// ### Get Vertex Index
    pub fn get_vertex_id(&self) -> u64 {
        self.id
    }

    /// ### Get Vertex Label
    pub fn get_vertex_label(&self) -> u64 {
        self.label
    }

    /// ### Get Vertex Order
    pub fn get_vertex_index(&self) -> u64 {
        self.index
    }
}

/// 单条边的信息
#[derive(Debug, Clone, Copy)]
pub struct PatternEdge {
    id: u64,
    label: u64,
    start_v_id: u64,
    end_v_id: u64,
    start_v_label: u64,
    end_v_label: u64,
}

impl PatternEdge {
    /// ### Create a New PatternEdge
    pub fn create(
        id: u64,
        label: u64,
        start_v_id: u64,
        end_v_id: u64,
        start_v_label: u64,
        end_v_label: u64
    ) -> PatternEdge {
        PatternEdge {
            id,
            label,
            start_v_id,
            end_v_id,
            start_v_label,
            end_v_label,
        }
    }


    /// ### Get Edge Label
    pub fn get_edge_label(&self) -> u64 {
        self.label
    }

    /// ### Get Edge Index
    pub fn get_edge_id(&self) -> u64 {
        self.id
    }

    /// ### Get the Indices of Both Start and End Vertices of the Edge
    pub fn get_edge_vertices_id(&self) -> (u64, u64) {
        (self.start_v_id, self.end_v_id)
    }

    /// ### Get the Labels of Both Start and End Vertices of the Edge
    pub fn get_edge_vertices_label(&self) -> (u64, u64) {
        (self.start_v_label, self.end_v_label)
    }
}

/// Pattern的全部信息，包含所有的点，边信息
/// 
/// edge_label_map: 拥有相同label的边的index集合
/// 
/// vertex_label_map: 拥有相同label的店的index集合
#[derive(Debug, Clone)]
pub struct Pattern {
    edges: BTreeMap<u64, PatternEdge>,
    vertices: BTreeMap<u64, PatternVertex>,
    edge_label_map: HashMap<u64, BTreeSet<u64>>,
    vertex_label_map: HashMap<u64, BTreeSet<u64>>,
}

/// Private Functions of Pattern
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

    /// ### Get the Order of two PatternEdges of a Pattern
    /// Order by Edge Label, Vertex Labels and Vertex Indices
    /// 
    /// Return equal if still cannot distinguish
    fn cmp_edges(&self, e1_index: u64, e2_index: u64) -> Ordering {
        // Get edges from BTreeMap
        let e1 = self.get_edge_from_edge_index(e1_index);
        let e2 = self.get_edge_from_edge_index(e2_index);
        // Compare the edge label
        match e1.label.cmp(&e2.label) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            _ => (),
        }
        // Compare the label of starting vertex
        match e1.start_v_label.cmp(&e2.start_v_label) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            _ => (),
        }
        // Compare the label of ending vertex
        match e1.end_v_label.cmp(&e2.end_v_label) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            _ => (),
        }
        // Get orders for starting vertex
        let (e1_start_v_order, e1_end_v_order) = self.get_edge_vertices_order(e1_index);
        let (e2_start_v_order, e2_end_v_order) = self.get_edge_vertices_order(e2_index);
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

    /// ### Get a vector of ordered edges's indexes of a Pattern
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

    /// ### [Public] Get PatternEdge Reference from Edge Index and Pattern Object
    pub fn get_edge_from_edge_index(&self, edge_index: u64) -> &PatternEdge {
        let edge = self.edges.get(&edge_index).unwrap();
        edge
    }

    /// ### [Public] Get the order of both start and end vertices of an edge
    pub fn get_edge_vertices_order(&self, edge_index: u64) -> (u64, u64) {
        let edge = self.get_edge_from_edge_index(edge_index);
        let start_v_order = self.get_vertex_index(&edge.start_v_id);
        let end_v_order = self.get_vertex_index(&edge.end_v_id);
        (start_v_order, end_v_order)
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
mod tests {
    use super::Direction;
    use super::Pattern;
    use super::PatternEdge;
    use super::PatternVertex;
    use super::{ExtendEdge, ExtendStep};
    use crate::pattern;

    fn build_pattern_case1() -> Pattern {
        let pattern_edge1 =
            PatternEdge { id: 0, label: 0, start_v_id: 0, end_v_id: 1, start_v_label: 0, end_v_label: 0 };
        let pattern_edge2 =
            PatternEdge { id: 1, label: 0, start_v_id: 1, end_v_id: 0, start_v_label: 0, end_v_label: 0 };
        let pattern_vec = vec![pattern_edge1, pattern_edge2];
        Pattern::from(pattern_vec)
    }

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

    fn build_extend_step_case1() -> ExtendStep {
        let extend_edge1 =
            ExtendEdge { start_v_label: 0, start_v_index: 0, edge_label: 1, dir: Direction::Out };
        let extend_edge2 = extend_edge1.clone();
        ExtendStep::from((1, vec![extend_edge1, extend_edge2]))
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
        for i in 0..pattern_after_extend.edges.len() as u64 {
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
        for i in 0..pattern_after_extend.vertices.len() as u64 {
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
}
