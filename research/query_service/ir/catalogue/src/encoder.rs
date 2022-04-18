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
use crate::codec::{Encode, Decode};
use ascii::{self, ToAsciiChar, AsciiChar, AsciiString};

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
		let edge = pattern.get_edge_from_id(*edge_index);
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

	/// ### Setter of end_v_label
	pub fn set_end_v_label(&mut self, end_v_label: u64) {
		self.end_v_label = end_v_label;
	}

	/// ### Setter of end_v_index
	pub fn set_end_v_index(&mut self, end_v_index: u64) {
		self.end_v_index = end_v_index;
	}
}

/// Getters
impl EncodeUnit {
	pub fn get_all_member_variable(&self) -> (u64, u64, u64, u64, u64, u64) {
		(
			self.edge_label,
			self.start_v_label,
			self.end_v_label,
			self.edge_direction.to_u8() as u64,
			self.start_v_index,
			self.end_v_index,
		)
	}
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
			edge_direction_bit_num,
			vertex_index_bit_num: min_vertex_index_bit_num,
		}
	}

	/// ### Compute the u8 value for each storage unit (AsciiChar or u8)
	pub fn get_encode_numerical_value(
		&self, value: u64,
		value_head: u8, value_tail: u8,
		storage_unit_valid_bit_num: u8, storage_unit_index: u8
	) -> u8 {
		let mut output: u64;
		let char_tail = storage_unit_index * storage_unit_valid_bit_num;
		let char_head = (storage_unit_index + 1) * storage_unit_valid_bit_num - 1;
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
			output = output * (1 << (storage_unit_valid_bit_num - shift_bit_num));
		}
		else if value_tail < char_tail && value_head > char_head {
			let right_shift_bit_num = char_tail - value_tail;
			output = value % (1 << right_shift_bit_num);
			output = output % (1 << storage_unit_valid_bit_num);
		}
		else {
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

/// Unit Testing
#[cfg(test)]
mod tests {
	use crate::encoder::*;
	use crate::pattern::*;
	use crate::extend_step::*;
	use crate::codec::{Encode, Decode};
	use crate::pattern_edge::*;
	use ascii::{self, ToAsciiChar, AsciiString};

	fn build_pattern_testcase_1() -> Pattern {
		let pattern_edge1 = PatternEdge::create(0, 1, 0, 1, 1, 2);
		let pattern_edge2 = PatternEdge::create(1, 2, 0, 2, 1, 3);
		let pattern_vec = vec![pattern_edge1, pattern_edge2];
		Pattern::from(pattern_vec)
	}

	fn build_pattern_testcase_2() -> Pattern {
		let edge_1 = PatternEdge::create(0, 1, 0, 1, 1, 2);
		let edge_2 = PatternEdge::create(1, 2, 0, 2, 1, 3);
		let edge_3 = PatternEdge::create(2, 3, 1, 2, 2, 3);
		let edge_4 = PatternEdge::create(3, 4, 0, 3, 1, 4);
		let edge_5 = PatternEdge::create(4, 5, 1, 3, 2, 4);
		let edge_6 = PatternEdge::create(5, 6, 3, 2, 4, 3);
		let pattern_edges = vec![edge_1, edge_2, edge_3, edge_4, edge_5, edge_6];
		Pattern::from(pattern_edges)
	}

	#[test]
	fn test_create_encode_unit_from_edge() {
		let pattern = build_pattern_testcase_1();
		let encode_unit_1 = pattern.to_encode_unit_by_edge_id(0);
		assert_eq!(encode_unit_1.edge_label, 1);
		assert_eq!(encode_unit_1.start_v_label, 1);
		assert_eq!(encode_unit_1.end_v_label, 2);
		assert_eq!(encode_unit_1.edge_direction, Direction::Out);
		assert_eq!(encode_unit_1.start_v_index, 0);
		assert_eq!(encode_unit_1.end_v_index, 0);
		let encode_unit_2 = pattern.to_encode_unit_by_edge_id(1);
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
	fn test_encode_ascii_string_from_encode_unit() {
		let pattern = build_pattern_testcase_1();
		let encoder = Encoder::initialize(2, 2, 2, 2);
		let encode_unit_1 = pattern.to_encode_unit_by_edge_id(0);
		let encode_string_1 = <EncodeUnit as Encode<AsciiString>>::encode_to(&encode_unit_1, &encoder);
		let mut expected_encode_string_1: AsciiString = AsciiString::new();
		let ascii_char_1 = (11 as u8).to_ascii_char().unwrap();
		let ascii_char_2 = (0 as u8).to_ascii_char().unwrap();
		expected_encode_string_1.push(ascii_char_1);
		expected_encode_string_1.push(ascii_char_2);
		assert_eq!(encode_string_1.len(), 2);
		assert_eq!(encode_string_1, expected_encode_string_1);
		let encode_unit_2 = pattern.to_encode_unit_by_edge_id(1);
		let encode_string_2 = <EncodeUnit as Encode<AsciiString>>::encode_to(&encode_unit_2, &encoder);
		let mut expected_encode_string_2: AsciiString = AsciiString::new();
		let ascii_char_1 = (19 as u8).to_ascii_char().unwrap();
		let ascii_char_2 = (64 as u8).to_ascii_char().unwrap();
		expected_encode_string_2.push(ascii_char_1);
		expected_encode_string_2.push(ascii_char_2);
		assert_eq!(encode_string_2.len(), 2);
		assert_eq!(encode_string_2, expected_encode_string_2);
	}

	#[test]
	fn test_pattern_encode_value_case_1() {
		let pattern = build_pattern_testcase_1();
		let encoder = Encoder::initialize(2, 2, 2, 2);
		let encode_value = <Pattern as Encode<AsciiString>>::encode_to(&pattern, &encoder);
		let mut expected_encode_string_1: AsciiString = AsciiString::new();
		let ascii_char_1 = (11 as u8).to_ascii_char().unwrap();
		let ascii_char_2 = (0 as u8).to_ascii_char().unwrap();
		expected_encode_string_1.push(ascii_char_1);
		expected_encode_string_1.push(ascii_char_2);
		let mut expected_encode_string_2: AsciiString = AsciiString::new();
		let ascii_char_1 = (19 as u8).to_ascii_char().unwrap();
		let ascii_char_2 = (64 as u8).to_ascii_char().unwrap();
		expected_encode_string_2.push(ascii_char_1);
		expected_encode_string_2.push(ascii_char_2);
		let expected_encode_value = expected_encode_string_1 + &expected_encode_string_2;

		assert_eq!(encode_value, expected_encode_value);
	}
}

