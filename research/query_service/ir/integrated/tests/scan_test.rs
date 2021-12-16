//
//! Copyright 2021 Alibaba Group Holding Limited.
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
//!
//!

mod common;

#[cfg(test)]
mod test {
    use graph_proxy::{create_demo_graph, SimplePartition};
    use graph_store::common::DefaultId;
    use graph_store::ldbc::LDBCVertexParser;
    use ir_common::generated::algebra as pb;
    use ir_common::generated::common as common_pb;
    use runtime::graph::element::GraphElement;
    use runtime::process::operator::source::SourceOperator;
    use runtime::process::record::Record;
    use std::sync::Arc;

    // g.V()
    fn scan_gen(scan_opr_pb: pb::Scan) -> Box<dyn Iterator<Item = Record> + Send> {
        create_demo_graph();
        let mut source_opr_pb =
            pb::logical_plan::Operator { opr: Some(pb::logical_plan::operator::Opr::Scan(scan_opr_pb)) };
        let source =
            SourceOperator::new(&mut source_opr_pb, 1, 1, Arc::new(SimplePartition { num_servers: 1 }))
                .unwrap();
        source.gen_source(0).unwrap()
    }

    fn idx_scan_gen(idx_scan_opr_pb: pb::IndexedScan) -> Box<dyn Iterator<Item = Record> + Send> {
        create_demo_graph();
        let mut source_opr_pb = pb::logical_plan::Operator {
            opr: Some(pb::logical_plan::operator::Opr::IndexedScan(idx_scan_opr_pb)),
        };
        let source =
            SourceOperator::new(&mut source_opr_pb, 1, 1, Arc::new(SimplePartition { num_servers: 1 }))
                .unwrap();
        source.gen_source(0).unwrap()
    }

    // g.V()
    #[test]
    fn scan_test() {
        let source_iter = scan_gen(pb::Scan { scan_opt: 0, alias: None, params: None });
        let mut result_ids = vec![];
        let v1: DefaultId = LDBCVertexParser::to_global_id(1, 0);
        let v2: DefaultId = LDBCVertexParser::to_global_id(2, 0);
        let v3: DefaultId = LDBCVertexParser::to_global_id(3, 1);
        let v4: DefaultId = LDBCVertexParser::to_global_id(4, 0);
        let v5: DefaultId = LDBCVertexParser::to_global_id(5, 1);
        let v6: DefaultId = LDBCVertexParser::to_global_id(6, 0);
        let mut expected_ids = vec![v1, v2, v3, v4, v5, v6];
        for record in source_iter {
            if let Some(element) = record.get(None).unwrap().as_graph_element() {
                result_ids.push(element.id() as usize)
            }
        }
        result_ids.sort();
        expected_ids.sort();
        assert_eq!(result_ids, expected_ids)
    }

    // g.V().hasLabel('person')
    #[test]
    fn scan_with_label_test() {
        let source_iter = scan_gen(pb::Scan {
            scan_opt: 0,
            alias: None,
            params: Some(pb::QueryParams {
                table_names: vec!["person".into()],
                columns: vec![],
                limit: None,
                predicate: None,
                requirements: vec![],
            }),
        });
        let mut result_ids = vec![];
        let v1: DefaultId = LDBCVertexParser::to_global_id(1, 0);
        let v2: DefaultId = LDBCVertexParser::to_global_id(2, 0);
        let v4: DefaultId = LDBCVertexParser::to_global_id(4, 0);
        let v6: DefaultId = LDBCVertexParser::to_global_id(6, 0);
        let mut expected_ids = vec![v1, v2, v4, v6];
        for record in source_iter {
            if let Some(element) = record.get(None).unwrap().as_graph_element() {
                result_ids.push(element.id() as usize)
            }
        }
        result_ids.sort();
        expected_ids.sort();
        assert_eq!(result_ids, expected_ids)
    }

    // g.V().hasLabel('person', 'software')
    #[test]
    fn scan_with_many_labels_test() {
        let source_iter = scan_gen(pb::Scan {
            scan_opt: 0,
            alias: None,
            params: Some(pb::QueryParams {
                table_names: vec!["person".into(), "software".into()],
                columns: vec![],
                limit: None,
                predicate: None,
                requirements: vec![],
            }),
        });
        let mut result_ids = vec![];
        let v1: DefaultId = LDBCVertexParser::to_global_id(1, 0);
        let v2: DefaultId = LDBCVertexParser::to_global_id(2, 0);
        let v3: DefaultId = LDBCVertexParser::to_global_id(3, 1);
        let v4: DefaultId = LDBCVertexParser::to_global_id(4, 0);
        let v5: DefaultId = LDBCVertexParser::to_global_id(5, 1);
        let v6: DefaultId = LDBCVertexParser::to_global_id(6, 0);
        let mut expected_ids = vec![v1, v2, v3, v4, v5, v6];
        for record in source_iter {
            if let Some(element) = record.get(None).unwrap().as_graph_element() {
                result_ids.push(element.id() as usize)
            }
        }
        result_ids.sort();
        expected_ids.sort();
        assert_eq!(result_ids, expected_ids)
    }

    // g.V(1)
    #[test]
    fn idx_scan_test() {
        let source_iter = idx_scan_gen(pb::IndexedScan {
            scan: Some(pb::Scan { scan_opt: 0, alias: None, params: None }),
            or_kv_equiv_pairs: vec![pb::indexed_scan::KvEquivPairs {
                pairs: vec![pb::indexed_scan::KvEquivPair {
                    key: Some(common_pb::Property {
                        item: Some(common_pb::property::Item::Id(common_pb::IdKey {})),
                    }),
                    value: Some(common_pb::Const {
                        value: Some(common_pb::Value { item: Some(common_pb::value::Item::I64(1)) }),
                    }),
                }],
            }],
        });

        let mut result_ids = vec![];
        let v1: DefaultId = LDBCVertexParser::to_global_id(1, 0);
        let mut expected_ids = vec![v1];
        for record in source_iter {
            if let Some(element) = record.get(None).unwrap().as_graph_element() {
                result_ids.push(element.id() as usize)
            }
        }
        result_ids.sort();
        expected_ids.sort();
        assert_eq!(result_ids, expected_ids)
    }

    // g.V([1, 2])
    #[test]
    fn idx_scan_may_ids_test() {
        let source_iter = idx_scan_gen(pb::IndexedScan {
            scan: Some(pb::Scan { scan_opt: 0, alias: None, params: None }),
            or_kv_equiv_pairs: vec![
                pb::indexed_scan::KvEquivPairs {
                    pairs: vec![pb::indexed_scan::KvEquivPair {
                        key: Some(common_pb::Property {
                            item: Some(common_pb::property::Item::Id(common_pb::IdKey {})),
                        }),
                        value: Some(common_pb::Const {
                            value: Some(common_pb::Value { item: Some(common_pb::value::Item::I64(1)) }),
                        }),
                    }],
                },
                pb::indexed_scan::KvEquivPairs {
                    pairs: vec![pb::indexed_scan::KvEquivPair {
                        key: Some(common_pb::Property {
                            item: Some(common_pb::property::Item::Id(common_pb::IdKey {})),
                        }),
                        value: Some(common_pb::Const {
                            value: Some(common_pb::Value { item: Some(common_pb::value::Item::I64(2)) }),
                        }),
                    }],
                },
            ],
        });

        let mut result_ids = vec![];
        let v1: DefaultId = LDBCVertexParser::to_global_id(1, 0);
        let v2: DefaultId = LDBCVertexParser::to_global_id(2, 0);
        let mut expected_ids = vec![v1, v2];
        for record in source_iter {
            if let Some(element) = record.get(None).unwrap().as_graph_element() {
                result_ids.push(element.id() as usize)
            }
        }
        result_ids.sort();
        expected_ids.sort();
        assert_eq!(result_ids, expected_ids)
    }
}
