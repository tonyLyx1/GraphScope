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
use crate::process::traversal::step::MapFuncGen;
use crate::process::traversal::Traverser;
use crate::structure::{AsTag, QueryParams, Vertex, VertexOrEdge};
use crate::{str_err, DynResult};
use bit_set::BitSet;
use pegasus::api::function::{FnResult, MapFunction};

struct IdentityFunc {
    params: QueryParams<Vertex>,
    tag: Option<AsTag>,
}

// runtime identity step is used in the following scenarios:
// 1. g.V().out().identity(props), where identity gives the props needs to be saved (will shuffle to out vertex firstly)
// TODO: 2. g.V().outE().as("a"), where identity may gives the tag "a". We do this only because compiler may give plan like this.
// 3. g.V().union(identity(), both()), which is the real gremlin identity step
// 4. g.V().count().as("a"), where identity gives the tag "a", since count() is an op in engine
// 5. Give hint of tags to remove. Since we may not able to remove tags in some OpKind, e.g., Filter, Sort, Group, etc, we add identity (map step) to remove tags.
impl MapFunction<Traverser, Traverser> for IdentityFunc {
    fn exec(&self, mut input: Traverser) -> FnResult<Traverser> {
        if let Some(tag) = self.tag {
            input.set_as_tag(tag);
        }
        if let Some(elem) = input.get_graph_element_mut() {
            if self.params.props.is_some() {
                // the case of preserving properties on demand for vertex
                if let VertexOrEdge::V(ori_v) = elem.get_mut() {
                    let id = ori_v.id;
                    let graph = crate::get_graph().ok_or(str_err("Graph is None"))?;
                    let mut r = graph.get_vertex(&[id], &self.params)?;
                    if let Some(v) = r.next() {
                        *ori_v = v;
                    } else {
                        return Err(str_err(&format!("vertex with id {} not found", id)));
                    }
                }
            }
        }
        Ok(input)
    }
}

pub struct IdentityStep {
    pub step: pb::IdentityStep,
    pub tag: Option<AsTag>,
}

impl MapFuncGen for IdentityStep {
    fn gen_map(self) -> DynResult<Box<dyn MapFunction<Traverser, Traverser>>> {
        let step = self.step;
        let is_all = step.is_all;
        let properties = step.properties;
        let mut params = QueryParams::new();
        if is_all || !properties.is_empty() {
            // the case when we need all properties or given properties
            params.props = Some(properties);
        } else {
            // the case when we do not need any property
            params.props = None;
        };
        Ok(Box::new(IdentityFunc { params, tag: self.tag }))
    }
}
