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

use ascii::{ToAsciiChar, AsciiString};

use super::pattern::Pattern;
use crate::extend_step::{ExtendEdge, ExtendStep};
use crate::pattern::PatternEdge;
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
    edge_label_bit_num: usize,
    vertex_label_bit_num: usize,
    direction_bit_num: usize,
    vertex_index_bit_num: usize,
}

impl Encoder {
    /// ### Initialize Encoder with User Definded Parameters
    pub fn initialize(
        edge_label_bit_num: usize, vertex_label_bit_num: usize, edge_direction_bit_num: usize,
        vertex_index_bit_num: usize,
    ) -> Encoder {
        Encoder {
            edge_label_bit_num,
            vertex_label_bit_num,
            direction_bit_num: edge_direction_bit_num,
            vertex_index_bit_num,
        }
    }

    /// ### Initialize the Encoder by Analyzing a Pattern
    /// The vertex_index_bit_num can be a user defined value if it is applicable to the pattern
    pub fn initialize_from_pattern(pattern: &Pattern, vertex_index_bit_num: usize) -> Encoder {
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
            direction_bit_num: edge_direction_bit_num,
            vertex_index_bit_num: min_vertex_index_bit_num,
        }
    }

    /// ### Compute the u8 value for each storage unit (AsciiChar or u8)
    pub fn get_encode_numerical_value(
        value: i32, value_head: usize, value_tail: usize, storage_unit_valid_bit_num: usize, storage_unit_index: usize,
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

    pub fn get_decode_value_by_head_tail(
        src_code: &Vec<u8>, head: usize, tail: usize, storage_unit_bit_num: usize,
    ) -> i32 {
        if head < tail {
            panic!("The head must be at least larger or equal to tail");
        }
        let mut output;

        let head_index = src_code.len() - 1 - (head / storage_unit_bit_num) as usize;
        let head_offset = head % storage_unit_bit_num;
        let tail_index = src_code.len() - 1 - (tail / storage_unit_bit_num) as usize;
        let tail_offset = tail % storage_unit_bit_num;

        if head_index >= src_code.len() || tail_index >= src_code.len() {
            panic!("The head and tail values are out of range");
        }

        if head_index == tail_index {
            output = (src_code[head_index] << (8 - 1 - head_offset) >> (8 - 1 - head_offset + tail_offset))
                as i32;
        } else {
            let index_diff = tail_index - head_index;
            output = (src_code[tail_index] as i32) >> tail_offset;
            for i in 1..index_diff {
                output += (src_code[tail_index - i] as i32)
                    << (storage_unit_bit_num - tail_offset + (i - 1) * storage_unit_bit_num);
            }
            output += ((src_code[head_index] << (8 - 1 - head_offset) >> (8 - 1 - head_offset)) as i32)
                << (storage_unit_bit_num - tail_offset + (index_diff - 1) * storage_unit_bit_num);
        }
        output
    }
}

/// Getters
impl Encoder {
    pub fn get_edge_label_bit_num(&self) -> usize {
        self.edge_label_bit_num
    }

    pub fn get_vertex_label_bit_num(&self) -> usize {
        self.vertex_label_bit_num
    }

    pub fn get_direction_bit_num(&self) -> usize {
        self.direction_bit_num
    }

    pub fn get_vertex_index_bit_num(&self) -> usize {
        self.vertex_index_bit_num
    }
}

pub struct EncodeUnit {
    values: Vec<i32>,
    heads: Vec<usize>,
    tails: Vec<usize>,
}

impl EncodeUnit {
    pub fn from_pattern_edge(pattern: &Pattern, pattern_edge: &PatternEdge, encoder: &Encoder) -> Self {
        let edge_label = pattern_edge.get_label();
        let start_v_label = pattern_edge.get_start_vertex_label();
        let end_v_label = pattern_edge.get_end_vertex_label();
        let start_v_index = pattern.get_vertex_index(pattern_edge.get_start_vertex_id());
        let end_v_index = pattern.get_vertex_index(pattern_edge.get_end_vertex_id());

        let edge_label_bit_num = encoder.get_edge_label_bit_num();
        let vertex_label_bit_num = encoder.get_vertex_label_bit_num();
        let vertex_index_bit_num = encoder.get_vertex_index_bit_num();

        let values = vec![edge_label, start_v_label, end_v_label, start_v_index, end_v_index];
        let heads = vec![
            edge_label_bit_num + 2 * vertex_label_bit_num + 2 * vertex_index_bit_num - 1,
            2 * vertex_label_bit_num + 2 * vertex_index_bit_num - 1,
            vertex_label_bit_num + 2 * vertex_index_bit_num - 1,
            2 * vertex_index_bit_num - 1,
            vertex_index_bit_num - 1,
        ];
        let tails = vec![
            2 * vertex_label_bit_num + 2 * vertex_index_bit_num,
            vertex_label_bit_num + 2 * vertex_index_bit_num,
            2 * vertex_index_bit_num,
            vertex_index_bit_num,
            0,
        ];
        EncodeUnit { values, heads, tails }
    }

    pub fn from_extend_edge(extend_edge: &ExtendEdge, encoder: &Encoder) -> Self {
        let start_v_label = extend_edge.get_start_vertex_label();
        let start_v_index = extend_edge.get_start_vertex_index();
        let edge_label = extend_edge.get_edge_label();
        let dir = extend_edge.get_direction();

        let vertex_label_bit_num = encoder.get_vertex_label_bit_num();
        let vertex_index_bit_num = encoder.get_vertex_index_bit_num();
        let edge_label_bit_num = encoder.get_edge_label_bit_num();
        let direction_bit_num = encoder.get_direction_bit_num();

        let values = vec![start_v_label, start_v_index, edge_label, dir as i32];
        let heads = vec![
            vertex_label_bit_num + vertex_index_bit_num + edge_label_bit_num + direction_bit_num - 1,
            vertex_index_bit_num + edge_label_bit_num + direction_bit_num - 1,
            edge_label_bit_num + direction_bit_num - 1,
            direction_bit_num - 1,
        ];
        let tails = vec![
            vertex_index_bit_num + edge_label_bit_num + direction_bit_num,
            edge_label_bit_num + direction_bit_num,
            direction_bit_num,
            0,
        ];
        EncodeUnit { values, heads, tails }
    }

    pub fn to_vec_u8(&self, storage_unit_bit_num: usize) -> Vec<u8> {
        let storage_unit_num = self.heads[0] / storage_unit_bit_num + 1;
        let mut encode_vec = Vec::with_capacity(storage_unit_num as usize);
        for i in (0..storage_unit_num).rev() {
            let mut unit_value: u8 = 0;
            for j in 0..self.values.len() {
                let value = self.values[j];
                let head = self.heads[j];
                let tail = self.tails[j];
                unit_value +=
                    Encoder::get_encode_numerical_value(value, head, tail, storage_unit_bit_num, i);
            }
            encode_vec.push(unit_value);
        }
        encode_vec
    }

    pub fn get_bits_num(&self) -> usize {
        self.heads[0] + 1
    }
}

impl Encode<Vec<u8>> for EncodeUnit {
    fn encode_to(&self, _encoder: &Encoder) -> Vec<u8> {
        self.to_vec_u8(8)
    }
}

impl Encode<AsciiString> for EncodeUnit {
    fn encode_to(&self, _encoder: &Encoder) -> AsciiString {
        let encode_vec = self.to_vec_u8(7);
        let mut encode_str = AsciiString::with_capacity(encode_vec.len());
        for char_value in encode_vec {
            encode_str.push(char_value.to_ascii_char().unwrap());
        }
        encode_str
    }
}

impl Encode<Vec<u8>> for Pattern {
    fn encode_to(&self, encoder: &Encoder) -> Vec<u8> {
        // Initialize an BTreeSet to Store the Encoding Units
        let mut set = BTreeSet::new();
        // Encode Each Edge in the Pattern as an Encoding Unit
        let edges = self.get_edges();
        for (_, edge) in edges.iter() {
            let encode_unit = EncodeUnit::from_pattern_edge(self, edge, encoder);
            let encode_vec: Vec<u8> = encode_unit.encode_to(encoder);
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

impl Encode<AsciiString> for Pattern {
    fn encode_to(&self, encoder: &Encoder) -> AsciiString {
        // Initialize an BTreeSet to Store the Encoding Units
        let mut set = BTreeSet::new();
        // Encode Each Edge in the Pattern as an Encoding Unit
        let edges = self.get_edges();
        for (_, edge) in edges.iter() {
            let encode_unit = EncodeUnit::from_pattern_edge(self, edge, encoder);
            let encode_str: AsciiString = encode_unit.encode_to(encoder);
            set.insert(encode_str);
        }

        let mut encode_str = AsciiString::new();
        let mut set_iter = set.iter();
        loop {
            match set_iter.next() {
                Some(value) => encode_str = encode_str + value,
                None => break,
            }
        }
        encode_str
    }
}

impl Encode<Vec<u8>> for ExtendStep {
    fn encode_to(&self, encoder: &Encoder) -> Vec<u8> {
        let mut encode_vec = Vec::new();
        encode_vec.push(self.get_target_v_label() as u8);
        for (_, extend_edges) in self.iter() {
            for extend_edge in extend_edges {
                let edge_code_vec: Vec<u8> =
                    EncodeUnit::from_extend_edge(extend_edge, encoder).encode_to(encoder);
                encode_vec.extend(&edge_code_vec);
            }
        }
        encode_vec
    }
}

impl Encode<AsciiString> for ExtendStep {
    fn encode_to(&self, encoder: &Encoder) -> AsciiString {
        let mut encode_str = AsciiString::new();
        encode_str.push(
            (self.get_target_v_label() as u8)
                .to_ascii_char()
                .unwrap(),
        );
        for (_, extend_edges) in self.iter() {
            for extend_edge in extend_edges {
                let edge_code_str: AsciiString =
                    EncodeUnit::from_extend_edge(extend_edge, encoder).encode_to(encoder);
                encode_str.push_str(&edge_code_str);
            }
        }
        encode_str
    }
}

// impl Decode<Vec<u8>> for ExtendStep {
//     fn decode_from(src_code: Vec<u8>, encoder: &Encoder) -> Self {
//         let vertex_label_bit_num = encoder.get_vertex_label_bit_num();
//         let vertex_index_bit_num = encoder.get_vertex_index_bit_num();
//         let edge_label_bit_num = encoder.get_edge_label_bit_num();
//         let direction_bit_num = encoder.get_direction_bit_num();

//         let bit_per_extend_edge = vertex_label_bit_num + vertex_index_bit_num + edge_label_bit_num + direction_bit_num;
//         let src_code_bit_sum = 8 * src_code.len();
//         let mut unit_tail = 0;
//         let mut unit_head = bit_per_extend_edge - 1;
//         let mut extend_edges = Vec::new();
//         while unit_head < src_code_bit_sum {
//             let dir_head = unit_tail+direction_bit_num-1;
//             let edge_label_head = unit_tail+direction_bit_num+edge_label_bit_num-1;
//             let start_v_index_head = unit_head - vertex_label_bit_num;
//             let start_v_label_head = unit_head;
//             let dir_tail = unit_tail;
//             let edge_label_tail = unit_tail + direction_bit_num;

//             let dir = (Encoder::get_decode_value_by_head_tail(&src_code, unit_tail+direction_bit_num-1, unit_tail, 8)) as Direction;
//             let edge_label = Encoder::get_decode_value_by_head_tail(&src_code, unit_tail+direction_bit_num-1, unit_tail+direction_bit_num, 8);
//         }
//         ExtendStep::from((0, vec![]))
//     }
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
        let edge1 = pattern.get_edge_from_id(0);
        let edge2 = pattern.get_edge_from_id(1);
        let encoder = Encoder::initialize_from_pattern(&pattern, 5);
        let encode_unit_1 = EncodeUnit::from_pattern_edge(&pattern, edge1, &encoder);
        assert_eq!(encode_unit_1.values[0], 1);
        assert_eq!(encode_unit_1.values[1], 1);
        assert_eq!(encode_unit_1.values[2], 2);
        assert_eq!(encode_unit_1.values[3], 0);
        assert_eq!(encode_unit_1.values[4], 0);
        let encode_unit_2 = EncodeUnit::from_pattern_edge(&pattern, edge2, &encoder);
        assert_eq!(encode_unit_2.values[0], 2);
        assert_eq!(encode_unit_2.values[1], 1);
        assert_eq!(encode_unit_2.values[2], 3);
        assert_eq!(encode_unit_2.values[3], 0);
        assert_eq!(encode_unit_2.values[4], 0);
    }

    #[test]
    fn test_initialize_encoder_from_parameter_case1() {
        let encoder = Encoder::initialize(2, 3, 4, 5);
        assert_eq!(encoder.edge_label_bit_num, 2);
        assert_eq!(encoder.vertex_label_bit_num, 3);
        assert_eq!(encoder.direction_bit_num, 4);
        assert_eq!(encoder.vertex_index_bit_num, 5);
    }

    #[test]
    fn test_initialize_encoder_from_parameter_case2() {
        let encoder = Encoder::initialize(2, 2, 2, 2);
        assert_eq!(encoder.edge_label_bit_num, 2);
        assert_eq!(encoder.vertex_label_bit_num, 2);
        assert_eq!(encoder.direction_bit_num, 2);
        assert_eq!(encoder.vertex_index_bit_num, 2);
    }

    #[test]
    fn test_initialize_encoder_from_pattern_case1() {
        let pattern = build_pattern_testcase_1();
        let default_vertex_index_bit_num = 0;
        let encoder = Encoder::initialize_from_pattern(&pattern, default_vertex_index_bit_num);
        assert_eq!(encoder.edge_label_bit_num, 1);
        assert_eq!(encoder.vertex_label_bit_num, 2);
        assert_eq!(encoder.direction_bit_num, 2);
        assert_eq!(encoder.vertex_index_bit_num, 1);
    }

    #[test]
    fn test_initialize_encoder_from_pattern_case2() {
        let pattern = build_pattern_testcase_2();
        let default_vertex_index_bit_num = 2;
        let encoder = Encoder::initialize_from_pattern(&pattern, default_vertex_index_bit_num);
        assert_eq!(encoder.edge_label_bit_num, 3);
        assert_eq!(encoder.vertex_label_bit_num, 2);
        assert_eq!(encoder.direction_bit_num, 2);
        assert_eq!(encoder.vertex_index_bit_num, 2);
    }

    #[test]
    fn encode_unit_to_ascii_string() {
        let pattern = build_pattern_testcase_1();
        let edge1 = pattern.get_edge_from_id(0);
        let edge2 = pattern.get_edge_from_id(1);
        let encoder = Encoder::initialize(2, 2, 2, 2);
        let encode_unit_1 = EncodeUnit::from_pattern_edge(&pattern, edge1, &encoder);
        let encode_string_1 = <EncodeUnit as Encode<AsciiString>>::encode_to(&encode_unit_1, &encoder);
        let expected_encode_string_1: AsciiString = generate_asciistring_from_vec(&vec![2, 96]);
        assert_eq!(encode_string_1.len(), 2);
        assert_eq!(encode_string_1, expected_encode_string_1);
        let encode_unit_2 = EncodeUnit::from_pattern_edge(&pattern, edge2, &encoder);
        let encode_string_2 = <EncodeUnit as Encode<AsciiString>>::encode_to(&encode_unit_2, &encoder);
        let expected_encode_string_2: AsciiString = generate_asciistring_from_vec(&vec![4, 112]);
        assert_eq!(encode_string_2.len(), 2);
        assert_eq!(encode_string_2, expected_encode_string_2);
    }

    #[test]
    fn encode_unit_to_vec_u8() {
        let pattern = build_pattern_testcase_1();
        let edge1 = pattern.get_edge_from_id(0);
        let edge2 = pattern.get_edge_from_id(1);
        let encoder = Encoder::initialize(2, 2, 2, 2);
        let encode_unit_1 = EncodeUnit::from_pattern_edge(&pattern, edge1, &encoder);
        let encode_vec_1 = <EncodeUnit as Encode<Vec<u8>>>::encode_to(&encode_unit_1, &encoder);
        let expected_encode_vec_1: Vec<u8> = vec![1, 96];
        assert_eq!(encode_vec_1.len(), 2);
        assert_eq!(encode_vec_1, expected_encode_vec_1);
        let encode_unit_2 = EncodeUnit::from_pattern_edge(&pattern, edge2, &encoder);
        let encode_vec_2 = <EncodeUnit as Encode<Vec<u8>>>::encode_to(&encode_unit_2, &encoder);
        let expected_encode_vec_2: Vec<u8> = vec![2, 112];
        assert_eq!(encode_vec_2.len(), 2);
        assert_eq!(encode_vec_2, expected_encode_vec_2);
    }

    #[test]
    fn encode_pattern_to_asciistring_case_1() {
        let pattern = build_pattern_testcase_1();
        let encoder = Encoder::initialize(2, 2, 2, 2);
        let encode_value = <Pattern as Encode<AsciiString>>::encode_to(&pattern, &encoder);
        let expected_encode_string_1: AsciiString = generate_asciistring_from_vec(&vec![2, 96]);
        let expected_encode_string_2: AsciiString = generate_asciistring_from_vec(&vec![4, 112]);
        let expected_encode_value = expected_encode_string_1 + &expected_encode_string_2;
        assert_eq!(encode_value, expected_encode_value);
    }

    #[test]
    fn encode_pattern_to_vec_u8_case_1() {
        let pattern = build_pattern_testcase_1();
        let encoder = Encoder::initialize(2, 2, 2, 2);
        let encode_vec = <Pattern as Encode<Vec<u8>>>::encode_to(&pattern, &encoder);
        let mut expected_encode_vec_1: Vec<u8> = vec![1, 96];
        let expected_encode_vec_2: Vec<u8> = vec![2, 112];
        expected_encode_vec_1.extend(expected_encode_vec_2);
        assert_eq!(encode_vec, expected_encode_vec_1);
    }

    #[test]
    fn test_get_decode_value_by_head_tail_vec8() {
        let src_code: Vec<u8> = vec![95, 115, 87];
        // inside one unit
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 0, 0, 8);
        assert_eq!(picked_value, 1);
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 15, 15, 8);
        assert_eq!(picked_value, 0);
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 1, 0, 8);
        assert_eq!(picked_value, 3);
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 2, 1, 8);
        assert_eq!(picked_value, 3);
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 7, 0, 8);
        assert_eq!(picked_value, 87);
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 15, 8, 8);
        assert_eq!(picked_value, 115);
        // neighboring units
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 8, 7, 8);
        assert_eq!(picked_value, 2);
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 9, 6, 8);
        assert_eq!(picked_value, 13);
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 8, 4, 8);
        assert_eq!(picked_value, 21);
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 13, 4, 8);
        assert_eq!(picked_value, 821);
        // crossing one unit
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 16, 7, 8);
        assert_eq!(picked_value, 742);
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 16, 6, 8);
        assert_eq!(picked_value, 1485);
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 18, 2, 8);
        assert_eq!(picked_value, 122069);
    }

    #[test]
    fn test_get_decode_value_by_head_tail_asciistring() {
        let src_code: Vec<u8> = vec![95, 115, 87];
        // inside one unit
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 0, 0, 7);
        assert_eq!(picked_value, 1);
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 13, 13, 7);
        assert_eq!(picked_value, 1);
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 1, 0, 7);
        assert_eq!(picked_value, 3);
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 2, 1, 7);
        assert_eq!(picked_value, 3);
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 6, 0, 7);
        assert_eq!(picked_value, 87);
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 13, 7, 7);
        assert_eq!(picked_value, 115);
        // neighboring units
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 7, 6, 7);
        assert_eq!(picked_value, 3);
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 9, 6, 7);
        assert_eq!(picked_value, 7);
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 8, 4, 7);
        assert_eq!(picked_value, 29);
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 13, 4, 7);
        assert_eq!(picked_value, 925);
        // crossing one unit
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 14, 6, 7);
        assert_eq!(picked_value, 487);
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 16, 6, 7);
        assert_eq!(picked_value, 2023);
        let picked_value = Encoder::get_decode_value_by_head_tail(&src_code, 18, 2, 7);
        assert_eq!(picked_value, 130677);
    }

    #[test]
    fn test_decode_from_encode_unit_to_vec_u8() {
        let pattern = build_pattern_testcase_1();
        let edge1 = pattern.get_edge_from_id(0);
        let edge2 = pattern.get_edge_from_id(1);
        let encoder = Encoder::initialize(2, 2, 2, 2);
        let encode_unit_1 = EncodeUnit::from_pattern_edge(&pattern, edge1, &encoder);
        let encode_vec_1 = <EncodeUnit as Encode<Vec<u8>>>::encode_to(&encode_unit_1, &encoder);
        let encode_unit_2 = EncodeUnit::from_pattern_edge(&pattern, edge2, &encoder);
        let encode_vec_2 = <EncodeUnit as Encode<Vec<u8>>>::encode_to(&encode_unit_2, &encoder);
        assert_eq!(Encoder::get_decode_value_by_head_tail(&encode_vec_1, 9, 8, 8), edge1.get_label());
        assert_eq!(
            Encoder::get_decode_value_by_head_tail(&encode_vec_1, 7, 6, 8),
            edge1.get_start_vertex_label()
        );
        assert_eq!(
            Encoder::get_decode_value_by_head_tail(&encode_vec_1, 5, 4, 8),
            edge1.get_end_vertex_label()
        );
        assert_eq!(
            Encoder::get_decode_value_by_head_tail(&encode_vec_1, 3, 2, 8),
            pattern.get_vertex_index(edge1.get_start_vertex_id())
        );
        assert_eq!(
            Encoder::get_decode_value_by_head_tail(&encode_vec_1, 1, 0, 8),
            pattern.get_vertex_index(edge1.get_end_vertex_id())
        );
        assert_eq!(Encoder::get_decode_value_by_head_tail(&encode_vec_2, 9, 8, 8), edge2.get_label());
        assert_eq!(
            Encoder::get_decode_value_by_head_tail(&encode_vec_2, 7, 6, 8),
            edge2.get_start_vertex_label()
        );
        assert_eq!(
            Encoder::get_decode_value_by_head_tail(&encode_vec_2, 5, 4, 8),
            edge2.get_end_vertex_label()
        );
        assert_eq!(
            Encoder::get_decode_value_by_head_tail(&encode_vec_2, 3, 2, 8),
            pattern.get_vertex_index(edge2.get_start_vertex_id())
        );
        assert_eq!(
            Encoder::get_decode_value_by_head_tail(&encode_vec_2, 1, 0, 8),
            pattern.get_vertex_index(edge2.get_end_vertex_id())
        );
    }

    #[test]
    fn test_decode_from_encode_unit_to_asciistring() {
        let pattern = build_pattern_testcase_1();
        let edge1 = pattern.get_edge_from_id(0);
        let edge2 = pattern.get_edge_from_id(1);
        let encoder = Encoder::initialize(2, 2, 2, 2);
        let encode_unit_1 = EncodeUnit::from_pattern_edge(&pattern, edge1, &encoder);
        let encode_string_1 = <EncodeUnit as Encode<AsciiString>>::encode_to(&encode_unit_1, &encoder);
        let encode_unit_2 = EncodeUnit::from_pattern_edge(&pattern, edge2, &encoder);
        let encode_string_2 = <EncodeUnit as Encode<AsciiString>>::encode_to(&encode_unit_2, &encoder);
        let encode_vec_1: Vec<u8> = encode_string_1
            .into_iter()
            .map(|ch| ch.as_byte())
            .collect();
        let encode_vec_2: Vec<u8> = encode_string_2
            .into_iter()
            .map(|ch| ch.as_byte())
            .collect();
        assert_eq!(Encoder::get_decode_value_by_head_tail(&encode_vec_1, 9, 8, 7), edge1.get_label());
        assert_eq!(
            Encoder::get_decode_value_by_head_tail(&encode_vec_1, 7, 6, 7),
            edge1.get_start_vertex_label()
        );
        assert_eq!(
            Encoder::get_decode_value_by_head_tail(&encode_vec_1, 5, 4, 7),
            edge1.get_end_vertex_label()
        );
        assert_eq!(
            Encoder::get_decode_value_by_head_tail(&encode_vec_1, 3, 2, 7),
            pattern.get_vertex_index(edge1.get_start_vertex_id())
        );
        assert_eq!(
            Encoder::get_decode_value_by_head_tail(&encode_vec_1, 1, 0, 7),
            pattern.get_vertex_index(edge1.get_end_vertex_id())
        );
        assert_eq!(Encoder::get_decode_value_by_head_tail(&encode_vec_2, 9, 8, 7), edge2.get_label());
        assert_eq!(
            Encoder::get_decode_value_by_head_tail(&encode_vec_2, 7, 6, 7),
            edge2.get_start_vertex_label()
        );
        assert_eq!(
            Encoder::get_decode_value_by_head_tail(&encode_vec_2, 5, 4, 7),
            edge2.get_end_vertex_label()
        );
        assert_eq!(
            Encoder::get_decode_value_by_head_tail(&encode_vec_2, 3, 2, 7),
            pattern.get_vertex_index(edge2.get_start_vertex_id())
        );
        assert_eq!(
            Encoder::get_decode_value_by_head_tail(&encode_vec_2, 1, 0, 7),
            pattern.get_vertex_index(edge2.get_end_vertex_id())
        );
    }
}
