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
use crate::generated::gremlin::select_step::Pop;
use crate::process::traversal::step::select_by::{ByModulating, SelectKey};
use crate::process::traversal::step::MapFuncGen;
use crate::process::traversal::traverser::TravelObject;
use crate::process::traversal::Traverser;
use crate::{str_err, DynResult, Element, FromPb};
use pegasus::api::function::*;

struct SelectStep {
    pop: Pop,
    multi_key_by: Vec<(SelectKey, ByModulating)>,
}

impl MapFunction<Traverser, Traverser> for SelectStep {
    fn exec(&self, mut input: Traverser) -> FnResult<Traverser> {
        for (key, by_mod) in self.multi_key_by.iter() {
            let selected_entry = key.select_pop(&input, self.pop).ok_or(str_err("select None entry"))?;
            let new_entry = by_mod.modulate_by(selected_entry)?;
            let tag = key.get_tag().ok_or(str_err("SelectKey is not Tag"))?;
            // TODO(bingqing): We add multi select result into select_result, but current traverser head (entry) remains unchanged
            input.add_select_result(tag, new_entry);
        }
        Ok(input)
    }
}

impl MapFuncGen for pb::SelectStep {
    fn gen_map(self) -> DynResult<Box<dyn MapFunction<Traverser, Traverser>>> {
        todo!()
    }
}
