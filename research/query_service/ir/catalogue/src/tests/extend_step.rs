#[cfg(test)]
mod unit_test {
    use crate::pattern::*;
    use crate::encoder::*;
    use crate::extend_step::*;

    fn build_extend_step_case1() -> ExtendStep {
        let extend_edge0 =
            ExtendEdge { start_v_label: 1, start_v_index: 0, edge_label: 1, dir: Direction::Out };
        let extend_edge1 = extend_edge0.clone();
        ExtendStep::from((1, vec![extend_edge0, extend_edge1]))
    }

    #[test]
    fn test_extend_step_case1_structure() {
        let extend_step1 = build_extend_step_case1();
        assert_eq!(extend_step1.target_v_label, 1);
        assert_eq!(extend_step1.extend_edges.len(), 1);
        assert_eq!(
            extend_step1
                .extend_edges
                .get(&(1, 0))
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            extend_step1.extend_edges.get(&(1, 0)).unwrap()[0],
            ExtendEdge { start_v_label: 1, start_v_index: 0, edge_label: 1, dir: Direction::Out }
        );
        assert_eq!(
            extend_step1.extend_edges.get(&(1, 0)).unwrap()[1],
            ExtendEdge { start_v_label: 1, start_v_index: 0, edge_label: 1, dir: Direction::Out }
        );
    }
}