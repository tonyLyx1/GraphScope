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

use crate::common::object::Object;
use crate::generated::gremlin as pb;
use crate::process::traversal::traverser::TravelObject;
use crate::process::traversal::Traverser;
use crate::str_err;
use crate::structure::AsTag;
use pegasus::api::function::*;

struct PathStep {
    tag: Option<AsTag>,
}

impl MapFunction<Traverser, Traverser> for PathStep {
    fn exec(&self, input: Traverser) -> FnResult<Traverser> {
        if let Some(mut t) = input.path() {
            if let Some(tag) = self.tag {
                t.set_as_tag(tag);
            }
            Ok(t)
        } else {
            Err(str_err("Traverser has no path;"))
        }
    }
}

pub struct PathLocalCountStep {
    pub tag: Option<AsTag>,
}

impl MapFunction<Traverser, Traverser> for PathLocalCountStep {
    fn exec(&self, mut input: Traverser) -> FnResult<Traverser> {
        // let count = input.get_path_len() as u64;
        // let count = TravelObject::Count(count);
        // let mut t = input.split(count);
        // if let Some(tag) = self.tag {
        //     t.set_as_tag(tag);
        // }
        // Ok(t)
        todo!()
    }
}
