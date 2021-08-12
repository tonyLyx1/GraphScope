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
use crate::process::traversal::pop::Pop;
use crate::process::traversal::step::MapFuncGen;
use crate::process::traversal::Traverser;
use crate::{str_err, DynResult, Element, FromPb};
use pegasus::api::function::*;

struct SelectStep {
    pop: Pop,
}

impl MapFunction<Traverser, Traverser> for SelectStep {
    fn exec(&self, _input: Traverser) -> FnResult<Traverser> {
        todo!()
    }
}

impl MapFuncGen for pb::SelectStep {
    fn gen_map(self) -> DynResult<Box<dyn MapFunction<Traverser, Traverser>>> {
        todo!()
    }
}
