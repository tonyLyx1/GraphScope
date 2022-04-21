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
    edge_label_bit_num: u8,
    vertex_label_bit_num: u8,
    direction_bit_num: u8,
    vertex_index_bit_num: u8,
}

impl Encoder {
    /// ### Initialize Encoder with User Definded Parameters
    pub fn initialize(
        edge_label_bit_num: u8, vertex_label_bit_num: u8, edge_direction_bit_num: u8,
        vertex_index_bit_num: u8,
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
            direction_bit_num: edge_direction_bit_num,
            vertex_index_bit_num: min_vertex_index_bit_num,
        }
    }

    /// ### Compute the u8 value for each storage unit (AsciiChar or u8)
    pub fn get_encode_numerical_value(
        value: i32, value_head: u8, value_tail: u8, storage_unit_valid_bit_num: u8, storage_unit_index: u8,
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
    pub fn get_edge_label_bit_num(&self) -> u8 {
        self.edge_label_bit_num
    }

    pub fn get_vertex_label_bit_num(&self) -> u8 {
        self.vertex_label_bit_num
    }

    pub fn get_direction_bit_num(&self) -> u8 {
        self.direction_bit_num
    }

    pub fn get_vertex_index_bit_num(&self) -> u8 {
        self.vertex_index_bit_num
    }
}

pub struct EncodeUnit {
    values: Vec<i32>,
    heads: Vec<u8>,
    tails: Vec<u8>,
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

    pub fn to_vec_u8(&self, storage_unit_bit_num: u8) -> Vec<u8> {
        let storage_unit_num = self.heads[0] / storage_unit_bit_num + 1;
        let mut encode_vec = Vec::with_capacity(storage_unit_num as usize);
        for i in (0..storage_unit_bit_num).rev() {
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

    pub fn get_bits_num(&self) -> u8 {
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
