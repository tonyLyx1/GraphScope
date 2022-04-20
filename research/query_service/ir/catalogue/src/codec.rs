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

use std::collections::BTreeSet;

use ascii::AsciiString;
use ascii::ToAsciiChar;

use super::pattern::{Direction, Pattern};
pub trait Encode<T> {
    fn encode_to(&self, encoder: &Encoder) -> T;
}

pub trait Decode<T>: Encode<T> {
    fn decode_from(src_code: T, encoder: &Encoder) -> Self;
}

/// ## Unique Pattern Identity Encoder
/// ### Member Variables
/// Contains the bit number that each variable in the encoding unit occupies
#[derive(Debug, Clone)]
pub struct Encoder {
    edge_label_bit_num: u8,
    vertex_label_bit_num: u8,
    edge_direction_bit_num: u8,
    vertex_index_bit_num: u8,
}

impl Encoder {
    /// ### Initialize Encoder with User Definded Parameters
    pub fn initialize(
        edge_label_bit_num: u8, vertex_label_bit_num: u8, edge_direction_bit_num: u8,
        vertex_index_bit_num: u8,
    ) -> Encoder {
        Encoder { edge_label_bit_num, vertex_label_bit_num, edge_direction_bit_num, vertex_index_bit_num }
    }

    /// ### Initialize the Encoder by Analyzing a Pattern
    /// The vertex_index_bit_num can be a user defined value if it is applicable to the pattern
    pub fn initialize_from_pattern(pattern: &Pattern, vertex_index_bit_num: u8) -> Encoder {
        let min_edge_label_bit_num = pattern.get_min_edge_label_bit_num();
        let min_vertex_label_bit_num = pattern.get_min_vertex_label_bit_num();
        let mut min_vertex_index_bit_num = pattern.get_min_vertex_index_bit_num();
        // Apply the user defined vertex_index_bit_num only if it is larger than the minimum value needed for the pattern
        if vertex_index_bit_num > min_vertex_index_bit_num {
            min_vertex_index_bit_num = vertex_index_bit_num;
        }

        let edge_direction_bit_num = 2;
        Encoder {
            edge_label_bit_num: min_edge_label_bit_num,
            vertex_label_bit_num: min_vertex_label_bit_num,
            edge_direction_bit_num,
            vertex_index_bit_num: min_vertex_index_bit_num,
        }
    }

    /// ### Compute the u8 value for each storage unit (AsciiChar or u8)
    pub fn get_encode_numerical_value(
        &self, value: i32, value_head: u8, value_tail: u8, storage_unit_valid_bit_num: u8,
        storage_unit_index: u8,
    ) -> u8 {
        let mut output: i32;
        let char_tail = storage_unit_index * storage_unit_valid_bit_num;
        let char_head = (storage_unit_index + 1) * storage_unit_valid_bit_num - 1;
        if value_tail > char_head || value_head < char_tail {
            output = 0;
        } else if value_tail >= char_tail && value_head <= char_head {
            let offset_bit_num = value_tail - char_tail;
            output = value * (1 << offset_bit_num);
        } else if value_tail < char_tail && value_head <= char_head {
            let shift_bit_num = char_tail - value_tail;
            output = value / (1 << shift_bit_num);
        } else if value_tail >= char_tail && value_head > char_head {
            let shift_bit_num = char_head + 1 - value_tail;
            output = value % (1 << shift_bit_num);
            output = output * (1 << (storage_unit_valid_bit_num - shift_bit_num));
        } else if value_tail < char_tail && value_head > char_head {
            let right_shift_bit_num = char_tail - value_tail;
            output = value % (1 << right_shift_bit_num);
            output = output % (1 << storage_unit_valid_bit_num);
        } else {
            panic!("Error in Converting Encode Unit to ASCII String: No Such Value Exists");
        }

        return output as u8;
    }
}

/// Getters
impl Encoder {
    /// ### Getter of all the member variables at a time
    pub fn get_all_member_variable(&self) -> (u8, u8, u8, u8) {
        (
            self.edge_label_bit_num,
            self.vertex_label_bit_num,
            self.edge_direction_bit_num,
            self.vertex_index_bit_num,
        )
    }

    /// ### Compute How many storage units should be used to store an encode unit
    /// #### Parameter
    /// storage_unit_bit_num: how many valid bits a storage unit has
    ///
    /// Remark: the storage_unit_bit_num refers to valid bits. For ascii char, it is u8 but can only take 7 bits
    pub fn get_storage_unit_num_per_encode_unit(&self, storage_unit_bit_num: u8) -> u8 {
        let sum_bit_num = 1 * self.edge_label_bit_num
            + 2 * self.vertex_label_bit_num
            + 1 * self.edge_direction_bit_num
            + 2 * self.vertex_index_bit_num;

        (sum_bit_num - 1) / storage_unit_bit_num + 1
    }
}

/// ## Edge-Based Encoding Unit
/// ### Member Variables
/// Each encoding unit represents an edge in the pattern and contains:
/// 1. Edge Label
/// 2. Src Vertex Label
/// 3. Dst Vertex Label
/// 4. Edge Direction
/// 5. Src Vertex Index
/// 6. Dst Vertex Index
#[derive(Debug, Clone)]
pub struct PatternEdgeEncodeUnit {
    edge_label: i32,
    start_v_label: i32,
    end_v_label: i32,
    edge_direction: Direction,
    start_v_index: i32,
    end_v_index: i32,
}

impl PatternEdgeEncodeUnit {
    /// ### Create a new EncodeUnit with Input Variables
    pub fn new(
        edge_label: i32, start_v_label: i32, end_v_label: i32, edge_direction: Direction,
        start_v_index: i32, end_v_index: i32,
    ) -> PatternEdgeEncodeUnit {
        PatternEdgeEncodeUnit {
            edge_label,
            start_v_label,
            end_v_label,
            edge_direction,
            start_v_index,
            end_v_index,
        }
    }

    /// ### Setter of end_v_label
    pub fn set_end_v_label(&mut self, end_v_label: i32) {
        self.end_v_label = end_v_label;
    }

    /// ### Setter of end_v_index
    pub fn set_end_v_index(&mut self, end_v_index: i32) {
        self.end_v_index = end_v_index;
    }
}

/// Getters
impl PatternEdgeEncodeUnit {
    pub fn get_all_member_variable(&self) -> (i32, i32, i32, i32, i32, i32) {
        (
            self.edge_label,
            self.start_v_label,
            self.end_v_label,
            self.edge_direction as i32,
            self.start_v_index,
            self.end_v_index,
        )
    }
}

impl Encode<AsciiString> for PatternEdgeEncodeUnit {
    fn encode_to(&self, encoder: &Encoder) -> AsciiString {
        // 根据Encoder的比特位参数来决定生成string的格式
        let mut encode_str = AsciiString::new();
        let (edge_label, start_v_label, end_v_label, edge_dir, start_v_index, end_v_index) =
            self.get_all_member_variable();
        let (edge_label_bit_num, v_label_bit_num, edge_dir_bit_num, v_index_bit_num) =
            encoder.get_all_member_variable();
        let storage_unit_valid_bit_num = 7;
        let ascii_char_num_per_encode_unit =
            encoder.get_storage_unit_num_per_encode_unit(storage_unit_valid_bit_num);
        // Compute Value head/tail for each field
        let (end_v_index_head, end_v_index_tail) = (v_index_bit_num - 1, 0);
        let (start_v_index_head, start_v_index_tail) =
            (end_v_index_head + v_index_bit_num, end_v_index_head + 1);
        let (edge_dir_head, edge_dir_tail) =
            (start_v_index_head + edge_dir_bit_num, start_v_index_head + 1);
        let (end_v_label_head, end_v_label_tail) = (edge_dir_head + v_label_bit_num, edge_dir_head + 1);
        let (start_v_label_head, start_v_label_tail) =
            (end_v_label_head + v_label_bit_num, end_v_label_head + 1);
        let (edge_label_head, edge_label_tail) =
            (start_v_label_head + edge_label_bit_num, start_v_label_head + 1);
        // Compute each ASCII char
        for i in (0..ascii_char_num_per_encode_unit).rev() {
            let char_value: u8 = encoder.get_encode_numerical_value(
                edge_label,
                edge_label_head,
                edge_label_tail,
                storage_unit_valid_bit_num,
                i,
            ) + encoder.get_encode_numerical_value(
                start_v_label,
                start_v_label_head,
                start_v_label_tail,
                storage_unit_valid_bit_num,
                i,
            ) + encoder.get_encode_numerical_value(
                end_v_label,
                end_v_label_head,
                end_v_label_tail,
                storage_unit_valid_bit_num,
                i,
            ) + encoder.get_encode_numerical_value(
                edge_dir,
                edge_dir_head,
                edge_dir_tail,
                storage_unit_valid_bit_num,
                i,
            ) + encoder.get_encode_numerical_value(
                start_v_index,
                start_v_index_head,
                start_v_index_tail,
                storage_unit_valid_bit_num,
                i,
            ) + encoder.get_encode_numerical_value(
                end_v_index,
                end_v_index_head,
                end_v_index_tail,
                storage_unit_valid_bit_num,
                i,
            );
            encode_str.push(char_value.to_ascii_char().unwrap());
        }

        encode_str
    }
}

impl Encode<Vec<u8>> for PatternEdgeEncodeUnit {
    fn encode_to(&self, encoder: &Encoder) -> Vec<u8> {
        let mut encode_vec: Vec<u8> = Vec::new();
        let (edge_label, start_v_label, end_v_label, edge_dir, start_v_index, end_v_index) =
            self.get_all_member_variable();
        let (edge_label_bit_num, v_label_bit_num, edge_dir_bit_num, v_index_bit_num) =
            encoder.get_all_member_variable();
        let storage_unit_valid_bit_num = 8;
        let ascii_char_num_per_encode_unit =
            encoder.get_storage_unit_num_per_encode_unit(storage_unit_valid_bit_num);
        // Compute Value head/tail for each field
        let (end_v_index_head, end_v_index_tail) = (v_index_bit_num - 1, 0);
        let (start_v_index_head, start_v_index_tail) =
            (end_v_index_head + v_index_bit_num, end_v_index_head + 1);
        let (edge_dir_head, edge_dir_tail) =
            (start_v_index_head + edge_dir_bit_num, start_v_index_head + 1);
        let (end_v_label_head, end_v_label_tail) = (edge_dir_head + v_label_bit_num, edge_dir_head + 1);
        let (start_v_label_head, start_v_label_tail) =
            (end_v_label_head + v_label_bit_num, end_v_label_head + 1);
        let (edge_label_head, edge_label_tail) =
            (start_v_label_head + edge_label_bit_num, start_v_label_head + 1);
        // Compute each ASCII char
        for i in (0..ascii_char_num_per_encode_unit).rev() {
            let char_value: u8 = encoder.get_encode_numerical_value(
                edge_label,
                edge_label_head,
                edge_label_tail,
                storage_unit_valid_bit_num,
                i,
            ) + encoder.get_encode_numerical_value(
                start_v_label,
                start_v_label_head,
                start_v_label_tail,
                storage_unit_valid_bit_num,
                i,
            ) + encoder.get_encode_numerical_value(
                end_v_label,
                end_v_label_head,
                end_v_label_tail,
                storage_unit_valid_bit_num,
                i,
            ) + encoder.get_encode_numerical_value(
                edge_dir,
                edge_dir_head,
                edge_dir_tail,
                storage_unit_valid_bit_num,
                i,
            ) + encoder.get_encode_numerical_value(
                start_v_index,
                start_v_index_head,
                start_v_index_tail,
                storage_unit_valid_bit_num,
                i,
            ) + encoder.get_encode_numerical_value(
                end_v_index,
                end_v_index_head,
                end_v_index_tail,
                storage_unit_valid_bit_num,
                i,
            );
            encode_vec.push(char_value);
        }

        encode_vec
    }
}

impl Encode<AsciiString> for Pattern {
    fn encode_to(&self, encoder: &Encoder) -> AsciiString {
        // Initialize an BTreeSet to Store the Encoding Units
        let mut set = BTreeSet::from([]);
        // Encode Each Edge in the Pattern as an Encoding Unit
        let edges = self.get_edges();
        for (edge_id, _) in edges.iter() {
            let encode_unit: PatternEdgeEncodeUnit = self.get_edge_encode_unit_by_id(*edge_id);
            let encode_string: AsciiString =
                <PatternEdgeEncodeUnit as Encode<AsciiString>>::encode_to(&encode_unit, encoder);
            set.insert(encode_string);
        }

        let mut encode_value = AsciiString::new();
        let mut set_iter = set.iter();
        loop {
            match set_iter.next() {
                Some(value) => encode_value = encode_value + &value,
                None => break,
            }
        }
        encode_value
    }
}

impl Encode<Vec<u8>> for Pattern {
    fn encode_to(&self, encoder: &Encoder) -> Vec<u8> {
        // Initialize an BTreeSet to Store the Encoding Units
        let mut set = BTreeSet::from([]);
        // Encode Each Edge in the Pattern as an Encoding Unit
        let edges = self.get_edges();
        for (edge_id, _) in edges.iter() {
            let encode_unit: PatternEdgeEncodeUnit = self.get_edge_encode_unit_by_id(*edge_id);
            let encode_vec: Vec<u8> =
                <PatternEdgeEncodeUnit as Encode<Vec<u8>>>::encode_to(&encode_unit, encoder);
            set.insert(encode_vec);
        }

        let mut encode_vec: Vec<u8> = Vec::new();
        let mut set_iter = set.iter();
        loop {
            match set_iter.next() {
                Some(vec) => encode_vec.extend(vec),
                None => break,
            }
        }

        encode_vec
    }
}

// impl Encode for ExtendEdge {
//   /// ### Convert ExtendEdge to EncodeUnit
//   /// Since ExtendEdge does not contain information about the ending vertex,
//   /// the end_v_label and end_v_index are set as 0 and wait for further modification
//   fn to_encode_unit(&self) -> EncodeUnit {
//       EncodeUnit::new(self.edge_label, self.start_v_label, 0, self.dir, self.start_v_index, 0)
//   }

//   fn to_encode_value(&self, encoder: &Encoder) -> AsciiString {
//     let encode_unit = self.to_encode_unit();
//     encode_unit.to_encode_value(encoder)
//   }
// }

// impl Encode for ExtendStep {
//   fn to_encode_value(&self, encoder: &Encoder) -> AsciiString {
//     // Initialize AsciiString
//     let mut encode_value = AsciiString::new();
//     // Update the index of the pattern
//     let target_v_index = self.get_target_v_index();
//     // Set End Vertex Information on the Encode Units of ExtendEdges
//     let extend_edges_iter = self.get_extend_edges().iter();
//     for (_, edges) in extend_edges_iter {
//         for edge in edges {
//             let mut encode_unit: EncodeUnit = edge.to_encode_unit();
//             encode_unit.set_end_v_label(self.get_target_v_label());
//             encode_unit.set_end_v_index(target_v_index);
//             encode_value = encode_value + &encode_unit.to_encode_value(encoder);
//         }
//     }

//     encode_value
//   }
// }

/// Unit Testing
#[cfg(test)]
mod tests {
    use ascii::{self, AsciiString, ToAsciiChar};

    use crate::codec::*;
    use crate::pattern::*;

    fn build_pattern_testcase_1() -> Pattern {
        let pattern_edge1 = PatternEdge::new(0, 1, 0, 1, 1, 2);
        let pattern_edge2 = PatternEdge::new(1, 2, 0, 2, 1, 3);
        let pattern_vec = vec![pattern_edge1, pattern_edge2];
        Pattern::from(pattern_vec)
    }

    fn build_pattern_testcase_2() -> Pattern {
        let edge_1 = PatternEdge::new(0, 1, 0, 1, 1, 2);
        let edge_2 = PatternEdge::new(1, 2, 0, 2, 1, 3);
        let edge_3 = PatternEdge::new(2, 3, 1, 2, 2, 3);
        let edge_4 = PatternEdge::new(3, 4, 0, 3, 1, 4);
        let edge_5 = PatternEdge::new(4, 5, 1, 3, 2, 4);
        let edge_6 = PatternEdge::new(5, 6, 3, 2, 4, 3);
        let pattern_edges = vec![edge_1, edge_2, edge_3, edge_4, edge_5, edge_6];
        Pattern::from(pattern_edges)
    }

    /// ### Generate AsciiString from Vector
    fn generate_asciistring_from_vec(vec: &Vec<u8>) -> AsciiString {
        let mut output = AsciiString::new();
        for value in vec {
            output.push(value.to_ascii_char().unwrap());
        }

        output
    }

    #[test]
    fn test_create_encode_unit_from_edge() {
        let pattern = build_pattern_testcase_1();
        let encode_unit_1 = pattern.get_edge_encode_unit_by_id(0);
        assert_eq!(encode_unit_1.edge_label, 1);
        assert_eq!(encode_unit_1.start_v_label, 1);
        assert_eq!(encode_unit_1.end_v_label, 2);
        assert_eq!(encode_unit_1.edge_direction, Direction::Out);
        assert_eq!(encode_unit_1.start_v_index, 0);
        assert_eq!(encode_unit_1.end_v_index, 0);
        let encode_unit_2 = pattern.get_edge_encode_unit_by_id(1);
        assert_eq!(encode_unit_2.edge_label, 2);
        assert_eq!(encode_unit_2.start_v_label, 1);
        assert_eq!(encode_unit_2.end_v_label, 3);
        assert_eq!(encode_unit_2.edge_direction, Direction::Out);
        assert_eq!(encode_unit_2.start_v_index, 0);
        assert_eq!(encode_unit_2.end_v_index, 0);
    }

    #[test]
    fn test_initialize_encoder_from_parameter_case1() {
        let encoder = Encoder::initialize(2, 3, 4, 5);
        assert_eq!(encoder.edge_label_bit_num, 2);
        assert_eq!(encoder.vertex_label_bit_num, 3);
        assert_eq!(encoder.edge_direction_bit_num, 4);
        assert_eq!(encoder.vertex_index_bit_num, 5);
    }

    #[test]
    fn test_initialize_encoder_from_parameter_case2() {
        let encoder = Encoder::initialize(2, 2, 2, 2);
        assert_eq!(encoder.edge_label_bit_num, 2);
        assert_eq!(encoder.vertex_label_bit_num, 2);
        assert_eq!(encoder.edge_direction_bit_num, 2);
        assert_eq!(encoder.vertex_index_bit_num, 2);
    }

    #[test]
    fn test_initialize_encoder_from_pattern_case1() {
        let pattern = build_pattern_testcase_1();
        let default_vertex_index_bit_num = 0;
        let encoder = Encoder::initialize_from_pattern(&pattern, default_vertex_index_bit_num);
        assert_eq!(encoder.edge_label_bit_num, 1);
        assert_eq!(encoder.vertex_label_bit_num, 2);
        assert_eq!(encoder.edge_direction_bit_num, 2);
        assert_eq!(encoder.vertex_index_bit_num, 1);
    }

    #[test]
    fn test_initialize_encoder_from_pattern_case2() {
        let pattern = build_pattern_testcase_2();
        let default_vertex_index_bit_num = 2;
        let encoder = Encoder::initialize_from_pattern(&pattern, default_vertex_index_bit_num);
        assert_eq!(encoder.edge_label_bit_num, 3);
        assert_eq!(encoder.vertex_label_bit_num, 2);
        assert_eq!(encoder.edge_direction_bit_num, 2);
        assert_eq!(encoder.vertex_index_bit_num, 2);
    }

    #[test]
    fn encode_unit_to_ascii_string() {
        let pattern = build_pattern_testcase_1();
        let encoder = Encoder::initialize(2, 2, 2, 2);
        let encode_unit_1 = pattern.get_edge_encode_unit_by_id(0);
        let encode_string_1 =
            <PatternEdgeEncodeUnit as Encode<AsciiString>>::encode_to(&encode_unit_1, &encoder);
        let expected_encode_string_1: AsciiString = generate_asciistring_from_vec(&vec![11, 0]);
        assert_eq!(encode_string_1.len(), 2);
        assert_eq!(encode_string_1, expected_encode_string_1);
        let encode_unit_2 = pattern.get_edge_encode_unit_by_id(1);
        let encode_string_2 =
            <PatternEdgeEncodeUnit as Encode<AsciiString>>::encode_to(&encode_unit_2, &encoder);
        let expected_encode_string_2: AsciiString = generate_asciistring_from_vec(&vec![19, 64]);
        assert_eq!(encode_string_2.len(), 2);
        assert_eq!(encode_string_2, expected_encode_string_2);
    }

    #[test]
    fn encode_unit_to_vec_u8() {
        let pattern = build_pattern_testcase_1();
        let encoder = Encoder::initialize(2, 2, 2, 2);
        let encode_unit_1 = pattern.get_edge_encode_unit_by_id(0);
        let encode_vec_1 = <PatternEdgeEncodeUnit as Encode<Vec<u8>>>::encode_to(&encode_unit_1, &encoder);
        let expected_encode_vec_1: Vec<u8> = vec![5, 128];
        assert_eq!(encode_vec_1.len(), 2);
        assert_eq!(encode_vec_1, expected_encode_vec_1);
        let encode_unit_2 = pattern.get_edge_encode_unit_by_id(1);
        let encode_vec_2 = <PatternEdgeEncodeUnit as Encode<Vec<u8>>>::encode_to(&encode_unit_2, &encoder);
        let expected_encode_vec_2: Vec<u8> = vec![9, 192];
        assert_eq!(encode_vec_2.len(), 2);
        assert_eq!(encode_vec_2, expected_encode_vec_2);
    }

    #[test]
    fn encode_pattern_to_asciistring_case_1() {
        let pattern = build_pattern_testcase_1();
        let encoder = Encoder::initialize(2, 2, 2, 2);
        let encode_value = <Pattern as Encode<AsciiString>>::encode_to(&pattern, &encoder);
        let expected_encode_string_1: AsciiString = generate_asciistring_from_vec(&vec![11, 0]);
        let expected_encode_string_2: AsciiString = generate_asciistring_from_vec(&vec![19, 64]);
        let expected_encode_value = expected_encode_string_1 + &expected_encode_string_2;
        assert_eq!(encode_value, expected_encode_value);
    }

    #[test]
    fn encode_pattern_to_vec_u8_case_1() {
        let pattern = build_pattern_testcase_1();
        let encoder = Encoder::initialize(2, 2, 2, 2);
        let encode_vec = <Pattern as Encode<Vec<u8>>>::encode_to(&pattern, &encoder);
        let mut expected_encode_vec_1: Vec<u8> = vec![5, 128];
        let expected_encode_vec_2: Vec<u8> = vec![9, 192];
        expected_encode_vec_1.extend(expected_encode_vec_2);
        assert_eq!(encode_vec, expected_encode_vec_1);
    }
}
