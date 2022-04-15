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
/// 
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

	/// ### Compute the AsciiChar of an Encode Unit in a certain Index
	pub fn to_ascii_char(&self, encoder: &Encoder, index: u8) -> AsciiChar {
		let (
			edge_label_bit_num,
			vertex_label_bit_num,
			edge_direction_bit_num,
			vertex_index_bit_num,
			bit_per_ascii_char,
			ascii_char_num_per_encode_unit,
		) = encoder.get_all_member_variables();
		let end_v_index_end_bit = 0;
		let start_v_index_end_bit = end_v_index_end_bit + vertex_index_bit_num;
		let edge_direction_end_bit = start_v_index_end_bit + vertex_index_bit_num;
		let end_v_label_end_bit = edge_direction_end_bit + edge_direction_bit_num;
		let start_v_label_end_bit = end_v_label_end_bit + vertex_label_bit_num;
		let edge_label_end_bit = start_v_label_end_bit + vertex_label_bit_num;
		let value: u8 = encoder.encode_unit_to_ascii_string_fill_char(self.edge_label, edge_label_bit_num, edge_label_end_bit, index)
									+ encoder.encode_unit_to_ascii_string_fill_char(self.start_v_label, vertex_label_bit_num, start_v_label_end_bit, index)
									+ encoder.encode_unit_to_ascii_string_fill_char(self.end_v_label, vertex_label_bit_num, end_v_label_end_bit, index)
									+ encoder.encode_unit_to_ascii_string_fill_char(self.edge_direction.into_u8() as u64, edge_direction_bit_num, edge_direction_end_bit, index)
									+ encoder.encode_unit_to_ascii_string_fill_char(self.start_v_index, vertex_index_bit_num, start_v_index_end_bit, index)
									+ encoder.encode_unit_to_ascii_string_fill_char(self.end_v_index, vertex_index_bit_num, end_v_index_end_bit, index);

		value.to_ascii_char().unwrap()
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


/// ## Unique Pattern Identity Encoder
/// ### Member Variables
/// Contains the bit number that each variable in the encoding unit occupies
#[derive(Debug, Clone)]
pub struct Encoder {
	edge_label_bit_num: u8,
	vertex_label_bit_num: u8,
	edge_direction_bit_num: u8,
	vertex_index_bit_num: u8,
	bit_per_ascii_char: u8,
	ascii_char_num_per_encode_unit: u8,
}

/// Getters
impl Encoder {
	pub fn get_all_member_variables(&self) -> (u8, u8, u8, u8, u8, u8) {
		(
			self.edge_label_bit_num,
			self.vertex_label_bit_num,
			self.edge_direction_bit_num,
			self.vertex_index_bit_num,
			self.bit_per_ascii_char,
			self.ascii_char_num_per_encode_unit,
		)
	}

	pub fn get_ascii_char_num_per_encode_unit(&self) -> u8 {
		self.ascii_char_num_per_encode_unit
	}
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
			bit_per_ascii_char,
			ascii_char_num_per_encode_unit,
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
		let sum_bit_num = min_edge_label_bit_num + min_vertex_label_bit_num * 2 + edge_direction_bit_num + min_vertex_index_bit_num * 2;
		let bit_per_ascii_char = 7;
		let ascii_char_num_per_encode_unit = (sum_bit_num - 1) / bit_per_ascii_char + 1;
		Encoder {
			edge_label_bit_num: min_edge_label_bit_num,
			vertex_label_bit_num: min_vertex_label_bit_num,
			edge_direction_bit_num,
			vertex_index_bit_num: min_vertex_index_bit_num,
			bit_per_ascii_char,
			ascii_char_num_per_encode_unit,
		}
	}

	/// ### Fill Encode Unit Value to a Specific ASCII Char
	/// Callee of encode_unit_to_ascii_string function 
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
}

/// Unit Testing
#[cfg(test)]
#[path = "./tests/encoder.rs"]
mod unit_test;
