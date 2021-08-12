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
use crate::process::traversal::step::util::StepSymbol;
use crate::process::traversal::step::Step;
use crate::process::traversal::Requirement;
use crate::process::traversal::Traverser;
use crate::structure::codec::pb_chain_to_filter;
use crate::structure::{AsTag, GraphElement, Label, QueryParams, Vertex, ID};
use crate::FromPb;
use bit_set::BitSet;
use graph_store::common::LabelId;
use pegasus::BuildJobError;
use pegasus_common::downcast::*;

/// V(),
pub struct GraphVertexStep {
    pub symbol: StepSymbol,
    pub params: QueryParams<Vertex>,
    src: Option<Vec<Vec<ID>>>,
    tag: Option<AsTag>,
    requirement: Requirement,
    // workers per server, for gen_source
    workers: usize,
    server_index: u64,
}

impl_as_any!(GraphVertexStep);

impl GraphVertexStep {
    pub fn new(req: Requirement) -> Self {
        GraphVertexStep {
            symbol: StepSymbol::V,
            src: None,
            tag: None,
            requirement: req,
            params: QueryParams::new(),
            workers: 1,
            server_index: 0,
        }
    }

    pub fn set_num_workers(&mut self, workers: usize) {
        self.workers = workers;
    }

    pub fn set_server_index(&mut self, index: u64) {
        self.server_index = index;
    }

    pub fn set_src(&mut self, ids: Vec<ID>, server_num: usize) {
        let mut partition = Vec::with_capacity(server_num);
        for _ in 0..server_num {
            partition.push(vec![]);
        }
        for id in ids {
            let idx = (id % server_num as ID) as usize;
            partition[idx].push(id);
        }
        self.src = Some(partition);
    }

    pub fn set_requirement(&mut self, requirement: Requirement) {
        self.requirement = requirement;
    }
    pub fn set_tag(&mut self, tag: AsTag) {
        self.tag = Some(tag);
    }
}

impl Step for GraphVertexStep {
    fn get_symbol(&self) -> StepSymbol {
        self.symbol
    }
}

impl GraphVertexStep {
    pub fn gen_source(self, worker_index: Option<usize>) -> Box<dyn Iterator<Item = Traverser> + Send> {
        let gen_flag = if let Some(w_index) = worker_index { w_index % self.workers == 0 } else { true };
        let source = if let Some(ref seeds) = self.src {
            // work 0 in current server are going to get_vertex
            if gen_flag {
                if let Some(src) = seeds.get(self.server_index as usize) {
                    if !src.is_empty() {
                        let graph = crate::get_graph().unwrap();
                        graph.get_vertex(src, &self.params).unwrap_or(Box::new(std::iter::empty()))
                    } else {
                        Box::new(std::iter::empty())
                    }
                } else {
                    Box::new(std::iter::empty())
                }
            } else {
                Box::new(std::iter::empty())
            }
        } else {
            // work 0 in current server are going to scan_vertex
            if gen_flag {
                let graph = crate::get_graph().unwrap();
                graph.scan_vertex(&self.params).unwrap_or(Box::new(std::iter::empty()))
            } else {
                // return an emtpy iterator;
                Box::new(std::iter::empty())
            }
        };

        let tag = self.tag;
        if self.requirement.contains(Requirement::PATH) {
            let iter = source.map(move |v| {
                let e: GraphElement = v.into();
                let mut t = Traverser::with_path(e);
                if let Some(tag) = tag {
                    t.set_as_tag(tag);
                }
                t
            });
            Box::new(iter)
        } else if self.requirement.contains(Requirement::LABELED_PATH) {
            let iter = source.map(move |v| {
                let e: GraphElement = v.into();
                let mut t = Traverser::with_labeled_path(e);
                if let Some(tag) = tag {
                    t.set_as_tag(tag);
                }
                t
            });
            Box::new(iter)
        } else {
            let iter = source.map(move |v| {
                let e: GraphElement = v.into();
                let mut t = Traverser::new(e);
                if let Some(tag) = tag {
                    t.set_as_tag(tag);
                }
                t
            });
            Box::new(iter)
        }
    }
}

// pub fn graph_step_from(
//     gremlin_step: &mut pb::GremlinStep, num_servers: usize,
// ) -> Result<GraphVertexStep, BuildJobError> {
//     if let Some(option) = gremlin_step.step.take() {
//         match option {
//             pb::gremlin_step::Step::GraphStep(mut opt) => {
//                 let requirements_pb = unsafe { std::mem::transmute(opt.traverser_requirements) };
//                 let requirements = Requirement::from_pb(requirements_pb)?;
//                 let mut step = GraphVertexStep::new(requirements);
//                 if let Some(tag) = gremlin_step.get_tag() {
//                     step.set_tag(tag);
//                 }
//                 let mut ids = vec![];
//                 for id in opt.ids {
//                     ids.push(id as ID);
//                 }
//                 if !ids.is_empty() {
//                     step.set_src(ids, num_servers);
//                 }
//                 let labels = std::mem::replace(&mut opt.labels, vec![]);
//                 step.params.labels = labels.into_iter().map(|id| Label::Id(id as LabelId)).collect();
//                 if let Some(ref test) = opt.predicates {
//                     if let Some(filter) = pb_chain_to_filter(test)? {
//                         step.params.set_filter(filter);
//                     }
//                 }
//                 return Ok(step);
//             }
//             _ => (),
//         }
//     }
//     Err("Unsupported source step in pb_request")?
// }
