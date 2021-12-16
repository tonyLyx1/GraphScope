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

pub mod element;
pub mod partitioner;
pub mod property;

use crate::expr::eval::Evaluator;
use crate::graph::element::{Edge, Vertex};
use ir_common::error::ParsePbError;
use ir_common::generated::algebra as algebra_pb;
use ir_common::generated::common as common_pb;
use ir_common::NameOrId;
use pegasus::api::function::{DynIter, FnResult};
use std::convert::{TryFrom, TryInto};
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;

pub type ID = u128;

/// The number of bits in an `ID`
pub const ID_BITS: usize = std::mem::size_of::<ID>() * 8;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Out = 0,
    In = 1,
    Both = 2,
}

impl From<algebra_pb::expand_base::Direction> for Direction {
    fn from(direction: algebra_pb::expand_base::Direction) -> Self
    where
        Self: Sized,
    {
        match direction {
            algebra_pb::expand_base::Direction::Out => Direction::Out,
            algebra_pb::expand_base::Direction::In => Direction::In,
            algebra_pb::expand_base::Direction::Both => Direction::Both,
        }
    }
}

#[derive(Default, Debug)]
pub struct QueryParams {
    pub labels: Vec<NameOrId>,
    pub limit: Option<usize>,
    pub props: Option<Vec<NameOrId>>,
    pub partitions: Option<Vec<u64>>,
    pub filter: Option<Arc<Evaluator>>,
}

impl TryFrom<Option<algebra_pb::QueryParams>> for QueryParams {
    type Error = ParsePbError;

    fn try_from(query_params_pb: Option<algebra_pb::QueryParams>) -> Result<Self, Self::Error> {
        query_params_pb.map_or(Ok(QueryParams::default()), |query_params_pb| {
            QueryParams::default()
                .with_labels(query_params_pb.table_names)?
                .with_filter(query_params_pb.predicate)?
                .with_limit(query_params_pb.limit)?
                .with_required_properties(query_params_pb.columns)?
                .with_extra_params(query_params_pb.requirements)
        })
    }
}

impl QueryParams {
    fn with_labels(mut self, labels_pb: Vec<common_pb::NameOrId>) -> Result<Self, ParsePbError> {
        self.labels = labels_pb
            .into_iter()
            .map(|label| label.try_into())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(self)
    }

    fn with_filter(mut self, filter_pb: Option<common_pb::SuffixExpr>) -> Result<Self, ParsePbError> {
        if let Some(filter_pb) = filter_pb {
            self.filter = Some(Arc::new(filter_pb.try_into()?));
        }
        Ok(self)
    }

    fn with_limit(mut self, limit_pb: Option<algebra_pb::Range>) -> Result<Self, ParsePbError> {
        if let Some(range) = limit_pb {
            // According to the semantics in gremlin, limit(-1) means no limit.
            if range.upper > 0 {
                self.limit = Some((range.upper - 1) as usize);
            } else if range.upper < 0 {
                Err(ParsePbError::from("Not a legal range"))?
            }
        }
        Ok(self)
    }

    // props specify the properties we query for, e.g.,
    // Some(vec![prop1, prop2]) indicates we need prop1 and prop2,
    // Some(vec![]) indicates we need all properties
    // and None indicates we do not need any property,
    fn with_required_properties(
        mut self, required_properties_pb: Vec<common_pb::NameOrId>,
    ) -> Result<Self, ParsePbError> {
        // TODO: Specify whether we need all properties or None properties
        // we assume that empty required_properties_pb vec indicates all properties needed
        self.props = Some(
            required_properties_pb
                .into_iter()
                .map(|prop_key| prop_key.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        );
        Ok(self)
    }

    // Extra query params for different storages
    fn with_extra_params(self, extra_params: Vec<String>) -> Result<Self, ParsePbError> {
        if !extra_params.is_empty() {
            Err(ParsePbError::NotSupported("extra_params in QueryParams is not supported yet".to_string()))?
        }
        Ok(self)
    }

    pub fn is_queryable(&self) -> bool {
        !(self.labels.is_empty()
            && self.filter.is_none()
            && self.limit.is_none()
            && self.partitions.is_none()
            && self.props.is_none())
    }
}

/// The function for graph query
pub trait Statement<I, O>: Send + 'static {
    fn exec(&self, next: I) -> FnResult<DynIter<O>>;
}

impl<I, O, F: 'static> Statement<I, O> for F
where
    F: Fn(I) -> FnResult<DynIter<O>> + Send + Sync,
{
    fn exec(&self, param: I) -> FnResult<DynIter<O>> {
        (self)(param)
    }
}

/// The interface of graph query in runtime
pub trait GraphProxy: Send + Sync {
    /// Scan all vertices with query parameters, and return an iterator over them.
    fn scan_vertex(&self, params: &QueryParams) -> FnResult<Box<dyn Iterator<Item = Vertex> + Send>>;

    /// Scan all edges with query parameters, and return an iterator over them.
    fn scan_edge(&self, params: &QueryParams) -> FnResult<Box<dyn Iterator<Item = Edge> + Send>>;

    /// Get vertices with the given global_ids (defined in runtime) and parameters, and return an iterator over them.
    fn get_vertex(
        &self, ids: &[ID], params: &QueryParams,
    ) -> FnResult<Box<dyn Iterator<Item = Vertex> + Send>>;

    /// Get edges with the given global_ids (defined in runtime) and parameters, and return an iterator over them.
    fn get_edge(&self, ids: &[ID], params: &QueryParams)
        -> FnResult<Box<dyn Iterator<Item = Edge> + Send>>;

    /// Get adjacent vertices of the given direction with parameters, and return the closure of Statement.
    /// We could further call the returned closure with input vertex and get its adjacent vertices.
    fn prepare_explore_vertex(
        &self, direction: Direction, params: &QueryParams,
    ) -> FnResult<Box<dyn Statement<ID, Vertex>>>;

    /// Get adjacent edges of the given direction with parameters, and return the closure of Statement.
    /// We could further call the returned closure with input vertex and get its adjacent edges.
    fn prepare_explore_edge(
        &self, direction: Direction, params: &QueryParams,
    ) -> FnResult<Box<dyn Statement<ID, Edge>>>;
}

lazy_static! {
    /// GRAPH_PROXY is a raw pointer which can be safely shared between threads.
    pub static ref GRAPH_PROXY: AtomicPtr<Arc<dyn GraphProxy>> = AtomicPtr::default();
}

pub fn register_graph(graph: Arc<dyn GraphProxy>) {
    let ptr = Box::into_raw(Box::new(graph));
    GRAPH_PROXY.store(ptr, Ordering::SeqCst);
}

pub fn get_graph() -> Option<Arc<dyn GraphProxy>> {
    let ptr = GRAPH_PROXY.load(Ordering::SeqCst);
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { (*ptr).clone() })
    }
}
