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

use std::collections::btree_map::Iter as ExtendStepIter;
use std::collections::{BTreeMap, VecDeque};

use crate::catalogue::{Direction, Index, LabelId};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtendEdge {
    start_v_label: LabelId,
    start_v_index: Index,
    edge_label: LabelId,
    dir: Direction,
}

/// Initializer of ExtendEdge
impl ExtendEdge {
    pub fn new(
        start_v_label: LabelId, start_v_index: Index, edge_label: LabelId, dir: Direction,
    ) -> ExtendEdge {
        ExtendEdge { start_v_label, start_v_index, edge_label, dir }
    }
}

/// Methods for access fields of PatternEdge
impl ExtendEdge {
    pub fn get_start_vertex_label(&self) -> LabelId {
        self.start_v_label
    }

    pub fn get_start_vertex_index(&self) -> Index {
        self.start_v_index
    }

    pub fn get_edge_label(&self) -> LabelId {
        self.edge_label
    }

    pub fn get_direction(&self) -> Direction {
        self.dir
    }
}

#[derive(Debug, Clone)]
pub struct ExtendStep {
    target_v_label: LabelId,
    /// Key: (start vertex label, start vertex index), Value: Vec<extend edge>
    /// Extend edges are classified by their start_v_labels and start_v_indices
    extend_edges: BTreeMap<(LabelId, Index), Vec<ExtendEdge>>,
}

/// Initializer of ExtendStep
impl From<(LabelId, Vec<ExtendEdge>)> for ExtendStep {
    /// Initialization of a ExtendStep needs
    /// 1. a target vertex label
    /// 2. all extend edges connect to the target verex label
    fn from((target_v_label, edges): (LabelId, Vec<ExtendEdge>)) -> ExtendStep {
        let mut new_extend_step = ExtendStep { target_v_label, extend_edges: BTreeMap::new() };
        for edge in edges {
            let edge_vec = new_extend_step
                .extend_edges
                .entry((edge.start_v_label, edge.start_v_index))
                .or_insert(Vec::new());
            edge_vec.push(edge);
        }
        new_extend_step
    }
}

/// Methods for access fileds or get info from ExtendStep
impl ExtendStep {
    /// For the iteration over the extend edges of ExtendStep
    pub fn iter(&self) -> ExtendStepIter<(LabelId, Index), Vec<ExtendEdge>> {
        self.extend_edges.iter()
    }

    pub fn get_target_v_label(&self) -> LabelId {
        self.target_v_label
    }

    /// Given a source vertex label and index,
    /// check whether this ExtendStep contains a extend edge from this kind of vertex
    pub fn has_extend_from_start_v(&self, v_label: LabelId, v_index: Index) -> bool {
        self.extend_edges
            .contains_key(&(v_label, v_index))
    }

    /// Get how many different kind of start vertex this ExtendStep has
    pub fn get_diff_start_v_num(&self) -> usize {
        self.extend_edges.len()
    }

    pub fn get_extend_edges_num(&self) -> usize {
        let mut edges_num = 0;
        for (_, edges) in &self.extend_edges {
            edges_num += edges.len()
        }
        edges_num
    }

    /// Given a source vertex label and index, find all extend edges connect to this kind of vertices
    pub fn get_extend_edges_by_start_v(
        &self, v_label: LabelId, v_index: Index,
    ) -> Option<&Vec<ExtendEdge>> {
        self.extend_edges.get(&(v_label, v_index))
    }
}

/// Get all the subsets of given Vec<T>
/// The algorithm is BFS
pub fn get_subsets<T: Clone>(origin_vec: Vec<T>) -> Vec<Vec<T>> {
    let n = origin_vec.len();
    let mut set_collections = Vec::with_capacity((2 as usize).pow(n as u32));
    let mut queue = VecDeque::new();
    for (i, element) in origin_vec.iter().enumerate() {
        queue.push_back((vec![element.clone()], i + 1));
    }
    while queue.len() > 0 {
        let (subset, max_index) = queue.pop_front().unwrap();
        set_collections.push(subset.clone());
        for i in max_index..n {
            let mut new_subset = subset.clone();
            new_subset.push(origin_vec[i].clone());
            queue.push_back((new_subset, i + 1));
        }
    }
    set_collections
}

#[cfg(test)]
mod tests {
    use crate::catalogue::extend_step::*;
    use crate::catalogue::test_cases::*;
    use crate::catalogue::Direction;

    #[test]
    fn test_extend_step_case1_structure() {
        let extend_step1 = build_extend_step_case1();
        assert_eq!(extend_step1.target_v_label, 1);
        assert_eq!(extend_step1.extend_edges.len(), 1);
        assert_eq!(
            extend_step1
                .extend_edges
                .get(&(0, 0))
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            extend_step1.extend_edges.get(&(0, 0)).unwrap()[0],
            ExtendEdge { start_v_label: 0, start_v_index: 0, edge_label: 1, dir: Direction::Out }
        );
        assert_eq!(
            extend_step1.extend_edges.get(&(0, 0)).unwrap()[1],
            ExtendEdge { start_v_label: 0, start_v_index: 0, edge_label: 1, dir: Direction::Out }
        );
    }
}
