#[cfg(test)]
mod unit_test {
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
		assert_eq!(encoder.bit_per_ascii_char, 7);
		assert_eq!(encoder.ascii_char_num_per_encode_unit, 4);
	}

	#[test]
	fn test_initialize_encoder_from_parameter_case2() {
		let encoder = Encoder::initialize(2, 2, 2, 2);
		assert_eq!(encoder.edge_label_bit_num, 2);
		assert_eq!(encoder.vertex_label_bit_num, 2);
		assert_eq!(encoder.edge_direction_bit_num, 2);
		assert_eq!(encoder.vertex_index_bit_num, 2);
		assert_eq!(encoder.bit_per_ascii_char, 7);
		assert_eq!(encoder.ascii_char_num_per_encode_unit, 2);
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
		assert_eq!(encoder.bit_per_ascii_char, 7);
		assert_eq!(encoder.ascii_char_num_per_encode_unit, 2);
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
		assert_eq!(encoder.bit_per_ascii_char, 7);
		assert_eq!(encoder.ascii_char_num_per_encode_unit, 2);
	}

	#[test]
	fn test_encode_ascii_string_from_encode_unit() {
		let pattern = build_pattern_testcase_1();
		let encoder = Encoder::initialize(2, 2, 2, 2);
		let encode_unit_1 = pattern.to_encode_unit_by_edge_id(0);
		let encode_string_1 = encode_unit_1.to_encode_value(&encoder);
		let mut expected_encode_string_1: AsciiString = AsciiString::new();
		let ascii_char_1 = (11 as u8).to_ascii_char().unwrap();
		let ascii_char_2 = (0 as u8).to_ascii_char().unwrap();
		expected_encode_string_1.push(ascii_char_1);
		expected_encode_string_1.push(ascii_char_2);
		assert_eq!(encode_string_1.len(), 2);
		assert_eq!(encode_string_1, expected_encode_string_1);
		let encode_unit_2 = pattern.to_encode_unit_by_edge_id(1);
		let encode_string_2 = encode_unit_2.to_encode_value(&encoder);
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
		let encode_value = pattern.to_encode_value(&encoder);
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
