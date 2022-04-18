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

use ascii::AsciiString;

use super::pattern::{Pattern, PatternEdge};
pub trait Encode<T> {
    fn encode_to(&self, encoder: &Encoder) -> T;
}

pub trait Decode<T>: Encode<T> {
    fn decode_from(src_code: T, encoder: &Encoder) -> Self;
}

pub struct Encoder {
    edge_label_bit_num: u8,
    vertex_label_bit_num: u8,
    edge_direction_bit_num: u8,
    vertex_index_bit_num: u8,
    bit_per_ascii_char: u8,
    ascii_char_num_per_encode_unit: u8,
}

impl Encode<Vec<u8>> for PatternEdge {
    fn encode_to(&self, _encoder: &Encoder) -> Vec<u8> {
        let mut code = Vec::with_capacity(3);
        code.push(self.label as u8);
        code.push(self.start_v_label as u8);
        code.push(self.end_v_label as u8);
        code
    }
}

impl Decode<Vec<u8>> for PatternEdge {
    fn decode_from(src_code: Vec<u8>, _encoder: &Encoder) -> Self {
        PatternEdge {
            id: -1,
            label: src_code[0] as i32,
            start_v_id: -1,
            end_v_id: -1,
            start_v_label: src_code[1] as i32,
            end_v_label: src_code[2] as i32,
        }
    }
}

impl Encode<AsciiString> for PatternEdge {
    fn encode_to(&self, encoder: &Encoder) -> AsciiString {
        AsciiString::new()
    }
}

impl Decode<AsciiString> for PatternEdge {
    fn decode_from(src_code: AsciiString, encoder: &Encoder) -> Self {
        PatternEdge { id: 0, label: 0, start_v_id: 0, end_v_id: 0, start_v_label: 0, end_v_label: 0 }
    }
}
