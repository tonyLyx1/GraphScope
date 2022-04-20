use crate::encoder::*;
use crate::extend_step::*;
use crate::pattern::*;
use ascii::{self, ToAsciiChar, AsciiString};
use std::collections::{BTreeSet};

pub trait Encode<T> {
  fn encode_to(&self, encoder: &Encoder) -> T;
}

pub trait Decode<T>: Encode<T> {
  fn decode_from(src_code: T, encoder: &Encoder) -> Self;
}

impl Encode<AsciiString> for EncodeUnit {
  fn encode_to(&self, encoder: &Encoder) -> AsciiString {
    // 根据Encoder的比特位参数来决定生成string的格式
		let mut encode_str = AsciiString::new();
    let (
      edge_label,
      start_v_label,
      end_v_label,
      edge_dir,
      start_v_index,
      end_v_index,
    ) = self.get_all_member_variable();
    let (
			edge_label_bit_num,
			v_label_bit_num,
			edge_dir_bit_num,
			v_index_bit_num,
		) = encoder.get_all_member_variable();
    let storage_unit_valid_bit_num = 7;
    let ascii_char_num_per_encode_unit = encoder.get_storage_unit_num_per_encode_unit(storage_unit_valid_bit_num);
    // Compute Value head/tail for each field
    let (end_v_index_head, end_v_index_tail) = (v_index_bit_num - 1, 0);
    let (start_v_index_head, start_v_index_tail) = (end_v_index_head + v_index_bit_num, end_v_index_head + 1);
    let (edge_dir_head, edge_dir_tail) = (start_v_index_head + edge_dir_bit_num, start_v_index_head + 1);
    let (end_v_label_head, end_v_label_tail) = (edge_dir_head + v_label_bit_num, edge_dir_head + 1);
    let (start_v_label_head, start_v_label_tail) = (end_v_label_head + v_label_bit_num, end_v_label_head + 1);
    let (edge_label_head, edge_label_tail) = (start_v_label_head + edge_label_bit_num, start_v_label_head + 1);
		// Compute each ASCII char
		for i in (0..ascii_char_num_per_encode_unit).rev() {
      let char_value: u8 = encoder.get_encode_numerical_value(edge_label, edge_label_head, edge_label_tail, storage_unit_valid_bit_num, i)
                         + encoder.get_encode_numerical_value(start_v_label, start_v_label_head, start_v_label_tail, storage_unit_valid_bit_num, i)
                         + encoder.get_encode_numerical_value(end_v_label, end_v_label_head, end_v_label_tail, storage_unit_valid_bit_num, i)
                         + encoder.get_encode_numerical_value(edge_dir, edge_dir_head, edge_dir_tail, storage_unit_valid_bit_num, i)
                         + encoder.get_encode_numerical_value(start_v_index, start_v_index_head, start_v_index_tail, storage_unit_valid_bit_num, i)
                         + encoder.get_encode_numerical_value(end_v_index, end_v_index_head, end_v_index_tail, storage_unit_valid_bit_num, i);
      encode_str.push(char_value.to_ascii_char().unwrap());
		}

		encode_str
  }
}

impl Encode<Vec<u8>> for EncodeUnit {
  fn encode_to(&self, encoder: &Encoder) -> Vec<u8> {
    let mut encode_vec: Vec<u8> = Vec::new();
    let (
      edge_label,
      start_v_label,
      end_v_label,
      edge_dir,
      start_v_index,
      end_v_index,
    ) = self.get_all_member_variable();
    let (
			edge_label_bit_num,
			v_label_bit_num,
			edge_dir_bit_num,
			v_index_bit_num,
		) = encoder.get_all_member_variable();
    let storage_unit_valid_bit_num = 8;
    let ascii_char_num_per_encode_unit = encoder.get_storage_unit_num_per_encode_unit(storage_unit_valid_bit_num);
    // Compute Value head/tail for each field
    let (end_v_index_head, end_v_index_tail) = (v_index_bit_num - 1, 0);
    let (start_v_index_head, start_v_index_tail) = (end_v_index_head + v_index_bit_num, end_v_index_head + 1);
    let (edge_dir_head, edge_dir_tail) = (start_v_index_head + edge_dir_bit_num, start_v_index_head + 1);
    let (end_v_label_head, end_v_label_tail) = (edge_dir_head + v_label_bit_num, edge_dir_head + 1);
    let (start_v_label_head, start_v_label_tail) = (end_v_label_head + v_label_bit_num, end_v_label_head + 1);
    let (edge_label_head, edge_label_tail) = (start_v_label_head + edge_label_bit_num, start_v_label_head + 1);
		// Compute each ASCII char
		for i in (0..ascii_char_num_per_encode_unit).rev() {
      let char_value: u8 = encoder.get_encode_numerical_value(edge_label, edge_label_head, edge_label_tail, storage_unit_valid_bit_num, i)
                         + encoder.get_encode_numerical_value(start_v_label, start_v_label_head, start_v_label_tail, storage_unit_valid_bit_num, i)
                         + encoder.get_encode_numerical_value(end_v_label, end_v_label_head, end_v_label_tail, storage_unit_valid_bit_num, i)
                         + encoder.get_encode_numerical_value(edge_dir, edge_dir_head, edge_dir_tail, storage_unit_valid_bit_num, i)
                         + encoder.get_encode_numerical_value(start_v_index, start_v_index_head, start_v_index_tail, storage_unit_valid_bit_num, i)
                         + encoder.get_encode_numerical_value(end_v_index, end_v_index_head, end_v_index_tail, storage_unit_valid_bit_num, i);
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
      let encode_unit: EncodeUnit = self.to_encode_unit_by_edge_id(*edge_id);
			let encode_string: AsciiString = <EncodeUnit as Encode<AsciiString>>::encode_to(&encode_unit, encoder);
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
      let encode_unit: EncodeUnit = self.to_encode_unit_by_edge_id(*edge_id);
			let encode_vec: Vec<u8> = <EncodeUnit as Encode<Vec<u8>>>::encode_to(&encode_unit, encoder);
			set.insert(encode_vec);
		}

    let mut encode_vec: Vec<u8> = Vec::new();
		let mut set_iter = set.iter();
		loop {
			match set_iter.next() {
				Some(vec) => encode_vec.extend(vec),
				None => break
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

  /// ### Generate AsciiString from Vector
  fn generate_asciistring_from_vec(vec: &Vec<u8>) -> AsciiString {
    let mut output = AsciiString::new();
    for value in vec {
      output.push(value.to_ascii_char().unwrap());
    }

    output
  }

  #[test]
	fn encode_unit_to_ascii_string() {
		let pattern = build_pattern_testcase_1();
		let encoder = Encoder::initialize(2, 2, 2, 2);
		let encode_unit_1 = pattern.to_encode_unit_by_edge_id(0);
		let encode_string_1 = <EncodeUnit as Encode<AsciiString>>::encode_to(&encode_unit_1, &encoder);
		let expected_encode_string_1: AsciiString = generate_asciistring_from_vec(&vec![11, 0]);
		assert_eq!(encode_string_1.len(), 2);
		assert_eq!(encode_string_1, expected_encode_string_1);
		let encode_unit_2 = pattern.to_encode_unit_by_edge_id(1);
		let encode_string_2 = <EncodeUnit as Encode<AsciiString>>::encode_to(&encode_unit_2, &encoder);
		let expected_encode_string_2: AsciiString = generate_asciistring_from_vec(&vec![19, 64]);
		assert_eq!(encode_string_2.len(), 2);
		assert_eq!(encode_string_2, expected_encode_string_2);
	}

  #[test]
  fn encode_unit_to_vec_u8() {
    let pattern = build_pattern_testcase_1();
		let encoder = Encoder::initialize(2, 2, 2, 2);
		let encode_unit_1 = pattern.to_encode_unit_by_edge_id(0);
		let encode_vec_1 = <EncodeUnit as Encode<Vec<u8>>>::encode_to(&encode_unit_1, &encoder);
		let expected_encode_vec_1: Vec<u8> = vec![5, 128];
		assert_eq!(encode_vec_1.len(), 2);
		assert_eq!(encode_vec_1, expected_encode_vec_1);
		let encode_unit_2 = pattern.to_encode_unit_by_edge_id(1);
		let encode_vec_2 = <EncodeUnit as Encode<Vec<u8>>>::encode_to(&encode_unit_2, &encoder);
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