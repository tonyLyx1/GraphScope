use std::cmp::{Ordering, min, max};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use crate::pattern::*;

/// 单个点的信息
/// 
/// Remark: 由于完全对称的点拥有相同的index，需使用index字段加以区分
#[derive(Debug, Clone)]
pub struct PatternVertex {
    pub id: u64,
    pub label: u64,
    pub index: u64,
    pub connect_edges: BTreeMap<u64, (u64, Direction)>,
    pub connect_vertices: BTreeMap<u64, Vec<(u64, Direction)>>,
    pub out_degree: u64,
    pub in_degree: u64,
}

impl PatternVertex {
    /// ### Get Vertex Index
    pub fn get_vertex_id(&self) -> u64 {
        self.id
    }

    /// ### Get Vertex Label
    pub fn get_vertex_label(&self) -> u64 {
        self.label
    }

    /// ### Get Vertex Index Reference
    pub fn get_vertex_index(&self) -> u64 {
        self.index
    }

    /// ### Set Vertex Index
    pub fn set_vertex_index(&mut self, vertex_index: u64) {
        self.index = vertex_index;
    }
}

impl PatternVertex {
  /// ### Get the Order of two PatternVertices
    /// Order by Vertex Label, In/Out Degree and the label id as well as ending vertex id of the connected edges
    /// 
    /// Return equal if still cannot distinguish
    pub fn cmp(&self, v2: &PatternVertex) -> Ordering {
      let v1 = self;
      // Compare Vertex Label
      match v1.label.cmp(&v2.label) {
          Ordering::Less => return Ordering::Less,
          Ordering::Greater => return Ordering::Greater,
          _ => (),
      }
      // Compare Vertex Out Degree
      match v1.out_degree.cmp(&v2.out_degree) {
          Ordering::Less => return Ordering::Less,
          Ordering::Greater => return Ordering::Greater,
          _ => (),
      }
      // Compare Vertex In Degree
      match v1.in_degree.cmp(&v2.in_degree) {
          Ordering::Less => return Ordering::Less,
          Ordering::Greater => return Ordering::Greater,
          _ => (),
      }

      // The number of connected edges must be equal as In/Out Degrees are equal
      let v1_connected_edges_iter = v1.connect_edges.iter();
      let mut v2_connected_edges_iter = v2.connect_edges.iter();
      for (v1_connected_edge_id, (v1_connected_edge_end_v_id, v1_connected_edge_dir)) in v1_connected_edges_iter {
          match v2_connected_edges_iter.next() {
              Some(edge_info) => {
                  let (v2_connected_edge_id, (v2_connected_edge_end_v_id, v2_connected_edge_dir)) = edge_info;
                  match v1_connected_edge_id.cmp(v2_connected_edge_id) {
                      Ordering::Less => return Ordering::Less,
                      Ordering::Greater => return Ordering::Greater,
                      _ => (),
                  }
                  match v1_connected_edge_end_v_id.cmp(v2_connected_edge_end_v_id) {
                      Ordering::Less => return Ordering::Less,
                      Ordering::Greater => return Ordering::Greater,
                      _ => (),
                  }
                  match v1_connected_edge_dir.to_u8().cmp(&v2_connected_edge_dir.to_u8()) {
                      Ordering::Less => return Ordering::Less,
                      Ordering::Greater => return Ordering::Greater,
                      _ => (),
                  }
              },
              None => break
          }
      }

      Ordering::Equal
  }
}
