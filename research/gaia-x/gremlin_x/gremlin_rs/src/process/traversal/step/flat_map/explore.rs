//
//! Copyright 2020 Alibaba Group Holding Limited.
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

use super::FlatMapFuncGen;
use crate::generated::gremlin as pb;
use crate::process::traversal::Traverser;
use crate::structure::codec::pb_chain_to_filter;
use crate::structure::{AsTag, Direction, Element, GraphElement, Label, QueryParams, Statement, ID};
use crate::{str_err, DynIter, DynResult, FromPb};
use bit_set::BitSet;
use graph_store::prelude::LabelId;
use pegasus::api::function::FlatMapFunction;
use std::sync::Arc;

pub struct FlatMapStatement<E: Into<GraphElement>> {
    tag: Option<AsTag>,
    stmt: Box<dyn Statement<ID, E>>,
}

impl<E: Into<GraphElement> + 'static> FlatMapFunction<Traverser, Traverser> for FlatMapStatement<E> {
    type Target = DynIter<Traverser>;

    fn exec(&self, input: Traverser) -> DynResult<DynIter<Traverser>> {
        if let Some(e) = input.get_graph_element() {
            let id = e.id();
            let tag = self.tag;
            let iter = self.stmt.exec(id)?.map(move |e| {
                let e = e.into();
                let mut t = input.split(e);
                if let Some(tag) = tag {
                    t.set_as_tag(tag);
                }
                t
            });
            Ok(Box::new(iter))
        } else {
            Err(str_err("invalid input for vertex/edge step"))
        }
    }
}

// /// out(), in(), both(), outE(), inE(), bothE()
// pub struct VertexStep {
//     pub step: pb::VertexStep,
//     pub tag: Option<AsTag>,
// }
//
// impl FlatMapFuncGen for VertexStep {
//     fn gen_flat_map(
//         self,
//     ) -> DynResult<Box<dyn FlatMapFunction<Traverser, Traverser, Target = DynIter<Traverser>>>> {
//         let mut step = self.step;
//         let direction_pb = unsafe { std::mem::transmute(step.direction) };
//         let direction = Direction::from_pb(direction_pb)?;
//         let labels = step.edge_labels.iter().map(|id| Label::Id(*id as LabelId)).collect();
//         let graph = crate::get_graph().ok_or(str_err("Graph is None"))?;
//         if step.return_type == 0 {
//             let mut params = QueryParams::new();
//             params.labels = labels;
//             if let Some(test) = step.predicates.take() {
//                 if let Some(filter) = pb_chain_to_filter(&test)? {
//                     params.set_filter(filter);
//                 }
//             }
//             let stmt = graph.prepare_explore_vertex(direction, &params)?;
//             Ok(Box::new(FlatMapStatement { tag: self.tag, stmt }))
//         } else if step.return_type == 1 {
//             let mut params = QueryParams::new();
//             params.labels = labels;
//             if let Some(test) = step.predicates.take() {
//                 if let Some(filter) = pb_chain_to_filter(&test)? {
//                     params.set_filter(filter);
//                 }
//             }
//             let stmt = graph.prepare_explore_edge(direction, &params)?;
//             Ok(Box::new(FlatMapStatement { tag: self.tag, stmt }))
//         } else {
//             Err(str_err("Wrong return type in VertexStep"))
//         }
//     }
// }
