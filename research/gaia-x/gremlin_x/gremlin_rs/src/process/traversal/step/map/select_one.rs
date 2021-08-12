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

use crate::process::traversal::step::select_by::{ByModulating, SelectKey};
use crate::process::traversal::Traverser;
use crate::str_err;
use crate::structure::AsTag;
use pegasus::api::function::{FnResult, MapFunction};

/// select forward and inline project;
///
/// select(keys) => { key = [`SelectKey::Current`], by_mod = [`ByModulating::Keys`] }
/// select(values) => { key = [`SelectKey::Current`], by_mod = [`ByModulating::Values`] }
/// select('a') => { key = [`SelectKey::Tagged(['a'])`], by_mod = [`ByModulating::Itself`] }
/// select('a').by(~Id) => { key = [`SelectKey::Tagged(['a'])`], by_mod = [ByModulating::Id] }
/// select('a').by(~Label) => { key = [`SelectKey::Tagged(['a'])`], by_mod = [ByModulating::Label] }
/// select('a').by('name') => { key = [`SelectKey::Tagged(['a'])`], by_mod = [ByModulating::Property('name')] }
/// select('a').by(properties('name', 'age')) =>  { key = [`SelectKey::Tagged(['a'])`], by_mod = [ByModulating::Properties(['name', 'age'])] }  
/// select('a').by(valueMap('name', 'age')) => { key = [`SelectKey::Tagged(['a'])`], by_mod = [ByModulating::ValueMap(['name', 'age'])] }
/// select('a').by($traversal) => { key = [`SelectKey::Tagged(['a'])`], by_mod = [ByModulating::Prepared] } where the `$traversal` result is prepared
/// in advance by fork subtask;
pub struct SelectOneStep {
    key: SelectKey,
    by_mod: ByModulating,
    tag: Option<AsTag>,
}

impl MapFunction<Traverser, Traverser> for SelectOneStep {
    fn exec(&self, _input: Traverser) -> FnResult<Traverser> {
        todo!()
    }
}
