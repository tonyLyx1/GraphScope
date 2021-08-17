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

use crate::process::traversal::step::*;
// use crate::process::traversal::step::{BySubJoin, HasAnyJoin};
// use crate::process::traversal::traverser::TraverserGroupKey;
use crate::process::traversal::Traverser;
use crate::structure::Element;
use crate::{str_err, Object, TraverserSinkEncoder};
use crate::{DynResult, Partitioner};
// use pegasus::api::function::CompareFunction;
use pegasus::api::function::{DynIter, FilterFunction, FlatMapFunction, MapFunction};
use pegasus::api::{Count, Fold, ReduceByKey, Source};
// use pegasus::api::{Counter, MaxMin, OrderDirect, ReduceByKeyWith, Sum};
use pegasus::api::{Filter, IntoDataflow, IterCondition, Iteration, Limit, Map, Unary};
// use pegasus::api::{Sinkable, SubTask};
use pegasus::api::{Sort, SortBy};
use pegasus::result::ResultSink;
use pegasus::stream::Stream;
use pegasus::{BuildJobError, Data};
// use pegasus_server::factory::JobParser;
// use pegasus_server::pb::fold::Kind;
// use pegasus_server::pb::operator_def::OpKind;
use crate::generated::gremlin as gremlin_pb;
use pegasus_server::pb as server_pb;
use pegasus_server::pb::operator::OpKind;
use pegasus_server::pb::{BinaryResource, MapKind, Operator};
use pegasus_server::service::JobParser;
use pegasus_server::JobRequest;
use prost::Message;
use std::sync::Arc;

type TraverserMap = Box<dyn MapFunction<Traverser, Traverser>>;
type TraverserFlatMap = Box<dyn FlatMapFunction<Traverser, Traverser, Target = DynIter<Traverser>>>;
type TraverserFilter = Box<dyn FilterFunction<Traverser>>;
// type TraverserCompare = Box<dyn CompareFunction<Traverser>>;

pub struct GremlinJobCompiler {
    partitioner: Arc<dyn Partitioner>,
    udf_gen: FnGenerator,
    num_servers: usize,
    server_index: u64,
}

struct FnGenerator {}

impl FnGenerator {
    fn new() -> Self {
        FnGenerator {}
    }

    fn gen_source(
        &self, res: &BinaryResource, num_servers: usize, server_index: u64,
    ) -> Result<DynIter<Traverser>, BuildJobError> {
        // let mut step = decode::<pb::gremlin::GremlinStep>(&res.resource)?;
        // let worker_id = pegasus::get_current_worker().expect("unreachable");
        // let num_workers = worker_id.peers as usize / num_servers;
        // let mut step = graph_step_from(&mut step, num_servers)?;
        // step.set_num_workers(num_workers);
        // step.set_server_index(server_index);
        // Ok(step.gen_source(Some(worker_id.index as usize)))
        todo!()
    }

    fn gen_map(&self, res: &BinaryResource) -> Result<TraverserMap, BuildJobError> {
        let step = decode::<gremlin_pb::GremlinStep>(&res.resource)?;
        Ok(step.gen_map()?)
    }

    fn gen_flat_map(&self, res: &BinaryResource) -> Result<TraverserFlatMap, BuildJobError> {
        // let step = decode::<pb::gremlin::GremlinStep>(&res.resource)?;
        // Ok(step.gen_flat_map()?)
        todo!()
    }

    fn gen_filter(&self, res: &BinaryResource) -> Result<TraverserFilter, BuildJobError> {
        // let step = decode::<pb::gremlin::GremlinStep>(&res.resource)?;
        // Ok(step.gen_filter()?)
        todo!()
    }

    fn get_subtask_kind(&self, res: &BinaryResource) -> Result<SubTraversalKind, BuildJobError> {
        todo!()
    }
}

impl GremlinJobCompiler {
    pub fn new<D: Partitioner>(partitioner: D, num_servers: usize, server_index: u64) -> Self {
        GremlinJobCompiler {
            partitioner: Arc::new(partitioner),
            udf_gen: FnGenerator::new(),
            num_servers,
            server_index,
        }
    }

    pub fn get_num_servers(&self) -> usize {
        self.num_servers
    }

    pub fn get_server_index(&self) -> u64 {
        self.server_index
    }

    fn install(
        &self, mut stream: Stream<Traverser>, plan: &[Operator],
    ) -> Result<Stream<Traverser>, BuildJobError> {
        for op in &plan[..] {
            if let Some(ref op) = op.op_kind {
                match op {
                    OpKind::Map(map) => {
                        if let Some(ref res) = map.resource {
                            let map_kind: server_pb::MapKind = unsafe { std::mem::transmute(map.kind) };
                            match map_kind {
                                server_pb::MapKind::Map => {
                                    let func = self.udf_gen.gen_map(res)?;
                                    stream = stream.map(move |input| func.exec(input))?;
                                }
                                server_pb::MapKind::Flatmap => {
                                    let func = self.udf_gen.gen_flat_map(res)?;
                                    stream = stream.flat_map(move |input| func.exec(input))?;
                                }
                                server_pb::MapKind::FilterMap => {
                                    todo!()
                                }
                                server_pb::MapKind::Filter => {
                                    let func = self.udf_gen.gen_filter(res)?;
                                    stream = stream.filter(move |input| func.test(input))?;
                                }
                            }
                        } else {
                            Err("map function not found;")?
                        }
                    }
                    OpKind::Comm(_) => {}
                    OpKind::Agg(_) => {}
                    OpKind::Iter(_) => {}
                }
            }
        }
        // for op in &plan[..] {
        //     if let Some(ref op) = op.op_kind {
        //         match op {
        //             OpKind::Exchange(_) => {
        //                 let p = self.partitioner.clone();
        //                 let worker_id = pegasus::get_current_worker().expect("unreachable");
        //                 let num_workers = worker_id.peers as usize / self.num_servers;
        //                 stream = stream.exchange_fn(move |t| {
        //                     Ok(if let Some(e) = t.get_graph_element() {
        //                         p.get_partition(&e.id(), num_workers)
        //                     } else {
        //                         0
        //                     })
        //                 });
        //             }
        //             OpKind::Broadcast(_) => {
        //                 stream = stream.broadcast();
        //             }
        //             OpKind::Aggregate(target) => {
        //                 stream = stream.aggregate_to(target.target);
        //             }
        //             OpKind::Map(map) => {
        //                 if let Some(ref res) = map.func {
        //                     let func = self.udf_gen.gen_map(res)?;
        //                     stream = stream.map(func)?;
        //                 } else {
        //                     Err("map function not found;")?
        //                 }
        //             }
        //             OpKind::FlatMap(map) => {
        //                 if let Some(ref res) = map.func {
        //                     let func = self.udf_gen.gen_flat_map(res)?;
        //                     stream = stream.flat_map(func)?;
        //                 } else {
        //                     Err("flat_map function not found;")?
        //                 }
        //             }
        //             OpKind::Filter(filter) => {
        //                 if let Some(ref res) = filter.func {
        //                     let func = self.udf_gen.gen_filter(res)?;
        //                     stream = stream.filter(func)?;
        //                 } else {
        //                     Err("filter function not found")?
        //                 }
        //             }
        //             OpKind::Limit(n) => {
        //                 stream = stream.limit(n.limit)?.aggregate_to(0).limit(n.limit)?;
        //             }
        //             OpKind::Order(ord) => {
        //                 if let Some(ref res) = ord.cmp {
        //                     // TODO(bingqing): why order is gen_map?
        //                     let set_order_key = self.udf_gen.gen_map(res)?;
        //                     stream = stream.map(set_order_key)?;
        //                 }
        //                 if let Some(ref limit) = ord.limit {
        //                     stream = stream
        //                         .top(limit.limit, OrderDirect::Asc)?
        //                         .aggregate_to(0)
        //                         .top(limit.limit, OrderDirect::Asc)?;
        //                 } else {
        //                     stream = stream.aggregate_to(0).sort(OrderDirect::Asc)?;
        //                 }
        //             }
        //             OpKind::Fold(fold) => {
        //                 if let Some(ref accum) = fold.kind {
        //                     match accum {
        //                         Kind::Cnt(_) => {
        //                             stream = stream
        //                                 .count()?
        //                                 .aggregate_to(0)
        //                                 .sum(0)?
        //                                 .map_fn(|v| Ok(Traverser::count(v)))?;
        //                         }
        //                         Kind::Sum(_) => {
        //                             stream = stream
        //                                 .sum(Traverser::sum(0))?
        //                                 .aggregate_to(0)
        //                                 .sum(Traverser::sum(0))?
        //                         }
        //                         Kind::Max(_) => {
        //                             stream = stream
        //                                 .max()?
        //                                 .filter_map_fn(|v| Ok(v))?
        //                                 .aggregate_to(0)
        //                                 .max()?
        //                                 .filter_map_fn(|v| Ok(v))?;
        //                         }
        //                         Kind::Min(_) => {
        //                             stream = stream
        //                                 .min()?
        //                                 .filter_map_fn(|v| Ok(v))?
        //                                 .aggregate_to(0)
        //                                 .min()?
        //                                 .filter_map_fn(|v| Ok(v))?;
        //                         }
        //                         Kind::List(_) => {
        //                             todo!()
        //                         }
        //                         Kind::Set(_) => {
        //                             todo!()
        //                         }
        //                         Kind::Custom(_res) => {
        //                             todo!()
        //                         }
        //                     }
        //                 } else {
        //                     Err("fold kind not found")?
        //                 }
        //             }
        //             OpKind::Group(group) => {
        //                 todo!()
        //                 // if let Some(kind) = group.fold.as_ref().and_then(|f| f.kind.as_ref()) {
        //                 //     match kind {
        //                 //         Kind::Cnt(_) => {
        //                 //             stream = stream
        //                 //                 .reduce_by_with(TraverserGroupKey, Counter::default)?
        //                 //                 .map_fn(|(k, v)| Ok(Traverser::group_count(k, v)))?;
        //                 //         }
        //                 //         Kind::Sum(_) => {
        //                 //             todo!()
        //                 //         }
        //                 //         Kind::Max(_) => {
        //                 //             todo!()
        //                 //         }
        //                 //         Kind::Min(_) => {
        //                 //             todo!()
        //                 //         }
        //                 //         Kind::List(_) => {
        //                 //             stream = stream
        //                 //                 .reduce_by(TraverserGroupKey)?
        //                 //                 .map_fn(|(k, v)| Ok(Traverser::group_by(k, v)))?;
        //                 //         }
        //                 //         Kind::Set(_) => {
        //                 //             todo!()
        //                 //         }
        //                 //         Kind::Custom(_) => {
        //                 //             todo!()
        //                 //         }
        //                 //     }
        //                 // } else {
        //                 //     Err("group function not found;")?;
        //                 // }
        //             }
        //             OpKind::Unfold(_) => {
        //                 stream = stream.flat_map_fn(|t| Ok(t.unfold()))?;
        //             }
        //             OpKind::Dedup(_) => {
        //                 // 1. set dedup key if needed;
        //                 // 2. dedup by dedup key;
        //                 todo!()
        //             }
        //             OpKind::Union(_) => {
        //                 todo!()
        //             }
        //             OpKind::Iterate(iter) => {
        //                 let until =
        //                     if let Some(condition) = iter.until.as_ref().and_then(|f| f.func.as_ref()) {
        //                         let cond = self.udf_gen.gen_filter(condition)?;
        //                         let mut until = IterCondition::new();
        //                         until.until(cond);
        //                         until.max_iters = iter.max_iters;
        //                         until
        //                     } else {
        //                         IterCondition::max_iters(iter.max_iters)
        //                     };
        //                 if let Some(ref iter_body) = iter.body {
        //                     stream = stream
        //                         .iterate_until(until, |start| self.install(start, &iter_body.plan[..]))?;
        //                 } else {
        //                     Err("iteration body can't be empty;")?
        //                 }
        //             }
        //             OpKind::Subtask(sub) => {
        //                 if let Some(ref body) = sub.task {
        //                     let sub_start = stream.clone();
        //                     let sub_end =
        //                         sub_start.fork_subtask(|start| self.install(start, &body.plan[..]))?;
        //                     match self
        //                         .udf_gen
        //                         .get_subtask_kind(sub.join.as_ref().expect("join notfound"))?
        //                     {
        //                         SubTraversalKind::GetGroupKey => {
        //                             stream = stream.join_subtask_fn(sub_end, |_info| {
        //                                 |parent, sub| {
        //                                     let mut p = std::mem::replace(parent, Default::default());
        //                                     p.set_group_key(sub);
        //                                     Ok(Some(p))
        //                                 }
        //                             })?;
        //                         }
        //                         SubTraversalKind::GetOrderKey => {
        //                             stream = stream.join_subtask_fn(sub_end, |_info| {
        //                                 |parent, sub| {
        //                                     let mut p = std::mem::replace(parent, Default::default());
        //                                     p.add_order_key(sub);
        //                                     Ok(Some(p))
        //                                 }
        //                             })?;
        //                         }
        //                         SubTraversalKind::GetDedupKey => {
        //                             stream = stream.join_subtask_fn(sub_end, |_info| {
        //                                 |parent, sub| {
        //                                     let mut p = std::mem::replace(parent, Default::default());
        //                                     p.set_dedup_key(sub);
        //                                     Ok(Some(p))
        //                                 }
        //                             })?;
        //                         }
        //                         SubTraversalKind::PrepareSelect(tag) => {
        //                             stream = stream.join_subtask_fn(sub_end, |_info| {
        //                                 move |parent, sub| {
        //                                     let mut p = std::mem::replace(parent, Default::default());
        //                                     p.add_select_result(tag, sub);
        //                                     Ok(Some(p))
        //                                 }
        //                             })?;
        //                         }
        //                         SubTraversalKind::WherePredicate => {
        //                             stream = stream.join_subtask_fn(sub_end, |_info| {
        //                                 |parent, _sub| {
        //                                     let p = std::mem::replace(parent, Default::default());
        //                                     Ok(Some(p))
        //                                 }
        //                             })?;
        //                         }
        //                         SubTraversalKind::GetGroupValue => {
        //                             stream = stream.join_subtask_fn(sub_end, |_info| {
        //                                 |parent, sub| {
        //                                     let mut p = std::mem::replace(parent, Default::default());
        //                                     p.update_group_value(sub);
        //                                     Ok(Some(p))
        //                                 }
        //                             })?;
        //                         }
        //                     }
        //                 }
        //             }
        //         }
        //     } else {
        //         Err("Unknown operator with empty kind;")?;
        //     }
        // }
        Ok(stream)
    }
}

impl JobParser<Traverser, Traverser> for GremlinJobCompiler {
    fn parse(
        &self, plan: &JobRequest, input: &mut Source<Traverser>, output: ResultSink<Traverser>,
    ) -> Result<(), BuildJobError> {
        unimplemented!()
    }
}

#[inline]
fn decode<T: Message + Default>(binary: &[u8]) -> Result<T, BuildJobError> {
    Ok(T::decode(binary).map_err(|e| format!("protobuf decode failure: {}", e))?)
}
