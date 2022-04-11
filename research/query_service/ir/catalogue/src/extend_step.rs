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

use std::collections::BTreeMap;

use super::pattern::Direction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtendEdge {
    pub start_v_label: u64,
    pub start_v_index: u64,
    pub target_v_label: u64,
    pub edge_label: u64,
    pub dir: Direction,
}

#[derive(Debug, Clone)]
pub struct ExtendStep {
    pub target_v_label: u64,
    // extend edges are classified by their start_v_labels and start_v_indices
    pub extend_edges: BTreeMap<(u64, u64), Vec<ExtendEdge>>,
}

impl From<Vec<ExtendEdge>> for ExtendStep {
    fn from(edges: Vec<ExtendEdge>) -> ExtendStep {
        if edges.len() == 0 {
            panic!("There should be at least one extend edge for a extend step!")
        }
        let target_v_label = edges[0].target_v_label;
        let mut new_extend_step = ExtendStep { target_v_label, extend_edges: BTreeMap::new() };
        for edge in edges {
            if edge.target_v_label != target_v_label {
                panic!("All extend edge should have the same target vertex label!")
            }
            let edge_vec = new_extend_step
                .extend_edges
                .entry((edge.start_v_label, edge.start_v_index))
                .or_insert(Vec::new());
            edge_vec.push(edge);
        }
        new_extend_step
    }
}

mod tests {
    use super::Direction;
    use super::ExtendEdge;
    use super::ExtendStep;

    fn build_extend_step_case1() -> ExtendStep {
        let extend_edge0 = ExtendEdge {
            start_v_label: 1,
            start_v_index: 0,
            target_v_label: 1,
            edge_label: 1,
            dir: Direction::Out,
        };
        let extend_edge1 = extend_edge0.clone();
        ExtendStep::from(vec![extend_edge0, extend_edge1])
    }

    #[test]
    fn test_extend_step_case1_structure() {
        let extend_step1 = build_extend_step_case1();
        assert_eq!(extend_step1.target_v_label, 1);
        assert_eq!(extend_step1.extend_edges.len(), 1);
        assert_eq!(
            extend_step1
                .extend_edges
                .get(&(1, 0))
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            extend_step1.extend_edges.get(&(1, 0)).unwrap()[0],
            ExtendEdge {
                start_v_label: 1,
                start_v_index: 0,
                target_v_label: 1,
                edge_label: 1,
                dir: Direction::Out
            }
        );
        assert_eq!(
            extend_step1.extend_edges.get(&(1, 0)).unwrap()[1],
            ExtendEdge {
                start_v_label: 1,
                start_v_index: 0,
                target_v_label: 1,
                edge_label: 1,
                dir: Direction::Out
            }
        );
    }
}
