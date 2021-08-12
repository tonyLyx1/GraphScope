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

use crate::process::traversal::step::Step;
use crate::process::traversal::{Requirement, Traverser};
use crate::structure::AsTag;
use crate::FromPb;
use crate::{str_err, DynResult};
use pegasus::api::function::MapFunction;

#[enum_dispatch]
pub trait MapFuncGen {
    fn gen_map(self) -> DynResult<Box<dyn MapFunction<Traverser, Traverser>>>;
}

mod edge_v;
mod get_path;
mod identity;
mod multi_select;
mod select_one;
mod transform_traverser;

// impl MapFuncGen for pb::GremlinStep {
//     fn gen_map(self) -> DynResult<Box<dyn MapFunction<Traverser, Traverser>>> {
//         let tag = self.get_tag();
//         let remove_tags = self.get_remove_tags();
//         if let Some(step) = self.step {
//             match step {
//                 pb::gremlin_step::Step::PathStep(path_step) => Ok(Box::new(path_step)),
//                 pb::gremlin_step::Step::SelectStep(select_step) => select_step.gen_map(),
//                 pb::gremlin_step::Step::IdentityStep(identity_step) => {
//                     let identity_step = IdentityStep { step: identity_step, tags, remove_tags };
//                     identity_step.gen_map()
//                 }
//                 pb::gremlin_step::Step::SelectOneWithoutBy(_) => {
//                     todo!()
//                 }
//                 pb::gremlin_step::Step::PathLocalCountStep(_s) => Ok(Box::new(PathLocalCountStep { tag })),
//                 pb::gremlin_step::Step::EdgeVertexStep(edge_vertex_step) => {
//                     let edge_vertex_step = EdgeVertexStep { step: edge_vertex_step, tags, remove_tags };
//                     edge_vertex_step.gen_map()
//                 }
//                 pb::gremlin_step::Step::TransformTraverserStep(s) => {
//                     let requirements_pb = unsafe { std::mem::transmute(s.traverser_requirements) };
//                     let requirements = Requirement::from_pb(requirements_pb)?;
//                     Ok(Box::new(TransformTraverserStep { requirement: requirements, remove_tags }))
//                 }
//                 _ => Err(str_to_dyn_error("pb GremlinStep is not a Map Step")),
//             }
//         } else {
//             Err(str_to_dyn_error("pb GremlinStep does not have a step"))
//         }
//     }
// }
