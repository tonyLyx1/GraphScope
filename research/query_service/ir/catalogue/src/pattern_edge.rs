use std::cmp::{Ordering, min, max};

/// 单条边的信息
#[derive(Debug, Clone, Copy)]
pub struct PatternEdge {
  pub id: u64,
  pub label: u64,
  pub start_v_id: u64,
  pub end_v_id: u64,
  pub start_v_label: u64,
  pub end_v_label: u64,
}

impl PatternEdge {
    /// ### Create a New PatternEdge
    pub fn create(
        id: u64,
        label: u64,
        start_v_id: u64,
        end_v_id: u64,
        start_v_label: u64,
        end_v_label: u64
    ) -> PatternEdge {
        PatternEdge {
            id,
            label,
            start_v_id,
            end_v_id,
            start_v_label,
            end_v_label,
        }
    }
}

/// Getters
impl PatternEdge {
    /// ### Get Edge Label
    pub fn get_edge_label(&self) -> u64 {
        self.label
    }

    /// ### Get Edge Index
    pub fn get_edge_id(&self) -> u64 {
        self.id
    }

    /// ### Get the Indices of Both Start and End Vertices of the Edge
    pub fn get_edge_vertices_id(&self) -> (u64, u64) {
        (self.start_v_id, self.end_v_id)
    }

    /// ### Get the Labels of Both Start and End Vertices of the Edge
    pub fn get_edge_vertices_label(&self) -> (u64, u64) {
        (self.start_v_label, self.end_v_label)
    }
}

impl PatternEdge {
  /// ### Get the Order of two PatternEdges of a Pattern
    /// Order by Edge Label, Vertex Labels and Vertex Indices
    /// 
    /// Return equal if still cannot distinguish
    pub fn cmp(&self, e2: &PatternEdge) -> Ordering {
      // Get edges from BTreeMap
      let e1 = self;
      // Compare the edge label
      match e1.label.cmp(&e2.label) {
          Ordering::Less => return Ordering::Less,
          Ordering::Greater => return Ordering::Greater,
          _ => (),
      }
      // Compare the label of starting vertex
      match e1.start_v_label.cmp(&e2.start_v_label) {
          Ordering::Less => return Ordering::Less,
          Ordering::Greater => return Ordering::Greater,
          _ => (),
      }
      // Compare the label of ending vertex
      match e1.end_v_label.cmp(&e2.end_v_label) {
          Ordering::Less => return Ordering::Less,
          Ordering::Greater => return Ordering::Greater,
          _ => (),
      }
    
      // Return as equal if still cannot distinguish
      Ordering::Equal
  }
}
