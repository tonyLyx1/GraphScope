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

use crate::process::traversal::traverser::TravelObject;
use crate::process::traversal::Traverser;
use crate::structure::{AsTag, Details};
use crate::{str_err, DynIter, DynResult, Element};
use bit_set::BitSet;
use pegasus::api::function::FlatMapFunction;

pub struct PropertiesStep {
    pub props: Vec<String>,
    pub tag: Option<AsTag>,
}

impl FlatMapFunction<Traverser, Traverser> for PropertiesStep {
    type Target = DynIter<Traverser>;

    fn exec(&self, input: Traverser) -> DynResult<DynIter<Traverser>> {
        if let Some(elem) = input.get_graph_element() {
            let mut result = vec![];
            for prop_name in self.props.iter() {
                let prop_value = elem.details().get_property(prop_name);
                if let Some(p) = prop_value.and_then(|p| p.try_to_owned()) {
                    let mut t = input.split(TravelObject::Properties((prop_name.to_owned(), p)));
                    if let Some(tag) = self.tag {
                        t.set_as_tag(tag);
                    }
                    result.push(t);
                }
            }

            Ok(Box::new(result.into_iter()))
        } else {
            Err(str_err("invalid input for values;"))
        }
    }
}
