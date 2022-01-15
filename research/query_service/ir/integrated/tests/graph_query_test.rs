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
    use ir_common::expr_parse::str_to_expr_pb;
    use ir_common::generated::algebra as pb;
    use ir_common::generated::common as common_pb;
    use ir_common::NameOrId;
    use ir_core::plan::logical::LogicalPlan;
    use ir_core::plan::meta::set_schema_simple;
    use ir_core::plan::physical::AsPhysical;
    use pegasus_client::builder::*;
    use pegasus_server::JobRequest;
    use runtime::graph::element::GraphElement;

    use crate::common::test::*;

    fn init_poc_request() -> JobRequest {
        let mut plan = LogicalPlan::default();
        plan.plan_meta.set_preprocess(true);

        set_schema_simple(
            vec![("person".to_string(), 0).into(), ("software".to_string(), 1).into()],
            vec![
                (
                    ("knows".to_string(), 0).into(),
                    ("person".to_string(), 0).into(),
                    ("person".to_string(), 0).into(),
                )
                    .into(),
                (
                    ("creates".to_string(), 1).into(),
                    ("person".to_string(), 0).into(),
                    ("software".to_string(), 1).into(),
                )
                    .into(),
            ],
            vec![],
        );

        // g.V().hasLabel("person").has("id", 1).both("creates").limit(10)
        let source_opr = pb::Scan {
            scan_opt: 0,
            alias: None,
            params: Some(pb::QueryParams {
                table_names: vec![common_pb::NameOrId::from("person".to_string())],
                columns: vec![],
                limit: None,
                predicate: None,
                requirements: vec![],
            }),
            idx_predicate: None,
        };
        let select_opr = pb::Select { predicate: Some(str_to_expr_pb("@.id == 1".to_string()).unwrap()) };
        let expand_opr = pb::EdgeExpand {
            base: Some(pb::ExpandBase {
                v_tag: None,
                direction: 2,
                params: Some(pb::QueryParams {
                    table_names: vec![common_pb::NameOrId::from("creates".to_string())],
                    columns: vec![],
                    limit: None,
                    predicate: None,
                    requirements: vec![],
                }),
            }),
            is_edge: false,
            alias: None,
        };
        let limit_opr = pb::Limit { range: Some(pb::Range { lower: 10, upper: 11 }) };
        let sink_opr = pb::Sink { tags: vec![], sink_current: true, id_name_mappings: vec![] };

        plan.append_operator_as_node(source_opr.into(), vec![])
            .unwrap();
        plan.append_operator_as_node(select_opr.into(), vec![0])
            .unwrap();
        plan.append_operator_as_node(expand_opr.into(), vec![1])
            .unwrap();
        plan.append_operator_as_node(limit_opr.into(), vec![2])
            .unwrap();
        plan.append_operator_as_node(sink_opr.into(), vec![3])
            .unwrap();

        let mut job_builder = JobBuilder::default();
        let mut plan_meta = plan.plan_meta.clone();
        plan.add_job_builder(&mut job_builder, &mut plan_meta)
            .unwrap();

        job_builder.build().unwrap()
    }

    #[test]
    fn test_poc_query() {
        initialize();
        let request = init_poc_request();
        let mut results = submit_query(request, 1);
        let mut result_collection = vec![];
        let expected_result_ids = vec![(1 << 56 | 3, NameOrId::Str("software".to_string()))];
        while let Some(result) = results.next() {
            match result {
                Ok(res) => {
                    let entry = parse_result(res).unwrap();
                    if let Some(vertex) = entry.get(None).unwrap().as_graph_element() {
                        result_collection.push((vertex.id(), vertex.label().unwrap().clone()));
                        println!("{:?}", vertex);
                    }
                }
                Err(e) => {
                    panic!("err result {:?}", e);
                }
            }
        }
        result_collection.sort();
        assert_eq!(result_collection, expected_result_ids)
    }
}
