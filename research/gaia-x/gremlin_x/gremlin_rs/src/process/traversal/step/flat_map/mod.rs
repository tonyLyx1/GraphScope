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
use crate::process::traversal::Traverser;
use crate::{str_err, DynResult};
use pegasus::api::function::{DynIter, FlatMapFunction};

mod explore;
mod values;

#[enum_dispatch]
pub trait FlatMapFuncGen {
    fn gen_flat_map(
        self,
    ) -> DynResult<Box<dyn FlatMapFunction<Traverser, Traverser, Target = DynIter<Traverser>>>>;
}

// impl FlatMapFuncGen for pb::GremlinStep {
//     fn gen_flat_map(
//         self,
//     ) -> DynResult<Box<dyn FlatMapFunction<Traverser, Traverser, Target = DynIter<Traverser>>>> {
//         let tag = self.get_tag();
//
//         if let Some(step) = self.step {
//             match step {
//                 pb::gremlin_step::Step::VertexStep(vertex_step) => {
//                     let vertex_step = VertexStep { step: vertex_step, tag };
//                     vertex_step.gen_flat_map()
//                 }
//                 pb::gremlin_step::Step::PropertiesStep(properties_step) => {
//                     Ok(Box::new(PropertiesStep { props: properties_step.properties.clone(), tag }))
//                 }
//                 pb::gremlin_step::Step::UnfoldStep(unfold_step) => Ok(Box::new(unfold_step)),
//                 _ => Err(str_err("pb GremlinStep is not a FlatMap Step")),
//             }
//         } else {
//             Err(str_err("pb GremlinStep does not have a step"))
//         }
//     }
// }
