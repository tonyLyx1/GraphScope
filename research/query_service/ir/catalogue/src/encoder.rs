//
//! ### This module is the implementation of the Unique Pattern Encoding Technique
//! 
//! It contains the following data structures:
//! 1. Edge-Based Encoding Unit
//! 
//! It contains the following functions:
//! 1. Set Vertex Index for the Whole Pattern
//! 2. Encode Pattern
//! 3. Extend Pattern Qk-1 to Qk
//! 4. Top-Down from Qk to Qk-1
//!
//! ### Copyright 2020 Alibaba Group Holding Limited.
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
use crate::pattern::Pattern;
use crate::pattern::Direction;
use ascii::{self, ToAsciiChar, AsciiString};


/// 两个模块：Pattern + Extend
/// 两个trait：encode + decode
/// 两个模块分别实现两个trait


/// ## Edge-Based Encoding Unit
/// ### Member Variables
/// Each encoding unit represents an edge in the pattern and contains:
/// 1. Edge Label
/// 2. Src Vertex Label
/// 3. Dst Vertex Label 
/// 4. Edge Direction
/// 5. Src Vertex Index
/// 6. Dst Vertex Index
/// ### Functions
/// 
#[derive(Debug, Clone)]
pub struct EncodeUnit {
    edge_label: u64,
    start_v_label: u64,
    end_v_label: u64,
    edge_direction: Direction,
    start_v_index: u64,
    end_v_index: u64,
}

impl EncodeUnit {
	/// ### Create a new EncodeUnit with Input Variables
	pub fn new(
		edge_label: u64,
		start_v_label: u64,
		end_v_label: u64,
		edge_direction: Direction,
		start_v_index: u64,
		end_v_index: u64,
	) -> EncodeUnit {
		EncodeUnit {
			edge_label,
			start_v_label,
			end_v_label,
			edge_direction,
			start_v_index,
			end_v_index,
		}
	}

	/// ### Create EncodeUnit of an Edge
	pub fn get_edge_encode_unit(pattern: &Pattern, edge_index: &u64) -> EncodeUnit {
		let edge = pattern.get_edge_from_edge_index(*edge_index);
		let (start_v_label, end_v_label) = edge.get_edge_vertices_label();
		let (start_v_index, end_v_index) = pattern.get_edge_vertices_order(*edge_index);
		
		EncodeUnit {
			edge_label: edge.get_edge_label(),
			start_v_label,
			end_v_label,
			edge_direction: Direction::Out,
			start_v_index,
			end_v_index,
		}
	}
}


/// ## Unique Pattern Identity Encoder
/// ### Member Variables
/// It contains the bit number that each variable in the encoding unit occupies
/// ### Functions
/// 
pub struct Encoder {
	edge_label_bit_num: u8,
	vertex_label_bit_num: u8,
	edge_direction_bit_num: u8,
	vertex_index_bit_num: u8,
	bit_per_ascii_char: u8,
	ascii_char_num_per_encode_unit: u8,
}

impl Encoder {
	pub fn initialize(
		edge_label_bit_num: u8,
		vertex_label_bit_num: u8,
		edge_direction_bit_num: u8,
		vertex_index_bit_num: u8,
	) -> Encoder {
		let sum_bit_num = edge_label_bit_num + vertex_label_bit_num * 2 + edge_direction_bit_num + vertex_index_bit_num * 2;
		let bit_per_ascii_char = 7;
		let ascii_char_num_per_encode_unit = (sum_bit_num - 1) / bit_per_ascii_char + 1;

		Encoder {
			edge_label_bit_num,
			vertex_label_bit_num,
			edge_direction_bit_num,
			vertex_index_bit_num,
			bit_per_ascii_char,
			ascii_char_num_per_encode_unit,
		}
	}

	/// ### Transfer the Encoding Unit into a String
	fn encode_unit_to_ascii_string(&self, unit: &EncodeUnit) -> AsciiString {
		// 根据Encoder的比特位参数来决定生成string的格式
		let mut encode_str = AsciiString::new();
		let ascii_char_num_per_encode_unit = self.ascii_char_num_per_encode_unit;
		// Compute the final bit of each variable in the encode unit
		let end_v_index_end_bit = 0;
		let start_v_index_end_bit = end_v_index_end_bit + self.vertex_index_bit_num;
		let edge_direction_end_bit = start_v_index_end_bit + self.vertex_index_bit_num;
		let end_v_label_end_bit = edge_direction_end_bit + self.edge_direction_bit_num;
		let start_v_label_end_bit = end_v_label_end_bit + self.vertex_label_bit_num;
		let edge_label_end_bit = start_v_label_end_bit + self.vertex_label_bit_num;
		// Compute each ASCII char
		for i in 0..ascii_char_num_per_encode_unit {
			let value: u8 = self.encode_unit_to_ascii_string_fill_char(unit.edge_label, self.edge_label_bit_num, edge_label_end_bit, i)
										+ self.encode_unit_to_ascii_string_fill_char(unit.start_v_label, self.vertex_label_bit_num, start_v_label_end_bit, i)
										+ self.encode_unit_to_ascii_string_fill_char(unit.end_v_label, self.vertex_label_bit_num, end_v_label_end_bit, i)
										+ self.encode_unit_to_ascii_string_fill_char(Direction::to_u8(&unit.edge_direction) as u64, self.edge_direction_bit_num, edge_direction_end_bit, i)
										+ self.encode_unit_to_ascii_string_fill_char(unit.start_v_index, self.vertex_index_bit_num, start_v_index_end_bit, i)
										+ self.encode_unit_to_ascii_string_fill_char(unit.end_v_index, self.vertex_index_bit_num, end_v_index_end_bit, i);
			println!("index {} has value {}", i, value);
			encode_str.push(value.to_ascii_char().unwrap());
		}

		encode_str
	}

	fn encode_unit_to_ascii_string_fill_char(&self, value: u64, bit_num: u8, end_bit: u8, char_num: u8) -> u8 {
		let mut output: u64;
		let bit_per_ascii_char = self.bit_per_ascii_char;
		let char_tail = char_num * bit_per_ascii_char;
		let char_head = (char_num + 1) * bit_per_ascii_char - 1;
		let value_tail = end_bit;
		let value_head = end_bit + bit_num - 1;
		if value_tail > char_head || value_head < char_tail {
			output = 0;
		}
		else if value_tail >= char_tail && value_head <= char_head {
			let offset_bit_num = value_tail - char_tail;
			output = value * (1 << offset_bit_num);
		}
		else if value_tail < char_tail && value_head <= char_head {
			let shift_bit_num = char_tail - value_tail;
			output = value / (1 << shift_bit_num);
		}
		else if value_tail >= char_tail && value_head > char_head {
			let shift_bit_num = char_head + 1 - value_tail;
			output = value % (1 << shift_bit_num);
			output = output * (1 << (self.bit_per_ascii_char - shift_bit_num));
		}
		else if value_tail < char_tail && value_head > char_head {
			let right_shift_bit_num = char_tail - value_tail;
			output = value % (1 << right_shift_bit_num);
			output = output % (1 << self.bit_per_ascii_char);
		}
		else {
			panic!("Error in Converting Encode Unit to ASCII String: No Such Value Exists");
		}

		return output as u8;
	}


	/// ### Set Initial Vertex Index Based on Comparison of Labels
	pub fn set_initial_vertex_index(&self, pattern: &mut Pattern) {
		// To Be Completed
	}

	/// ### Set Accurate Vertex Index based on Initial Vertex Indices
	pub fn set_accurate_vertex_index(&self, pattern: &mut Pattern) {
		// To be Completed
	}

	/// Set Vertex Indices
	pub fn set_vertex_index(&self, pattern: &mut Pattern) {
		// Set Initial Indices First
		self.set_initial_vertex_index(pattern);
		// Set Accurate Indices
		self.set_accurate_vertex_index(pattern);
	}

	/// Encode a Pattern into a String
	pub fn encode_pattern(&self, pattern: &Pattern) -> AsciiString {
		// Initialize an BTreeSet to Store the Encoding Units
		let mut set = BTreeSet::from([]);
		// Encode Each Edge in the Pattern as an Encoding Unit
		let edges = pattern.get_edges();
		for (edge_id, _) in edges.iter() {
			let encode_unit: EncodeUnit = EncodeUnit::get_edge_encode_unit(pattern, edge_id);
			let encode_string: AsciiString = self.encode_unit_to_ascii_string(&encode_unit);
			set.insert(encode_string);
		}

		let mut encode_value = AsciiString::new();
		let mut set_iter = set.iter();
		loop {
			match set_iter.next() {
				Some(value) => encode_value = encode_value + &value,
				None => break
			}
		}

		println!("Encode Value: {:?}", encode_value);

		encode_value
	}
}



/// Unit Testing
#[cfg(test)]
mod tests {
	use super::{Encoder, EncodeUnit};
	use std::cmp::Ordering;
	use std::collections::{BTreeMap, BTreeSet, HashMap};
	use crate::pattern::{PatternEdge, Pattern};
	use crate::pattern::Direction;
	use ascii::{self, ToAsciiChar, AsciiString};

	fn build_pattern_testcase_1() -> Pattern {
		let pattern_edge1 = PatternEdge::create(0, 1, 0, 0, 1, 2);
		let pattern_edge2 = PatternEdge::create(1, 2, 0, 0, 1, 3);
		let pattern_vec = vec![pattern_edge1, pattern_edge2];
		Pattern::from(pattern_vec)
	}

	fn build_pattern_testcase_2() -> Pattern {
		let edge_1 = PatternEdge::create(0, 1, 0, 0, 1, 2);
		let edge_2 = PatternEdge::create(1, 2, 0, 0, 1, 3);
		let edge_3 = PatternEdge::create(2, 3, 0, 0, 2, 3);
		let edge_4 = PatternEdge::create(3, 4, 0, 0, 1, 4);
		let edge_5 = PatternEdge::create(4, 5, 0, 0, 2, 4);
		let edge_6 = PatternEdge::create(5, 6, 0, 0, 4, 3);
		let pattern_edges = vec![edge_1, edge_2, edge_3, edge_4, edge_5, edge_6];
		Pattern::from(pattern_edges)
	}

	#[test]
	fn test_create_encode_unit_from_edge() {
		let pattern = build_pattern_testcase_1();
		let encode_unit_1 = EncodeUnit::get_edge_encode_unit(&pattern, &0);
		assert_eq!(encode_unit_1.edge_label, 1);
		assert_eq!(encode_unit_1.start_v_label, 1);
		assert_eq!(encode_unit_1.end_v_label, 2);
		assert_eq!(encode_unit_1.edge_direction, Direction::Out);
		assert_eq!(encode_unit_1.start_v_index, 0);
		assert_eq!(encode_unit_1.end_v_index, 0);
		let encode_unit_2 = EncodeUnit::get_edge_encode_unit(&pattern, &1);
		assert_eq!(encode_unit_2.edge_label, 2);
		assert_eq!(encode_unit_2.start_v_label, 1);
		assert_eq!(encode_unit_2.end_v_label, 3);
		assert_eq!(encode_unit_2.edge_direction, Direction::Out);
		assert_eq!(encode_unit_2.start_v_index, 0);
		assert_eq!(encode_unit_2.end_v_index, 0);
	}

	#[test]
	fn test_initialize_encoder() {
		let encoder = Encoder::initialize(2, 3, 4, 5);
		assert_eq!(encoder.edge_label_bit_num, 2);
		assert_eq!(encoder.vertex_label_bit_num, 3);
		assert_eq!(encoder.edge_direction_bit_num, 4);
		assert_eq!(encoder.vertex_index_bit_num, 5);
		assert_eq!(encoder.bit_per_ascii_char, 7);
		assert_eq!(encoder.ascii_char_num_per_encode_unit, 4);
	}

	#[test]
	fn test_initialize_encoder_case_2() {
		let encoder = Encoder::initialize(2, 2, 2, 2);
		assert_eq!(encoder.edge_label_bit_num, 2);
		assert_eq!(encoder.vertex_label_bit_num, 2);
		assert_eq!(encoder.edge_direction_bit_num, 2);
		assert_eq!(encoder.vertex_index_bit_num, 2);
		assert_eq!(encoder.bit_per_ascii_char, 7);
		assert_eq!(encoder.ascii_char_num_per_encode_unit, 2);
	}

	#[test]
	fn test_encode_ascii_string_from_encode_unit() {
		let pattern = build_pattern_testcase_1();
		let encoder = Encoder::initialize(2, 2, 2, 2);
		let encode_unit_1 = EncodeUnit::get_edge_encode_unit(&pattern, &0);
		let encode_string_1 = encoder.encode_unit_to_ascii_string(&encode_unit_1);
		let mut expected_encode_string_1: AsciiString = AsciiString::new();
		let ascii_char_1 = (0 as u8).to_ascii_char().unwrap();
		let ascii_char_2 = (11 as u8).to_ascii_char().unwrap();
		expected_encode_string_1.push(ascii_char_1);
		expected_encode_string_1.push(ascii_char_2);
		assert_eq!(encode_string_1.len(), 2);
		assert_eq!(encode_string_1, expected_encode_string_1);
		let encode_unit_2 = EncodeUnit::get_edge_encode_unit(&pattern, &1);
		let encode_string_2 = encoder.encode_unit_to_ascii_string(&encode_unit_2);
		let mut expected_encode_string_2: AsciiString = AsciiString::new();
		let ascii_char_1 = (64 as u8).to_ascii_char().unwrap();
		let ascii_char_2 = (19 as u8).to_ascii_char().unwrap();
		expected_encode_string_2.push(ascii_char_1);
		expected_encode_string_2.push(ascii_char_2);
		assert_eq!(encode_string_2.len(), 2);
		assert_eq!(encode_string_2, expected_encode_string_2);
	}

	
	#[test]
	fn test_pattern_encode_value_case_1() {
		let pattern = build_pattern_testcase_1();
		let encoder = Encoder::initialize(2, 2, 2, 2);
		let encode_value = encoder.encode_pattern(&pattern);

		let encode_unit_1 = EncodeUnit::get_edge_encode_unit(&pattern, &0);
		let encode_string_1 = encoder.encode_unit_to_ascii_string(&encode_unit_1);
		let mut expected_encode_string_1: AsciiString = AsciiString::new();
		let ascii_char_1 = (0 as u8).to_ascii_char().unwrap();
		let ascii_char_2 = (11 as u8).to_ascii_char().unwrap();
		expected_encode_string_1.push(ascii_char_1);
		expected_encode_string_1.push(ascii_char_2);
		assert_eq!(encode_string_1.len(), 2);
		assert_eq!(encode_string_1, expected_encode_string_1);
		let encode_unit_2 = EncodeUnit::get_edge_encode_unit(&pattern, &1);
		let encode_string_2 = encoder.encode_unit_to_ascii_string(&encode_unit_2);
		let mut expected_encode_string_2: AsciiString = AsciiString::new();
		let ascii_char_1 = (64 as u8).to_ascii_char().unwrap();
		let ascii_char_2 = (19 as u8).to_ascii_char().unwrap();
		expected_encode_string_2.push(ascii_char_1);
		expected_encode_string_2.push(ascii_char_2);
		let expected_encode_value = expected_encode_string_2 + &expected_encode_string_1;

		assert_eq!(encode_value, expected_encode_value);
	}
}
