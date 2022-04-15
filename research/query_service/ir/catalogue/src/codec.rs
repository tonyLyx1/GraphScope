use crate::encoder::*;
use crate::extend_step::*;
use crate::pattern::*;
use ascii::{self, AsciiString};
use std::collections::{BTreeSet};

pub trait Encode {
  fn to_encode_unit(&self) -> EncodeUnit {
    EncodeUnit::new(0, 0, 0, Direction::Out, 0, 0)
  }

  fn to_encode_value(&self, encoder: &Encoder) -> AsciiString;
}

pub trait Decode {
  // To Be Completed
}

impl Encode for EncodeUnit {
  fn to_encode_value(&self, encoder: &Encoder) -> AsciiString {
    // 根据Encoder的比特位参数来决定生成string的格式
		let mut encode_str = AsciiString::new();
    let ascii_char_num_per_encode_unit = encoder.get_ascii_char_num_per_encode_unit();
		// Compute each ASCII char
		for i in (0..ascii_char_num_per_encode_unit).rev() {
      encode_str.push(self.to_ascii_char(encoder, i));
		}

		encode_str
  }
}

impl Encode for ExtendEdge {
  /// ### Convert ExtendEdge to EncodeUnit
  /// Since ExtendEdge does not contain information about the ending vertex,
  /// the end_v_label and end_v_index are set as 0 and wait for further modification
  fn to_encode_unit(&self) -> EncodeUnit {
      EncodeUnit::new(self.edge_label, self.start_v_label, 0, self.dir, self.start_v_index, 0)
  }

  fn to_encode_value(&self, encoder: &Encoder) -> AsciiString {
    let encode_unit = self.to_encode_unit();
    encode_unit.to_encode_value(encoder)
  }
}

impl Encode for ExtendStep {
  fn to_encode_value(&self, encoder: &Encoder) -> AsciiString {
    // Initialize AsciiString
    let mut encode_value = AsciiString::new();
    // Update the index of the pattern
    let target_v_index = self.get_target_v_index();
    // Set End Vertex Information on the Encode Units of ExtendEdges
    let extend_edges_iter = self.get_extend_edges().iter();
    for (_, edges) in extend_edges_iter {
        for edge in edges {
            let mut encode_unit: EncodeUnit = edge.to_encode_unit();
            encode_unit.set_end_v_label(self.get_target_v_label());
            encode_unit.set_end_v_index(target_v_index);
            encode_value = encode_value + &encode_unit.to_encode_value(encoder);
        }
    }

    encode_value
  }
}

impl Encode for Pattern {
  fn to_encode_value(&self, encoder: &Encoder) -> AsciiString {
    // Initialize an BTreeSet to Store the Encoding Units
		let mut set = BTreeSet::from([]);
		// Encode Each Edge in the Pattern as an Encoding Unit
		let edges = self.get_edges();
		for (edge_id, _) in edges.iter() {
      let encode_unit: EncodeUnit = self.to_encode_unit_by_edge_id(*edge_id);
			let encode_string: AsciiString = encode_unit.to_encode_value(encoder);
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
