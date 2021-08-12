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

use crate::process::traversal::Traverser;
use crate::structure::filter::element::ElementFilter;
use crate::structure::filter::Predicate;
use crate::structure::{AsTag, Reverse};

pub struct HasTag {
    pub tag: AsTag,
    filter: ElementFilter,
}

impl HasTag {
    pub fn new(tag: AsTag, filter: ElementFilter) -> Self {
        HasTag { tag, filter }
    }
}

impl Reverse for HasTag {
    fn reverse(&mut self) {
        self.filter.reverse();
    }
}

impl Predicate<Traverser> for HasTag {
    fn test(&self, entry: &Traverser) -> Option<bool> {
        todo!()
        // if let Some(item) = entry.select_first(self.tag) {
        //     let item = item.as_element().expect("invalid input for has tag ");
        //     self.filter.test(item)
        // } else {
        //     Some(false)
        // }
    }
}
