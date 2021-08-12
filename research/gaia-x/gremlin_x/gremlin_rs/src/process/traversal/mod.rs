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

use crate::structure::codec::ParseError;
use crate::FromPb;

bitflags! {
    #[derive(Default)]
    pub struct Requirement: u64 {
        const BULK          = 0b000000001;
        const LABELED_PATH    = 0b000000010;
        const NESTED_LOOP    = 0b000000100;
        const OBJECT        = 0b000001000;
        const ONE_BULK       = 0b000010000;
        const PATH          = 0b000100000;
        const SACK          = 0b001000000;
        const SIDE_EFFECT    = 0b010000000;
        const SINGLE_LOOP    = 0b100000000;
    }
}

// impl FromPb<Vec<pb::TraverserRequirement>> for Requirement {
//     fn from_pb(requirements_pb: Vec<pb::TraverserRequirement>) -> Result<Self, ParseError>
//     where
//         Self: Sized,
//     {
//         let mut requirements: Requirement = Default::default();
//         for requirement_pb in requirements_pb {
//             match requirement_pb {
//                 TraverserRequirement::Bulk => requirements.insert(Requirement::BULK),
//                 TraverserRequirement::LabeledPath => requirements.insert(Requirement::LABELED_PATH),
//                 TraverserRequirement::NestedLoop => requirements.insert(Requirement::NESTED_LOOP),
//                 TraverserRequirement::Object => requirements.insert(Requirement::OBJECT),
//                 TraverserRequirement::OneBulk => requirements.insert(Requirement::ONE_BULK),
//                 TraverserRequirement::Path => requirements.insert(Requirement::PATH),
//                 TraverserRequirement::Sack => requirements.insert(Requirement::SACK),
//                 TraverserRequirement::SideEffects => requirements.insert(Requirement::SIDE_EFFECT),
//                 TraverserRequirement::SingleLoop => requirements.insert(Requirement::SINGLE_LOOP),
//             }
//         }
//         Ok(requirements)
//     }
// }

pub mod pop;
pub mod step;
pub mod traverser;
pub use traverser::Traverser;
