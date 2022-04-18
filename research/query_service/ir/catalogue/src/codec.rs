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
