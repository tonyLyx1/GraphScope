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

use crate::progress::EndSignal;
use crate::tag::Tag;
use pegasus_common::buffer::{Batch, BatchFactory, BatchPool, BufferReader};
use pegasus_common::codec::{Decode, Encode};
use pegasus_common::io::{ReadExt, WriteExt};
use std::fmt::Debug;

pub trait Data: Clone + Send + Sync + Debug + Encode + Decode + 'static {}
impl<T: Clone + Send + Sync + Debug + Encode + Decode + 'static> Data for T {}

pub struct DataSet<T> {
    /// the tag of scope this data set belongs to;
    pub tag: Tag,
    /// the index of worker who created this dataset;
    pub src: u32,
    /// batch sequence;
    pub seq: u64,
    /// sequence of the data set;
    pub end: Option<EndSignal>,
    /// data details;
    data: Batch<T>,
    /// flag indicates if the stream is abandoned
    is_discarded: bool,
}

impl<D> DataSet<D> {
    #[inline]
    pub fn empty() -> Self {
        DataSet { tag: Tag::Root, seq: 0, end: None, src: 0, data: Batch::new(), is_discarded: false }
    }

    pub fn new(tag: Tag, src: u32, seq: u64, data: Batch<D>) -> Self {
        DataSet { tag, src, seq, end: None, data, is_discarded: false }
    }

    pub fn set_last(&mut self, mut end: EndSignal) {
        end.seq = self.seq;
        self.end = Some(end);
    }

    pub fn discard(&mut self) {
        self.is_discarded = true;
    }

    pub fn is_discarded(&self) -> bool {
        self.is_discarded
    }

    pub fn is_last(&self) -> bool {
        self.end.is_some()
    }

    pub fn take_end(&mut self) -> Option<EndSignal> {
        self.end.take()
    }

    pub fn take_batch(&mut self) -> Batch<D> {
        std::mem::replace(&mut self.data, Batch::new())
    }

    /// This is a interruptable drain, which means that the item will be removed only when it is
    /// visited by the iterator;
    #[inline]
    pub fn drain(&mut self) -> impl Iterator<Item = D> + '_ {
        &mut self.data
    }

    #[inline]
    pub fn drain_to_end(&mut self) -> impl Iterator<Item = MarkedData<D>> + '_ {
        let len = self.data.len();
        DrainEndIter { len, data: &mut self.data, end: &mut self.end, cur: 0 }
    }

    #[inline]
    pub fn tag(&self) -> Tag {
        self.tag.clone()
    }
}

struct DrainEndIter<'a, D> {
    len: usize,
    data: &'a mut Batch<D>,
    end: &'a mut Option<EndSignal>,
    cur: usize,
}

impl<'a, D> Iterator for DrainEndIter<'a, D> {
    type Item = MarkedData<D>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            if let Some(end) = self.end.take() {
                Some(MarkedData::Marked(None, end))
            } else {
                None
            }
        } else {
            if let Some(data) = self.data.next() {
                self.cur += 1;
                if self.cur == self.len {
                    // this maybe the last;
                    if let Some(end) = self.end.take() {
                        Some(MarkedData::Marked(Some(data), end))
                    } else {
                        Some(MarkedData::Data(data))
                    }
                } else {
                    Some(MarkedData::Data(data))
                }
            } else {
                None
            }
        }
    }
}

pub enum MarkedData<D> {
    Data(D),
    Marked(Option<D>, EndSignal),
}

impl<D> Debug for DataSet<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dataset[{:?}, len={}]", self.tag, self.data.len())
    }
}

impl<D> std::ops::Deref for DataSet<D> {
    type Target = Batch<D>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<D> std::ops::DerefMut for DataSet<D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> AsRef<Tag> for DataSet<T> {
    fn as_ref(&self) -> &Tag {
        &self.tag
    }
}

impl<D: Data> Encode for DataSet<D> {
    fn write_to<W: WriteExt>(&self, writer: &mut W) -> std::io::Result<()> {
        self.tag.write_to(writer)?;
        writer.write_u32(self.src)?;
        writer.write_u64(self.seq)?;
        self.end.write_to(writer)?;
        let len = self.data.len() as u64;
        writer.write_u64(len)?;
        if let Some(iter) = self.data.iter() {
            for x in iter {
                x.write_to(writer)?;
            }
        }
        if self.is_discarded {
            writer.write_u8(1)?;
        } else {
            writer.write_u8(0)?;
        }
        Ok(())
    }
}

impl<D: Data> Decode for DataSet<D> {
    fn read_from<R: ReadExt>(reader: &mut R) -> ::std::io::Result<Self> {
        let tag = Tag::read_from(reader)?;
        let src = reader.read_u32()?;
        let seq = reader.read_u64()?;
        let end = Option::<EndSignal>::read_from(reader)?;
        let len = reader.read_u64()? as usize;
        let batch = if len == 0 {
            Batch::new()
        } else {
            let mut batch = Batch::with_capacity(len);
            for _ in 0..len {
                let item = D::read_from(reader)?;
                batch.push(item);
            }
            batch
        };
        let is_discarded = if reader.read_u8()? == 1 { true } else { false };
        Ok(DataSet { tag, src, seq, end, data: batch, is_discarded })
    }
}

impl<D: Data> Clone for DataSet<D> {
    fn clone(&self) -> Self {
        DataSet {
            tag: self.tag.clone(),
            seq: self.seq,
            src: self.src,
            end: self.end.clone(),
            data: self.data.clone(),
            is_discarded: self.is_discarded,
        }
    }
}

pub(crate) struct DataSetPool<D: Send, F: BatchFactory<D>> {
    pub tag: Tag,
    pub src: u32,
    seq: u64,
    current: DataSet<D>,
    pool: BatchPool<D, F>,
}

impl<D: Send, F: BatchFactory<D>> DataSetPool<D, F> {
    pub fn new(tag: Tag, src: u32, pool: BatchPool<D, F>) -> Self {
        DataSetPool { tag, seq: 0, current: DataSet::empty(), src, pool }
    }

    pub fn take_current(&mut self) -> DataSet<D> {
        std::mem::replace(&mut self.current, DataSet::empty())
    }

    pub fn get_batch_mut(&mut self) -> Option<&mut DataSet<D>> {
        if self.current.capacity() == 0 {
            if let Some(next) = self.fetch() {
                self.current = next;
                Some(&mut self.current)
            } else {
                None
            }
        } else {
            Some(&mut self.current)
        }
    }

    fn fetch(&mut self) -> Option<DataSet<D>> {
        if let Some(batch) = self.pool.fetch() {
            let seq = self.seq;
            self.seq += 1;
            Some(DataSet::new(self.tag.clone(), self.src, seq, batch))
        } else {
            None
        }
    }

    pub fn tmp(&mut self, msg: D) -> DataSet<D> {
        let seq = self.seq;
        let mut d = DataSet::new(self.tag.clone(), self.src, seq, Batch::with_capacity(1));
        self.seq += 1;
        d.push(msg);
        d
    }

    pub fn is_idle(&self) -> bool {
        self.current.is_empty() && self.pool.is_idle()
    }

    pub fn get_seq(&self) -> u64 {
        self.seq
    }
}

////////////////////////////////

pub struct MicroBatch<T> {
    /// the tag of scope this data set belongs to;
    pub tag: Tag,
    /// the index of worker who created this dataset;
    pub src: u32,
    /// sequence of the data batch;
    pub seq: u64,
    /// if this is the last batch of a scope;
    end: Option<EndSignal>,
    /// read only data details;
    data: BufferReader<T>,
}

#[allow(dead_code)]
impl<D> MicroBatch<D> {
    #[inline]
    pub fn empty() -> Self {
        MicroBatch { tag: Tag::Root, seq: 0, src: 0, end: None, data: BufferReader::new() }
    }

    pub fn new(tag: Tag, src: u32, data: BufferReader<D>) -> Self {
        MicroBatch { tag, src, seq: 0, end: None, data }
    }

    pub fn set_end(&mut self, end: EndSignal) {
        self.end = Some(end);
    }

    pub fn set_tag(&mut self, tag: Tag) {
        if let Some(end) = self.end.as_mut() {
            end.tag = tag.clone();
        }
        self.tag = tag;
    }

    pub fn is_last(&self) -> bool {
        self.end.is_some()
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn take_end(&mut self) -> Option<EndSignal> {
        self.end.take()
    }

    pub fn share(&mut self) -> Self {
        let shared = self.data.make_share();
        MicroBatch {
            tag: self.tag.clone(),
            src: self.src,
            seq: self.seq,
            end: self.end.clone(),
            data: shared,
        }
    }

    #[inline]
    pub fn tag(&self) -> Tag {
        self.tag.clone()
    }
}

#[allow(dead_code)]
impl<D: Clone> MicroBatch<D> {
    #[inline]
    pub fn drain(&mut self) -> impl Iterator<Item = D> + '_ {
        &mut self.data
    }
}

impl<D> Debug for MicroBatch<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "batch[{:?} len={}]", self.tag, self.data.len())
    }
}

impl<D> std::ops::Deref for MicroBatch<D> {
    type Target = BufferReader<D>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// impl<D> std::ops::DerefMut for MicroBatch<D> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.data
//     }
// }

impl<D: Data> Clone for MicroBatch<D> {
    fn clone(&self) -> Self {
        MicroBatch {
            tag: self.tag.clone(),
            seq: self.seq,
            src: self.src,
            end: self.end.clone(),
            data: self.data.clone(),
        }
    }
}

impl<D: Data> Encode for MicroBatch<D> {
    fn write_to<W: WriteExt>(&self, writer: &mut W) -> std::io::Result<()> {
        self.tag.write_to(writer)?;
        writer.write_u64(self.seq)?;
        writer.write_u32(self.src)?;
        self.end.write_to(writer)?;
        let len = self.data.len() as u64;
        writer.write_u64(len)?;
        for data in self.data.iter() {
            data.write_to(writer)?;
        }
        Ok(())
    }
}

impl<D: Data> Decode for MicroBatch<D> {
    fn read_from<R: ReadExt>(_reader: &mut R) -> std::io::Result<Self> {
        // let tag = Tag::read_from(reader)?;
        // let seq = reader.read_u64()?;
        // let src = reader.read_u32()?;
        // let end = Option::<EndSignal>::read_from(reader)?;
        // let len = reader.read_u64()?;
        todo!("buffer reuse")
    }
}