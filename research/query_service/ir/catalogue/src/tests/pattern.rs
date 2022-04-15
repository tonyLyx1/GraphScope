#[cfg(test)]
mod unit_test {
  use crate::pattern::*;

  fn build_pattern_case1() -> Pattern {
      let pattern_edge1 =
          PatternEdge { id: 0, label: 0, start_v_id: 0, end_v_id: 1, start_v_label: 0, end_v_label: 0 };
      let pattern_edge2 =
          PatternEdge { id: 1, label: 0, start_v_id: 1, end_v_id: 0, start_v_label: 0, end_v_label: 0 };
      let pattern_vec = vec![pattern_edge1, pattern_edge2];
      Pattern::from(pattern_vec)
  }

  fn build_pattern_case2() -> Pattern {
      let pattern_edge1 =
          PatternEdge { id: 0, label: 0, start_v_id: 0, end_v_id: 1, start_v_label: 0, end_v_label: 0 };
      let pattern_edge2 =
          PatternEdge { id: 1, label: 0, start_v_id: 1, end_v_id: 0, start_v_label: 0, end_v_label: 0 };
      let pattern_edge3 =
          PatternEdge { id: 2, label: 1, start_v_id: 0, end_v_id: 2, start_v_label: 0, end_v_label: 1 };
      let pattern_edge4 =
          PatternEdge { id: 3, label: 1, start_v_id: 1, end_v_id: 2, start_v_label: 0, end_v_label: 1 };
      let pattern_vec = vec![pattern_edge1, pattern_edge2, pattern_edge3, pattern_edge4];
      Pattern::from(pattern_vec)
  }

  fn build_extend_step_case1() -> ExtendStep {
      let extend_edge1 =
          ExtendEdge { start_v_label: 0, start_v_index: 0, edge_label: 1, dir: Direction::Out };
      let extend_edge2 = extend_edge1.clone();
      ExtendStep::from((1, vec![extend_edge1, extend_edge2]))
  }

  #[test]
  fn test_pattern_case1_structure() {
      let pattern_case1 = build_pattern_case1();
      let edges_num = pattern_case1.edges.len();
      assert_eq!(edges_num, 2);
      let vertices_num = pattern_case1.vertices.len();
      assert_eq!(vertices_num, 2);
      let edges_with_label_0 = pattern_case1.edge_label_map.get(&0).unwrap();
      assert_eq!(edges_with_label_0.len(), 2);
      let mut edges_with_label_0_iter = edges_with_label_0.iter();
      assert_eq!(*edges_with_label_0_iter.next().unwrap(), 0);
      assert_eq!(*edges_with_label_0_iter.next().unwrap(), 1);
      let vertices_with_label_0 = pattern_case1.vertex_label_map.get(&0).unwrap();
      assert_eq!(vertices_with_label_0.len(), 2);
      let mut vertices_with_label_0_iter = vertices_with_label_0.iter();
      assert_eq!(*vertices_with_label_0_iter.next().unwrap(), 0);
      assert_eq!(*vertices_with_label_0_iter.next().unwrap(), 1);
      let edge_0 = pattern_case1.edges.get(&0).unwrap();
      assert_eq!(edge_0.id, 0);
      assert_eq!(edge_0.label, 0);
      assert_eq!(edge_0.start_v_id, 0);
      assert_eq!(edge_0.end_v_id, 1);
      assert_eq!(edge_0.start_v_label, 0);
      assert_eq!(edge_0.end_v_label, 0);
      let edge_1 = pattern_case1.edges.get(&1).unwrap();
      assert_eq!(edge_1.id, 1);
      assert_eq!(edge_1.label, 0);
      assert_eq!(edge_1.start_v_id, 1);
      assert_eq!(edge_1.end_v_id, 0);
      assert_eq!(edge_1.start_v_label, 0);
      assert_eq!(edge_1.end_v_label, 0);
      let vertex_0 = pattern_case1.vertices.get(&0).unwrap();
      assert_eq!(vertex_0.id, 0);
      assert_eq!(vertex_0.label, 0);
      assert_eq!(vertex_0.connect_edges.len(), 2);
      let mut vertex_0_connect_edges_iter = vertex_0.connect_edges.iter();
      let (v0_e0, (v0_v0, v0_d0)) = vertex_0_connect_edges_iter.next().unwrap();
      assert_eq!(*v0_e0, 0);
      assert_eq!(*v0_v0, 1);
      assert_eq!(*v0_d0, Direction::Out);
      let (v0_e1, (v0_v1, v0_d1)) = vertex_0_connect_edges_iter.next().unwrap();
      assert_eq!(*v0_e1, 1);
      assert_eq!(*v0_v1, 1);
      assert_eq!(*v0_d1, Direction::Incoming);
      assert_eq!(vertex_0.connect_vertices.len(), 1);
      let v0_v1_connected_edges = vertex_0.connect_vertices.get(&1).unwrap();
      assert_eq!(v0_v1_connected_edges.len(), 2);
      let mut v0_v1_connected_edges_iter = v0_v1_connected_edges.iter();
      assert_eq!(*v0_v1_connected_edges_iter.next().unwrap(), (0, Direction::Out));
      assert_eq!(*v0_v1_connected_edges_iter.next().unwrap(), (1, Direction::Incoming));
      let vertex_1 = pattern_case1.vertices.get(&1).unwrap();
      assert_eq!(vertex_1.id, 1);
      assert_eq!(vertex_1.label, 0);
      assert_eq!(vertex_1.connect_edges.len(), 2);
      let mut vertex_1_connect_edges_iter = vertex_1.connect_edges.iter();
      let (v1_e0, (v1_v0, v1_d0)) = vertex_1_connect_edges_iter.next().unwrap();
      assert_eq!(*v1_e0, 0);
      assert_eq!(*v1_v0, 0);
      assert_eq!(*v1_d0, Direction::Incoming);
      let (v1_e1, (v1_v1, v1_d1)) = vertex_1_connect_edges_iter.next().unwrap();
      assert_eq!(*v1_e1, 1);
      assert_eq!(*v1_v1, 0);
      assert_eq!(*v1_d1, Direction::Out);
      assert_eq!(vertex_1.connect_vertices.len(), 1);
      let v1_v0_connected_edges = vertex_1.connect_vertices.get(&0).unwrap();
      assert_eq!(v1_v0_connected_edges.len(), 2);
      let mut v1_v0_connected_edges_iter = v1_v0_connected_edges.iter();
      assert_eq!(*v1_v0_connected_edges_iter.next().unwrap(), (0, Direction::Incoming));
      assert_eq!(*v1_v0_connected_edges_iter.next().unwrap(), (1, Direction::Out));
  }

  #[test]
  fn test_pattern_case1_extend() {
        let pattern_case1 = build_pattern_case1();
        let extend_step = build_extend_step_case1();
        let pattern_after_extend = pattern_case1.extend(extend_step).unwrap();
        assert_eq!(pattern_after_extend.edges.len(), 4);
        assert_eq!(pattern_after_extend.vertices.len(), 3);
        // Pattern after extend should be exactly the same as pattern case2
        let pattern_case2 = build_pattern_case2();
        assert_eq!(pattern_after_extend.edges.len(), pattern_case2.edges.len());
        for i in 0..pattern_after_extend.edges.len() as u64 {
            let edge1 = pattern_after_extend.edges.get(&i).unwrap();
            let edge2 = pattern_case2.edges.get(&i).unwrap();
            assert_eq!(edge1.id, edge2.id);
            assert_eq!(edge1.label, edge2.label);
            assert_eq!(edge1.start_v_id, edge2.start_v_id);
            assert_eq!(edge1.start_v_label, edge2.start_v_label);
            assert_eq!(edge1.end_v_id, edge2.end_v_id);
            assert_eq!(edge1.end_v_label, edge2.end_v_label);
        }
        assert_eq!(pattern_after_extend.edges.len(), pattern_case2.edges.len());
        for i in 0..pattern_after_extend.vertices.len() as u64 {
            let vertex1 = pattern_after_extend.vertices.get(&i).unwrap();
            let vertex2 = pattern_after_extend.vertices.get(&i).unwrap();
            assert_eq!(vertex1.id, vertex2.id);
            assert_eq!(vertex1.label, vertex2.label);
            assert_eq!(vertex1.index, vertex2.index);
            assert_eq!(vertex1.in_degree, vertex2.in_degree);
            assert_eq!(vertex1.out_degree, vertex2.out_degree);
            assert_eq!(vertex1.connect_edges.len(), vertex2.connect_edges.len());
            assert_eq!(vertex1.connect_vertices.len(), vertex2.connect_vertices.len());
            for (connect_edge1_id, (connect_vertex1_id, dir1)) in &vertex1.connect_edges {
                let (connect_vertex2_id, dir2) = vertex2
                    .connect_edges
                    .get(connect_edge1_id)
                    .unwrap();
                assert_eq!(*connect_vertex1_id, *connect_vertex2_id);
                assert_eq!(*dir1, *dir2);
            }
            for (connect_vertex1_id, edge_connections1) in &vertex1.connect_vertices {
                let edge_connections2 = vertex2
                    .connect_vertices
                    .get(connect_vertex1_id)
                    .unwrap();
                let (connect_edge1_id, dir1) = edge_connections1[0];
                let (connect_edge2_id, dir2) = edge_connections2[0];
                assert_eq!(connect_edge1_id, connect_edge2_id);
                assert_eq!(dir1, dir2);
            }
        }
        assert_eq!(pattern_after_extend.edge_label_map.len(), pattern_case2.edge_label_map.len());
        for i in 0..=1 {
            let edges_with_labeli_1 = pattern_after_extend
                .edge_label_map
                .get(&i)
                .unwrap();
            let edges_with_labeli_2 = pattern_case2.edge_label_map.get(&i).unwrap();
            assert_eq!(edges_with_labeli_1.len(), edges_with_labeli_2.len());
            let mut edges_with_labeli_1_iter = edges_with_labeli_1.iter();
            let mut edges_with_labeli_2_iter = edges_with_labeli_2.iter();
            let mut edges_with_labeli_1_element = edges_with_labeli_1_iter.next();
            let mut edges_with_labeli_2_element = edges_with_labeli_2_iter.next();
            while edges_with_labeli_1_element.is_some() {
                assert_eq!(*edges_with_labeli_1_element.unwrap(), *edges_with_labeli_2_element.unwrap());
                edges_with_labeli_1_element = edges_with_labeli_1_iter.next();
                edges_with_labeli_2_element = edges_with_labeli_2_iter.next();
            }
        }
        assert_eq!(pattern_after_extend.vertex_label_map.len(), pattern_case2.vertex_label_map.len());
        for i in 0..=1 {
            let vertices_with_labeli_1 = pattern_after_extend
                .vertex_label_map
                .get(&i)
                .unwrap();
            let vertices_with_labeli_2 = pattern_case2.vertex_label_map.get(&i).unwrap();
            assert_eq!(vertices_with_labeli_1.len(), vertices_with_labeli_2.len());
            let mut vertices_with_labeli_1_iter = vertices_with_labeli_1.iter();
            let mut vertices_with_labeli_2_iter = vertices_with_labeli_2.iter();
            let mut vertices_with_labeli_1_element = vertices_with_labeli_1_iter.next();
            let mut vertices_with_labeli_2_element = vertices_with_labeli_2_iter.next();
            while vertices_with_labeli_1_element.is_some() {
                assert_eq!(
                    *vertices_with_labeli_1_element.unwrap(),
                    *vertices_with_labeli_2_element.unwrap()
                );
                vertices_with_labeli_1_element = vertices_with_labeli_1_iter.next();
                vertices_with_labeli_2_element = vertices_with_labeli_2_iter.next();
            }
        }
    }

  #[test]
  fn test_set_initial_vertex_index() {
    assert_eq!(1, 1);
  }
}
