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

use crate::generated::gremlin as pb;
use crate::process::traversal::step::map::get_path::{CountLocalStep, PathStep};
use crate::process::traversal::step::map::identity::IdentityStep;
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

impl MapFuncGen for pb::GremlinStep {
    fn gen_map(self) -> DynResult<Box<dyn MapFunction<Traverser, Traverser>>> {
        // TODO(bingqing): confirm if tag is ok (or should be tags?)
        let tag = self.get_tag();
        let remove_tags = self.get_remove_tags();
        if let Some(step) = self.step {
            match step {
                pb::gremlin_step::Step::PathStep(_path_step) => Ok(Box::new(PathStep { tag })),
                pb::gremlin_step::Step::SelectStep(select_step) => select_step.gen_map(),
                pb::gremlin_step::Step::IdentityStep(identity_step) => {
                    let identity_step = IdentityStep { step: identity_step, tag };
                    identity_step.gen_map()
                }
                pb::gremlin_step::Step::SelectOneWithoutBy(_) => {
                    todo!()
                }
                // TODO(bingqing): Rename PathLocalCountStep as CountLocalStep
                pb::gremlin_step::Step::PathLocalCountStep(_s) => Ok(Box::new(CountLocalStep { tag })),
                pb::gremlin_step::Step::EdgeVertexStep(edge_vertex_step) => {
                    // let edge_vertex_step = EdgeVertexStep { step: edge_vertex_step, tags, remove_tags };
                    // edge_vertex_step.gen_map()
                    todo!()
                }
                pb::gremlin_step::Step::TransformTraverserStep(s) => {
                    todo!()
                    // let requirements_pb = unsafe { std::mem::transmute(s.traverser_requirements) };
                    // let requirements = Requirement::from_pb(requirements_pb)?;
                    // Ok(Box::new(TransformTraverserStep { requirement: requirements, remove_tags }))
                }
                _ => Err(str_err("pb GremlinStep is not a Map Step")),
            }
        } else {
            Err(str_err("pb GremlinStep does not have a step"))
        }
    }
}
