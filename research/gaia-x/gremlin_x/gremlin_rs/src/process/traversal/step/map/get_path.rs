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
use crate::process::traversal::traverser::{Entry, Path, TravelObject};
use crate::process::traversal::Traverser;
use crate::str_err;
use crate::structure::AsTag;
use pegasus::api::function::*;

pub struct PathStep {
    pub tag: Option<AsTag>,
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

pub struct CountLocalStep {
    pub tag: Option<AsTag>,
}

impl MapFunction<Traverser, Traverser> for CountLocalStep {
    fn exec(&self, mut input: Traverser) -> FnResult<Traverser> {
        let entry = input.get_entry();
        let count = match entry {
            Entry::Element(_) => 1,
            Entry::Pair(_, _) => 1,
            Entry::Collection(collection) => collection.len() as u64,
            Entry::Group(_, _) => 1,
            Entry::Map(map) => map.len() as u64,
        };
        let count_traverser = TravelObject::Count(count);
        let mut t = input.split(count_traverser);
        if let Some(tag) = self.tag {
            t.set_as_tag(tag);
        }
        Ok(t)
    }
}
