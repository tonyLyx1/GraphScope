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
use crate::process::traversal::step::map::MapFuncGen;
use crate::process::traversal::Traverser;
use crate::structure::{AsTag, EndPointOpt, GraphElement, QueryParams, Vertex, VertexOrEdge};
use crate::{str_err, DynResult, FromPb};
use bit_set::BitSet;
use pegasus::api::function::{FnResult, MapFunction};

struct EdgeVertexFunc {
    tag: Option<AsTag>,
    params: QueryParams<Vertex>,
    get_src: bool,
}

impl MapFunction<Traverser, Traverser> for EdgeVertexFunc {
    fn exec(&self, mut input: Traverser) -> FnResult<Traverser> {
        if let Some(elem) = input.get_graph_element() {
            match elem.get() {
                VertexOrEdge::E(e) => {
                    let id = if self.get_src { e.src_id } else { e.dst_id };
                    let graph = crate::get_graph().ok_or(str_err("Graph is None"))?;
                    let mut r = graph.get_vertex(&[id], &self.params)?;
                    if let Some(v) = r.next() {
                        let e: GraphElement = v.into();
                        let mut t = input.split(e);
                        if let Some(tag) = self.tag {
                            t.set_as_tag(tag);
                        }
                        Ok(t)
                    } else {
                        Err(str_err(&format!("Vertex with id {} not found", id)))
                    }
                }
                _ => Err(str_err("Should not call `EdgeVertexStep` on a vertex")),
            }
        } else {
            Err(str_err("invalid input for `EdgeVertexStep`"))
        }
    }
}

// pub struct EdgeVertexStep {
//     pub step: pb::EdgeVertexStep,
//     pub tag: Option<AsTag>,
// }
//
// impl MapFuncGen for EdgeVertexStep {
//     fn gen_map(self) -> DynResult<Box<dyn MapFunction<Traverser, Traverser>>> {
//         let step = self.step;
//         let opt_pb = unsafe { std::mem::transmute(step.endpoint_opt) };
//         let opt = EndPointOpt::from_pb(opt_pb)?;
//         match opt {
//             EndPointOpt::Out => {
//                 Ok(Box::new(EdgeVertexFunc { tag: self.tag, params: QueryParams::new(), get_src: true })
//                     as Box<dyn MapFunction<Traverser, Traverser>>)
//             }
//             EndPointOpt::In => {
//                 Ok(Box::new(EdgeVertexFunc { tag: self.tag, params: QueryParams::new(), get_src: false })
//                     as Box<dyn MapFunction<Traverser, Traverser>>)
//             }
//             EndPointOpt::Other => Err(str_err("`otherV()` has not been supported")),
//         }
//     }
// }
