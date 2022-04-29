type ID = i32;
type LabelID = i32;
type Index = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Out = 0,
    In,
}

pub mod codec;
pub mod extend_step;
pub mod pattern;
pub mod pattern_meta;

#[cfg(test)]
pub(crate) mod test_cases {
    use std::collections::{BTreeMap, HashMap};
    use std::fs::File;

    use ir_core::{plan::meta::Schema, JsonIO};
    use rand::Rng;

    use crate::Direction;
    use crate::pattern::*;
    use crate::pattern_meta::*;
    use crate::extend_step::*;

    /// The pattern looks like:
    /// A <-> A
    /// where <-> means two edges
    /// A's label's id is 0
    /// The edges's labels' id are both 0
    /// The left A has id 0
    /// The right A has id 1
    pub fn build_pattern_case1() -> Pattern {
        let pattern_vec = vec![
            PatternEdge::new(0, 0, 0, 1, 0, 0),
            PatternEdge::new(1, 0, 1, 0, 0, 0)
        ];
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
            PatternEdge::new(3, 1, 1, 2, 0, 1)
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
        let mut rng = rand::thread_rng();
        let pattern_vec = vec![
            PatternEdge::new(rng.gen::<i32>(), 0, 0, 1, 0, 0),
            PatternEdge::new(rng.gen::<i32>(), 1, 0, 2, 0, 1),
            PatternEdge::new(rng.gen::<i32>(), 1, 1, 3, 0, 1),
            PatternEdge::new(rng.gen::<i32>(), 2, 2, 3, 1, 1)
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
        let mut rng = rand::thread_rng();
        let pattern_vec = vec![
            PatternEdge::new(rng.gen::<i32>(), 0, 0, 1, 0, 0),
            PatternEdge::new(rng.gen::<i32>(), 0, 1, 0, 0, 0),
            PatternEdge::new(rng.gen::<i32>(), 1, 0, 2, 0, 1),
            PatternEdge::new(rng.gen::<i32>(), 1, 1, 3, 0, 1),
            PatternEdge::new(rng.gen::<i32>(), 2, 2, 3, 1, 1)
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
        let mut rng = rand::thread_rng();
        let label_a = 3;
        let label_b = 2;
        let label_c = 1;
        let label_d = 0;
        let id_vec_a: Vec<i32> = vec![100, 200, 300, 400];
        let id_vec_b: Vec<i32> = vec![10, 20, 30];
        let id_vec_c: Vec<i32> = vec![1, 2, 3];
        let id_vec_d: Vec<i32> = vec![1000];
        let pattern_vec = vec![
            PatternEdge::new(rng.gen::<i32>(), 15, id_vec_c[0], id_vec_b[1], label_c, label_b), 
            PatternEdge::new(rng.gen::<i32>(), 30, id_vec_a[0], id_vec_b[1], label_a, label_b),
            PatternEdge::new(rng.gen::<i32>(), 15, id_vec_c[2], id_vec_b[1], label_c, label_b),
            PatternEdge::new(rng.gen::<i32>(), 30, id_vec_a[0], id_vec_b[0], label_a, label_b),
            PatternEdge::new(rng.gen::<i32>(), 30, id_vec_a[3], id_vec_b[1], label_a, label_b),
            PatternEdge::new(rng.gen::<i32>(), 30, id_vec_a[3], id_vec_b[2], label_a, label_b),
            PatternEdge::new(rng.gen::<i32>(), 30, id_vec_a[1], id_vec_b[2], label_a, label_b),
            PatternEdge::new(rng.gen::<i32>(), 20, id_vec_a[1], id_vec_a[2], label_a, label_a),
            PatternEdge::new(rng.gen::<i32>(), 20, id_vec_a[2], id_vec_a[1], label_a, label_a),
            PatternEdge::new(rng.gen::<i32>(), 15, id_vec_c[1], id_vec_b[2], label_c, label_b),
            PatternEdge::new(rng.gen::<i32>(), 5, id_vec_d[0], id_vec_c[1], label_d, label_c)
        ];
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
    pub fn build_extend_step_case1() -> ExtendStep {
        let extend_edge1 = ExtendEdge::new(0, 0, 1, Direction::Out);
        let extend_edge2 = extend_edge1.clone();
        ExtendStep::from((1, vec![extend_edge1, extend_edge2]))
    }

    pub fn get_modern_pattern_meta() -> PatternMeta {
        let modern_schema_file = match File::open("resource/modern_schema.json") {
            Ok(file) => file,
            Err(_) => File::open("catalogue/resource/modern_schema.json").unwrap(),
        };
        let modern_schema = Schema::from_json(modern_schema_file).unwrap();
        PatternMeta::from(modern_schema)
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
        pattern.get_vertex_mut_from_id(1).unwrap().set_index(1);
        pattern
    }

    /// Person -> created -> Software
    pub fn build_modern_pattern_case4() -> Pattern {
        let pattern_edge = PatternEdge::new(0, 1, 0, 1, 0, 1);
        Pattern::from(vec![pattern_edge])
    }

    pub fn get_ldbc_pattern_meta() -> PatternMeta {
        let ldbc_schema_file = match File::open("resource/ldbc_schema.json") {
            Ok(file) => file,
            Err(_) => File::open("catalogue/resource/ldbc_schema.json").unwrap(),
        };
        let ldbc_schema = Schema::from_json(ldbc_schema_file).unwrap();
        PatternMeta::from(ldbc_schema)
    }

    /// Pattern from ldbc schema file
    /// Person -> knows -> Person
    pub fn build_ldbc_pattern_case1() -> Pattern {
        let pattern_edge = PatternEdge::new(0, 12, 0, 1, 1, 1);
        let mut pattern = Pattern::from(vec![pattern_edge]);
        pattern.get_vertex_mut_from_id(1).unwrap().set_index(1);
        pattern
    }
}