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

use std::convert::TryInto;

use ir_common::generated::algebra as algebra_pb;
use ir_common::generated::algebra::get_v::VOpt;
use ir_common::NameOrId;
use pegasus::api::function::{FnResult, MapFunction};

use crate::error::{FnExecError, FnGenResult};
use crate::graph::element::Vertex;
use crate::graph::property::{DefaultDetails, DynDetails};
use crate::process::operator::map::MapFuncGen;
use crate::process::record::Record;

#[derive(Debug)]
struct GetVertexOperator {
    start_tag: Option<NameOrId>,
    opt: VOpt,
    alias: Option<NameOrId>,
}

impl MapFunction<Record, Record> for GetVertexOperator {
    fn exec(&self, mut input: Record) -> FnResult<Record> {
        let entry = input
            .get(self.start_tag.as_ref())
            .ok_or(FnExecError::get_tag_error("get tag failed in GetVertexOperator"))?;
        if let Some(e) = entry.as_graph_edge() {
            let (id, label) = match self.opt {
                VOpt::Start => (e.src_id, e.get_src_label()),
                VOpt::End => (e.dst_id, e.get_dst_label()),
                VOpt::Other => (e.get_other_id(), e.get_other_label()),
            };
            let vertex =
                Vertex::new(id, label.map(|l| l.clone()), DynDetails::new(DefaultDetails::default()));
            input.append(vertex, self.alias.clone());
            Ok(input)
        } else if let Some(_p) = entry.as_graph_path() {
            //TODO(bingqing): endV for path
            todo!()
        } else {
            Err(FnExecError::unexpected_data_error(
                "Can only apply `GetV` (`Auxilia` instead) on an edge or path entry",
            ))?
        }
    }
}

impl MapFuncGen for algebra_pb::GetV {
    fn gen_map(self) -> FnGenResult<Box<dyn MapFunction<Record, Record>>> {
        let start_tag = self
            .tag
            .map(|name_or_id| name_or_id.try_into())
            .transpose()?;
        let opt: VOpt = unsafe { ::std::mem::transmute(self.opt) };
        let alias = self
            .alias
            .map(|name_or_id| name_or_id.try_into())
            .transpose()?;
        let get_vertex_operator = GetVertexOperator { start_tag, opt, alias };
        debug!("Runtime get_vertex operator: {:?}", get_vertex_operator);
        Ok(Box::new(get_vertex_operator))
    }
}
