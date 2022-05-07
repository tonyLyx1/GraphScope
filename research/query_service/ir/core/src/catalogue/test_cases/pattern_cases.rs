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

use std::collections::HashMap;

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;

use crate::catalogue::pattern::*;
use crate::catalogue::{PatternId, PatternLabelId};

/// The pattern looks like:
/// A <-> A
/// where <-> means two edges
/// A's label's id is 0
/// The edges's labels' id are both 0
/// The left A has id 0
/// The right A has id 1
pub fn build_pattern_case1() -> Pattern {
    let pattern_vec = vec![PatternEdge::new(0, 0, 0, 1, 0, 0), PatternEdge::new(1, 0, 1, 0, 0, 0)];
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
pub fn build_pattern_case2() -> Pattern {
    let pattern_vec = vec![
        PatternEdge::new(0, 0, 0, 1, 0, 0),
        PatternEdge::new(1, 0, 1, 0, 0, 0),
        PatternEdge::new(2, 1, 0, 2, 0, 1),
        PatternEdge::new(3, 1, 1, 2, 0, 1),
    ];
    Pattern::from(pattern_vec)
}

/// The pattern looks like:
///     B(2) -> B(3)
///     |       |
///     A(0) -> A(1)
/// where <-> means two edges
/// Vertex Label Map:
///     A: 0, B: 1,
/// Edge Label Map:
///     A-A: 0, A->B: 1, B-B: 2,
pub fn build_pattern_case3() -> Pattern {
    let pattern_vec = vec![
        PatternEdge::new(gen_id(), 0, 0, 1, 0, 0),
        PatternEdge::new(gen_id(), 1, 0, 2, 0, 1),
        PatternEdge::new(gen_id(), 1, 1, 3, 0, 1),
        PatternEdge::new(gen_id(), 2, 2, 3, 1, 1),
    ];
    Pattern::from(pattern_vec)
}

/// The pattern looks like:
///     B(2)  -> B(3)
///     |        |
///     A(0) <-> A(1)
/// where <-> means two edges
/// Vertex Label Map:
///     A: 0, B: 1,
/// Edge Label Map:
///     A-A: 0, A->B: 1, B-B: 2,
pub fn build_pattern_case4() -> Pattern {
    let pattern_vec = vec![
        PatternEdge::new(gen_id(), 0, 0, 1, 0, 0),
        PatternEdge::new(gen_id(), 0, 1, 0, 0, 0),
        PatternEdge::new(gen_id(), 1, 0, 2, 0, 1),
        PatternEdge::new(gen_id(), 1, 1, 3, 0, 1),
        PatternEdge::new(gen_id(), 2, 2, 3, 1, 1),
    ];
    Pattern::from(pattern_vec)
}

/// The pattern looks like
///         A(0) -> B(0)    A(1) <-> A(2)
///         |               |
/// C(0) -> B(1) <- A(3) -> B(2) <- C(1) <- D(0)
///         |
///         C(2)
/// where <-> means bidirectional edges
/// Vertex Label Map
///     A: 3, B: 2, C: 1, D: 0
/// Edge Label Map:
///     A-A: 20, A-B: 30, B-C: 15, C-D: 5
pub fn build_pattern_case5() -> Pattern {
    let label_a = 3;
    let label_b = 2;
    let label_c = 1;
    let label_d = 0;
    let id_vec_a: Vec<PatternId> = vec![100, 200, 300, 400];
    let id_vec_b: Vec<PatternId> = vec![10, 20, 30];
    let id_vec_c: Vec<PatternId> = vec![1, 2, 3];
    let id_vec_d: Vec<PatternId> = vec![1000];
    let pattern_vec = vec![
        PatternEdge::new(gen_id(), 15, id_vec_c[0], id_vec_b[1], label_c, label_b),
        PatternEdge::new(gen_id(), 30, id_vec_a[0], id_vec_b[1], label_a, label_b),
        PatternEdge::new(gen_id(), 15, id_vec_c[2], id_vec_b[1], label_c, label_b),
        PatternEdge::new(gen_id(), 30, id_vec_a[0], id_vec_b[0], label_a, label_b),
        PatternEdge::new(gen_id(), 30, id_vec_a[3], id_vec_b[1], label_a, label_b),
        PatternEdge::new(gen_id(), 30, id_vec_a[3], id_vec_b[2], label_a, label_b),
        PatternEdge::new(gen_id(), 30, id_vec_a[1], id_vec_b[2], label_a, label_b),
        PatternEdge::new(gen_id(), 20, id_vec_a[1], id_vec_a[2], label_a, label_a),
        PatternEdge::new(gen_id(), 20, id_vec_a[2], id_vec_a[1], label_a, label_a),
        PatternEdge::new(gen_id(), 15, id_vec_c[1], id_vec_b[2], label_c, label_b),
        PatternEdge::new(gen_id(), 5, id_vec_d[0], id_vec_c[1], label_d, label_c),
    ];
    Pattern::from(pattern_vec)
}

/// The pattern looks like:
/// B <- A -> C
/// Vertex Label Map:
/// A: 1, B: 2, C: 3
/// Edge Label Map:
/// A->B: 1, A->C: 2
pub fn build_pattern_case6() -> Pattern {
    let pattern_edge1 = PatternEdge::new(0, 1, 0, 1, 1, 2);
    let pattern_edge2 = PatternEdge::new(1, 2, 0, 2, 1, 3);
    let pattern_vec = vec![pattern_edge1, pattern_edge2];
    Pattern::from(pattern_vec)
}

/// The pattern looks like:
///         A
///        /|\
///       / D \
///      //  \ \
///     B  ->  C
/// Vertex Label Map:
/// A: 1, B: 2, C: 3, D: 4
/// Edge Label Map:
/// A->B: 0, A->C: 1, B->C: 2, A->D: 3, B->D: 4, D->C: 5
pub fn build_pattern_case7() -> Pattern {
    let edge_1 = PatternEdge::new(0, 1, 0, 1, 1, 2);
    let edge_2 = PatternEdge::new(1, 2, 0, 2, 1, 3);
    let edge_3 = PatternEdge::new(2, 3, 1, 2, 2, 3);
    let edge_4 = PatternEdge::new(3, 4, 0, 3, 1, 4);
    let edge_5 = PatternEdge::new(4, 5, 1, 3, 2, 4);
    let edge_6 = PatternEdge::new(5, 6, 3, 2, 4, 3);
    let pattern_edges = vec![edge_1, edge_2, edge_3, edge_4, edge_5, edge_6];
    Pattern::from(pattern_edges)
}

/// Pattern from modern schema file
/// Person only Pattern
pub fn build_modern_pattern_case1() -> Pattern {
    Pattern::from((0, 0))
}

/// Software only Pattern
pub fn build_modern_pattern_case2() -> Pattern {
    Pattern::from((0, 1))
}

/// Person -> knows -> Person
pub fn build_modern_pattern_case3() -> Pattern {
    let pattern_edge = PatternEdge::new(0, 0, 0, 1, 0, 0);
    let mut pattern = Pattern::from(vec![pattern_edge]);
    pattern
        .get_vertex_mut_from_id(1)
        .unwrap()
        .set_index(1);
    pattern
}

/// Person -> created -> Software
pub fn build_modern_pattern_case4() -> Pattern {
    let pattern_edge = PatternEdge::new(0, 1, 0, 1, 0, 1);
    Pattern::from(vec![pattern_edge])
}

/// Test Cases for Index Ranking
fn gen_edge_label_map(edges: Vec<String>) -> HashMap<String, PatternLabelId> {
    let mut rng = StdRng::from_seed([0; 32]);
    let mut values: Vec<PatternLabelId> = (0..=255).collect();
    values.shuffle(&mut rng);
    let mut edge_label_map: HashMap<String, PatternLabelId> = HashMap::new();
    let mut index = 0;
    for edge in edges {
        if index >= values.len() {
            panic!("Error in gen_edge_label_map: index of out of scope");
        }
        edge_label_map.insert(edge, values[index]);
        index += 1;
    }

    edge_label_map
}

fn gen_id() -> PatternId {
    let mut rng = rand::thread_rng();
    (rng.gen::<PatternId>()).abs()
}

pub fn build_pattern_index_ranking_case1() -> (Pattern, HashMap<String, PatternId>) {
    let mut vertex_label_map: HashMap<String, PatternLabelId> = HashMap::new();
    let mut vertex_id_map: HashMap<String, PatternId> = HashMap::new();
    let edge_label_map: HashMap<String, PatternLabelId> =
        gen_edge_label_map(vec![String::from("A->A"), String::from("A->B")]);
    vertex_label_map.insert(String::from("A"), 1);
    vertex_id_map.insert(String::from("A0"), gen_id());
    vertex_id_map.insert(String::from("A1"), gen_id());
    let pattern_vec = vec![PatternEdge::new(
        gen_id(),
        *edge_label_map.get("A->A").unwrap(),
        *vertex_id_map.get("A0").unwrap(),
        *vertex_id_map.get("A1").unwrap(),
        *vertex_label_map.get("A").unwrap(),
        *vertex_label_map.get("A").unwrap(),
    )];
    (Pattern::from(pattern_vec), vertex_id_map)
}

pub fn build_pattern_index_ranking_case2() -> (Pattern, HashMap<String, PatternId>) {
    let mut vertex_label_map: HashMap<String, PatternLabelId> = HashMap::new();
    let mut vertex_id_map: HashMap<String, PatternId> = HashMap::new();
    let edge_label_map: HashMap<String, PatternLabelId> =
        gen_edge_label_map(vec![String::from("A->A"), String::from("A->B")]);
    vertex_label_map.insert(String::from("A"), 1);
    vertex_id_map.insert(String::from("A0"), gen_id());
    vertex_id_map.insert(String::from("A1"), gen_id());
    let pattern_vec = vec![
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
    ];
    (Pattern::from(pattern_vec), vertex_id_map)
}

pub fn build_pattern_index_ranking_case3() -> (Pattern, HashMap<String, PatternId>) {
    let mut vertex_label_map: HashMap<String, PatternLabelId> = HashMap::new();
    let mut vertex_id_map: HashMap<String, PatternId> = HashMap::new();
    let edge_label_map: HashMap<String, PatternLabelId> =
        gen_edge_label_map(vec![String::from("A->A"), String::from("A->B")]);
    vertex_label_map.insert(String::from("A"), 1);
    vertex_label_map.insert(String::from("B"), 2);
    vertex_id_map.insert(String::from("A0"), gen_id());
    vertex_id_map.insert(String::from("A1"), gen_id());
    vertex_id_map.insert(String::from("B0"), gen_id());
    let pattern_vec = vec![
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
    ];
    (Pattern::from(pattern_vec), vertex_id_map)
}

pub fn build_pattern_index_ranking_case4() -> (Pattern, HashMap<String, PatternId>) {
    let mut vertex_label_map: HashMap<String, PatternLabelId> = HashMap::new();
    let mut vertex_id_map: HashMap<String, PatternId> = HashMap::new();
    let edge_label_map: HashMap<String, PatternLabelId> =
        gen_edge_label_map(vec![String::from("A->A"), String::from("A->B")]);
    vertex_label_map.insert(String::from("A"), 1);
    vertex_label_map.insert(String::from("B"), 2);
    vertex_id_map.insert(String::from("A0"), gen_id());
    vertex_id_map.insert(String::from("A1"), gen_id());
    vertex_id_map.insert(String::from("A2"), gen_id());
    let pattern_vec = vec![
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("A2").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A2").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
    ];
    (Pattern::from(pattern_vec), vertex_id_map)
}

pub fn build_pattern_index_ranking_case5() -> (Pattern, HashMap<String, PatternId>) {
    let mut vertex_label_map: HashMap<String, PatternLabelId> = HashMap::new();
    let mut vertex_id_map: HashMap<String, PatternId> = HashMap::new();
    let edge_label_map: HashMap<String, PatternLabelId> =
        gen_edge_label_map(vec![String::from("A->A"), String::from("A->B")]);
    vertex_label_map.insert(String::from("A"), 1);
    vertex_label_map.insert(String::from("B"), 2);
    vertex_id_map.insert(String::from("A0"), gen_id());
    vertex_id_map.insert(String::from("A1"), gen_id());
    vertex_id_map.insert(String::from("A2"), gen_id());
    let pattern_vec = vec![
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("A2").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A2").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
    ];
    (Pattern::from(pattern_vec), vertex_id_map)
}

pub fn build_pattern_index_ranking_case6() -> (Pattern, HashMap<String, PatternId>) {
    let mut vertex_label_map: HashMap<String, PatternLabelId> = HashMap::new();
    let mut vertex_id_map: HashMap<String, PatternId> = HashMap::new();
    let edge_label_map: HashMap<String, PatternLabelId> =
        gen_edge_label_map(vec![String::from("A->A"), String::from("A->B"), String::from("B->A")]);
    vertex_label_map.insert(String::from("A"), 1);
    vertex_label_map.insert(String::from("B"), 2);
    vertex_id_map.insert(String::from("A0"), gen_id());
    vertex_id_map.insert(String::from("A1"), gen_id());
    vertex_id_map.insert(String::from("B0"), gen_id());
    let pattern_vec = vec![
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->A").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
    ];
    (Pattern::from(pattern_vec), vertex_id_map)
}

pub fn build_pattern_index_ranking_case7() -> (Pattern, HashMap<String, PatternId>) {
    let mut vertex_label_map: HashMap<String, PatternLabelId> = HashMap::new();
    let mut vertex_id_map: HashMap<String, PatternId> = HashMap::new();
    let edge_label_map: HashMap<String, PatternLabelId> =
        gen_edge_label_map(vec![String::from("A->A"), String::from("A->B")]);
    vertex_label_map.insert(String::from("A"), 1);
    vertex_label_map.insert(String::from("B"), 2);
    vertex_id_map.insert(String::from("A0"), gen_id());
    vertex_id_map.insert(String::from("A1"), gen_id());
    vertex_id_map.insert(String::from("B0"), gen_id());
    let pattern_vec = vec![
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
    ];
    (Pattern::from(pattern_vec), vertex_id_map)
}

pub fn build_pattern_index_ranking_case8() -> (Pattern, HashMap<String, PatternId>) {
    let mut vertex_label_map: HashMap<String, PatternLabelId> = HashMap::new();
    let mut vertex_id_map: HashMap<String, PatternId> = HashMap::new();
    let edge_label_map: HashMap<String, PatternLabelId> =
        gen_edge_label_map(vec![String::from("A->A"), String::from("A->B")]);
    vertex_label_map.insert(String::from("A"), 1);
    vertex_label_map.insert(String::from("B"), 2);
    vertex_id_map.insert(String::from("A0"), gen_id());
    vertex_id_map.insert(String::from("A1"), gen_id());
    vertex_id_map.insert(String::from("B0"), gen_id());
    vertex_id_map.insert(String::from("B1"), gen_id());
    let pattern_vec = vec![
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
    ];
    (Pattern::from(pattern_vec), vertex_id_map)
}

pub fn build_pattern_index_ranking_case9() -> (Pattern, HashMap<String, PatternId>) {
    let mut vertex_label_map: HashMap<String, PatternLabelId> = HashMap::new();
    let mut vertex_id_map: HashMap<String, PatternId> = HashMap::new();
    let edge_label_map: HashMap<String, PatternLabelId> =
        gen_edge_label_map(vec![String::from("A->A"), String::from("A->B"), String::from("B->B")]);
    vertex_label_map.insert(String::from("A"), 1);
    vertex_label_map.insert(String::from("B"), 2);
    vertex_id_map.insert(String::from("A0"), gen_id());
    vertex_id_map.insert(String::from("A1"), gen_id());
    vertex_id_map.insert(String::from("B0"), gen_id());
    vertex_id_map.insert(String::from("B1"), gen_id());
    let pattern_vec = vec![
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->B").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
    ];
    (Pattern::from(pattern_vec), vertex_id_map)
}

pub fn build_pattern_index_ranking_case10() -> (Pattern, HashMap<String, PatternId>) {
    let mut vertex_label_map: HashMap<String, PatternLabelId> = HashMap::new();
    let mut vertex_id_map: HashMap<String, PatternId> = HashMap::new();
    let edge_label_map: HashMap<String, PatternLabelId> =
        gen_edge_label_map(vec![String::from("A->A"), String::from("A->B"), String::from("B->B")]);
    vertex_label_map.insert(String::from("A"), 1);
    vertex_label_map.insert(String::from("B"), 2);
    vertex_id_map.insert(String::from("A0"), gen_id());
    vertex_id_map.insert(String::from("A1"), gen_id());
    vertex_id_map.insert(String::from("B0"), gen_id());
    vertex_id_map.insert(String::from("B1"), gen_id());
    let pattern_vec = vec![
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->B").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->B").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
    ];
    (Pattern::from(pattern_vec), vertex_id_map)
}

pub fn build_pattern_index_ranking_case11() -> (Pattern, HashMap<String, PatternId>) {
    let mut vertex_label_map: HashMap<String, PatternLabelId> = HashMap::new();
    let mut vertex_id_map: HashMap<String, PatternId> = HashMap::new();
    let edge_label_map: HashMap<String, PatternLabelId> =
        gen_edge_label_map(vec![String::from("A->A"), String::from("A->B"), String::from("B->B")]);
    vertex_label_map.insert(String::from("A"), 1);
    vertex_label_map.insert(String::from("B"), 2);
    vertex_id_map.insert(String::from("A0"), gen_id());
    vertex_id_map.insert(String::from("A1"), gen_id());
    vertex_id_map.insert(String::from("B0"), gen_id());
    vertex_id_map.insert(String::from("B1"), gen_id());
    vertex_id_map.insert(String::from("B2"), gen_id());
    let pattern_vec = vec![
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->B").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->B").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->B").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_id_map.get("B2").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
    ];
    (Pattern::from(pattern_vec), vertex_id_map)
}

pub fn build_pattern_index_ranking_case12() -> (Pattern, HashMap<String, PatternId>) {
    let mut vertex_label_map: HashMap<String, PatternLabelId> = HashMap::new();
    let mut vertex_id_map: HashMap<String, PatternId> = HashMap::new();
    let edge_label_map: HashMap<String, PatternLabelId> =
        gen_edge_label_map(vec![String::from("A->A"), String::from("A->B"), String::from("B->B")]);
    vertex_label_map.insert(String::from("A"), 1);
    vertex_label_map.insert(String::from("B"), 2);
    vertex_id_map.insert(String::from("A0"), gen_id());
    vertex_id_map.insert(String::from("A1"), gen_id());
    vertex_id_map.insert(String::from("B0"), gen_id());
    vertex_id_map.insert(String::from("B1"), gen_id());
    vertex_id_map.insert(String::from("B2"), gen_id());
    vertex_id_map.insert(String::from("B3"), gen_id());
    let pattern_vec = vec![
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->B").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->B").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->B").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_id_map.get("B2").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->B").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_id_map.get("B3").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
    ];
    (Pattern::from(pattern_vec), vertex_id_map)
}

pub fn build_pattern_index_ranking_case13() -> (Pattern, HashMap<String, PatternId>) {
    let mut vertex_label_map: HashMap<String, PatternLabelId> = HashMap::new();
    let mut vertex_id_map: HashMap<String, PatternId> = HashMap::new();
    let edge_label_map: HashMap<String, PatternLabelId> = gen_edge_label_map(vec![
        String::from("A->A"),
        String::from("A->B"),
        String::from("B->B"),
        String::from("B->A"),
    ]);
    vertex_label_map.insert(String::from("A"), 1);
    vertex_label_map.insert(String::from("B"), 2);
    vertex_id_map.insert(String::from("A0"), gen_id());
    vertex_id_map.insert(String::from("A1"), gen_id());
    vertex_id_map.insert(String::from("A2"), gen_id());
    vertex_id_map.insert(String::from("B0"), gen_id());
    vertex_id_map.insert(String::from("B1"), gen_id());
    vertex_id_map.insert(String::from("B2"), gen_id());
    let pattern_vec = vec![
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->B").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->B").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->B").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_id_map.get("B2").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->A").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_id_map.get("A2").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
    ];
    (Pattern::from(pattern_vec), vertex_id_map)
}

pub fn build_pattern_index_ranking_case14() -> (Pattern, HashMap<String, PatternId>) {
    let mut vertex_label_map: HashMap<String, PatternLabelId> = HashMap::new();
    let mut vertex_id_map: HashMap<String, PatternId> = HashMap::new();
    let edge_label_map: HashMap<String, PatternLabelId> = gen_edge_label_map(vec![
        String::from("A->A"),
        String::from("A->B"),
        String::from("B->B"),
        String::from("B->C"),
    ]);
    vertex_label_map.insert(String::from("A"), 1);
    vertex_label_map.insert(String::from("B"), 2);
    vertex_label_map.insert(String::from("C"), 3);
    vertex_id_map.insert(String::from("A0"), gen_id());
    vertex_id_map.insert(String::from("A1"), gen_id());
    vertex_id_map.insert(String::from("B0"), gen_id());
    vertex_id_map.insert(String::from("B1"), gen_id());
    vertex_id_map.insert(String::from("B2"), gen_id());
    vertex_id_map.insert(String::from("B3"), gen_id());
    vertex_id_map.insert(String::from("C0"), gen_id());
    let pattern_vec = vec![
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->B").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->B").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->B").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_id_map.get("B2").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->B").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_id_map.get("B3").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->C").unwrap(),
            *vertex_id_map.get("B2").unwrap(),
            *vertex_id_map.get("C0").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("C").unwrap(),
        ),
    ];
    (Pattern::from(pattern_vec), vertex_id_map)
}

pub fn build_pattern_index_ranking_case15() -> (Pattern, HashMap<String, PatternId>) {
    let mut vertex_label_map: HashMap<String, PatternLabelId> = HashMap::new();
    let mut vertex_id_map: HashMap<String, PatternId> = HashMap::new();
    let edge_label_map: HashMap<String, PatternLabelId> =
        gen_edge_label_map(vec![String::from("A->A"), String::from("A->B"), String::from("B->C")]);
    vertex_label_map.insert(String::from("A"), 1);
    vertex_label_map.insert(String::from("B"), 2);
    vertex_label_map.insert(String::from("C"), 3);
    vertex_id_map.insert(String::from("A0"), gen_id());
    vertex_id_map.insert(String::from("A1"), gen_id());
    vertex_id_map.insert(String::from("A2"), gen_id());
    vertex_id_map.insert(String::from("A3"), gen_id());
    vertex_id_map.insert(String::from("B0"), gen_id());
    vertex_id_map.insert(String::from("B1"), gen_id());
    vertex_id_map.insert(String::from("B2"), gen_id());
    vertex_id_map.insert(String::from("C0"), gen_id());
    vertex_id_map.insert(String::from("C1"), gen_id());
    let pattern_vec = vec![
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->C").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_id_map.get("C0").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("C").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->C").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_id_map.get("C1").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("C").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("B2").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A2").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A2").unwrap(),
            *vertex_id_map.get("A3").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A3").unwrap(),
            *vertex_id_map.get("A2").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
    ];
    (Pattern::from(pattern_vec), vertex_id_map)
}

pub fn build_pattern_index_ranking_case16() -> (Pattern, HashMap<String, PatternId>) {
    let mut vertex_label_map: HashMap<String, PatternLabelId> = HashMap::new();
    let mut vertex_id_map: HashMap<String, PatternId> = HashMap::new();
    let edge_label_map: HashMap<String, PatternLabelId> = gen_edge_label_map(vec![
        String::from("A->A"),
        String::from("A->B"),
        String::from("B->C"),
        String::from("C->D"),
    ]);
    vertex_label_map.insert(String::from("A"), 1);
    vertex_label_map.insert(String::from("B"), 2);
    vertex_label_map.insert(String::from("C"), 3);
    vertex_label_map.insert(String::from("D"), 4);
    vertex_id_map.insert(String::from("A0"), gen_id());
    vertex_id_map.insert(String::from("A1"), gen_id());
    vertex_id_map.insert(String::from("A2"), gen_id());
    vertex_id_map.insert(String::from("A3"), gen_id());
    vertex_id_map.insert(String::from("B0"), gen_id());
    vertex_id_map.insert(String::from("B1"), gen_id());
    vertex_id_map.insert(String::from("B2"), gen_id());
    vertex_id_map.insert(String::from("C0"), gen_id());
    vertex_id_map.insert(String::from("C1"), gen_id());
    vertex_id_map.insert(String::from("D0"), gen_id());
    let pattern_vec = vec![
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->C").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_id_map.get("C0").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("C").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("B->C").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_id_map.get("C1").unwrap(),
            *vertex_label_map.get("B").unwrap(),
            *vertex_label_map.get("C").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("B0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("B2").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->B").unwrap(),
            *vertex_id_map.get("A2").unwrap(),
            *vertex_id_map.get("B1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("B").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A2").unwrap(),
            *vertex_id_map.get("A3").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A3").unwrap(),
            *vertex_id_map.get("A2").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("C->D").unwrap(),
            *vertex_id_map.get("C1").unwrap(),
            *vertex_id_map.get("D0").unwrap(),
            *vertex_label_map.get("C").unwrap(),
            *vertex_label_map.get("D").unwrap(),
        ),
    ];
    (Pattern::from(pattern_vec), vertex_id_map)
}

pub fn build_pattern_index_ranking_case17() -> (Pattern, HashMap<String, PatternId>) {
    let mut vertex_label_map: HashMap<String, PatternLabelId> = HashMap::new();
    let mut vertex_id_map: HashMap<String, PatternId> = HashMap::new();
    let edge_label_map: HashMap<String, PatternLabelId> =
        gen_edge_label_map(vec![String::from("A->A"), String::from("A->B"), String::from("B->C")]);
    vertex_label_map.insert(String::from("A"), 1);
    vertex_id_map.insert(String::from("A0"), gen_id());
    vertex_id_map.insert(String::from("A1"), gen_id());
    vertex_id_map.insert(String::from("A2"), gen_id());
    vertex_id_map.insert(String::from("A3"), gen_id());
    vertex_id_map.insert(String::from("A4"), gen_id());
    vertex_id_map.insert(String::from("A5"), gen_id());
    let pattern_vec = vec![
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("A2").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A2").unwrap(),
            *vertex_id_map.get("A3").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A3").unwrap(),
            *vertex_id_map.get("A4").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A4").unwrap(),
            *vertex_id_map.get("A5").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
    ];
    (Pattern::from(pattern_vec), vertex_id_map)
}

pub fn build_pattern_index_ranking_case18() -> (Pattern, HashMap<String, PatternId>) {
    let mut vertex_label_map: HashMap<String, PatternLabelId> = HashMap::new();
    let mut vertex_id_map: HashMap<String, PatternId> = HashMap::new();
    let edge_label_map: HashMap<String, PatternLabelId> =
        gen_edge_label_map(vec![String::from("A->A"), String::from("A->B"), String::from("B->C")]);
    vertex_label_map.insert(String::from("A"), 1);
    vertex_id_map.insert(String::from("A0"), gen_id());
    vertex_id_map.insert(String::from("A1"), gen_id());
    vertex_id_map.insert(String::from("A2"), gen_id());
    vertex_id_map.insert(String::from("A3"), gen_id());
    vertex_id_map.insert(String::from("A4"), gen_id());
    vertex_id_map.insert(String::from("A5"), gen_id());
    let pattern_vec = vec![
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A1").unwrap(),
            *vertex_id_map.get("A2").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A2").unwrap(),
            *vertex_id_map.get("A3").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A3").unwrap(),
            *vertex_id_map.get("A4").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A4").unwrap(),
            *vertex_id_map.get("A5").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
        PatternEdge::new(
            gen_id(),
            *edge_label_map.get("A->A").unwrap(),
            *vertex_id_map.get("A5").unwrap(),
            *vertex_id_map.get("A0").unwrap(),
            *vertex_label_map.get("A").unwrap(),
            *vertex_label_map.get("A").unwrap(),
        ),
    ];
    (Pattern::from(pattern_vec), vertex_id_map)
}
