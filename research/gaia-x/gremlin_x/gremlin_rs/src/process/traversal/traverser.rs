use crate::structure::{AsTag, GraphElement};
use crate::{str_err, DynError, Object};
use pegasus::api::function::{DynIter, FnResult};
use pegasus::api::{Key, KeySelector};
use pegasus::codec::*;
use pegasus::Data;
use pegasus_common::collections::Collection;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::AddAssign;

#[derive(Debug, Clone)]
pub enum TravelObject {
    None,
    /// from the ".value()" or ".values('xxx')" step;
    Value(Object),
    /// from the ".properties('xxx')" step;
    Properties((String, Object)),
    /// from the ".valueMap('xxx')" or ".elementMap('xxx')" or ".propertyMap('xxx')" steps;
    ValueMap(Vec<(String, Object)>),
    /// from the ".count()" step;
    Count(u64),
    /// from the ".sum()" step;
    Sum(Object),

    Entry(Object),
}

#[derive(Debug, Clone)]
pub enum Element {
    InGraph(GraphElement),
    OutGraph(TravelObject),
}

#[derive(Debug, Clone)]
pub enum Entry {
    Element(Element),
    Pair(Element, Element),
    Collection(Vec<Element>),
    Group(Element, Vec<Element>),
}

#[derive(Debug, Clone)]
pub enum Path {
    LabeledPath(Vec<(AsTag, Entry)>),
    Path(Vec<(Option<AsTag>, Entry)>),
}

#[derive(Debug, Clone, Default)]
pub struct Traverser {
    entry: Entry,
    path: Option<Path>,
    tag: Option<AsTag>,
    group_key: Option<Entry>,
    order_key: Vec<Entry>,
    dedup_key: Option<Entry>,
    selected: Vec<(AsTag, Entry)>,
}

pub struct TraverserGroupKey;

impl KeySelector<Traverser> for TraverserGroupKey {
    type Target = Entry;

    fn select(&self, item: &Traverser) -> FnResult<Cow<Self::Target>> {
        todo!()
    }
}

impl Traverser {
    pub fn new<E: Into<Entry>>(entry: E) -> Self {
        Traverser { entry: entry.into(), ..Self::default() }
    }

    pub fn with_labeled_path<E: Into<Entry>>(entry: E) -> Self {
        Traverser { entry: entry.into(), path: Some(Path::LabeledPath(vec![])), ..Self::default() }
    }

    pub fn with_path<E: Into<Entry>>(entry: E) -> Self {
        Traverser { entry: entry.into(), path: Some(Path::Path(vec![])), ..Self::default() }
    }

    pub fn count(cnt: u64) -> Self {
        let cnt = TravelObject::Count(cnt);
        Traverser::new(cnt)
    }

    pub fn sum(sum: u64) -> Self {
        let cnt = TravelObject::Sum(sum.into());
        Traverser::new(cnt)
    }

    pub fn group_count<E: Into<Entry>>(key: E, count: u64) -> Self {
        let count = Element::OutGraph(TravelObject::Count(count));
        let key = key.into();
        Traverser { entry: Entry::Pair(key.into(), count), ..Self::default() }
    }

    pub fn group_by<K: Into<Entry>, V: Into<Entry>>(key: K, value: Vec<V>) -> Self {
        let key = key.into();
        let mut v = Vec::with_capacity(value.len());
        for item in value {
            let entry = item.into();
            v.push(entry.into());
        }
        let entry = Entry::Group(key.into(), v);
        Traverser { entry, ..Self::default() }
    }

    pub fn group_fold<K: Into<Entry>, V: Into<Entry>>(key: K, value: V) -> Self {
        let key = key.into();
        let value = value.into();
        let entry = Entry::Pair(key.into(), value.into());
        Traverser { entry, ..Self::default() }
    }

    pub fn set_as_tag(&mut self, as_tag: AsTag) {
        self.tag = Some(as_tag);
    }

    pub fn set_group_key<E: Into<Entry>>(&mut self, key: E) {
        self.group_key = Some(key.into());
    }

    pub fn add_order_key<E: Into<Entry>>(&mut self, key: E) {
        self.order_key.push(key.into());
    }

    pub fn set_dedup_key<E: Into<Entry>>(&mut self, key: E) {
        self.dedup_key = Some(key.into());
    }

    pub fn add_select_result<E: Into<Entry>>(&mut self, tag: AsTag, value: E) {
        self.selected.push((tag, value.into()));
    }

    pub fn update_group_value<E: Into<Entry>>(&mut self, value: E) -> Result<(), DynError> {
        match self.entry {
            Entry::Group(_, ref mut values) => {
                let v = value.into();
                values.clear();
                values.push(v.into());
                Ok(())
            }
            Entry::Pair(_, ref mut v) => {
                let value = value.into();
                *v = value.into();
                Ok(())
            }
            _ => Err(str_err("no group struct found")),
        }
    }

    pub fn split<E: Into<Entry>>(&self, entry: E) -> Self {
        match self.path {
            Some(Path::LabeledPath(ref path)) => {
                let mut path = path.clone();
                if let Some(tag) = self.tag {
                    // TODO: optimize clone;
                    let old = self.entry.clone();
                    path.push((tag, old));
                }
                Traverser { entry: entry.into(), path: Some(Path::LabeledPath(path)), ..Self::default() }
            }
            Some(Path::Path(ref path)) => {
                let mut path = path.clone();
                let old = self.entry.clone();
                path.push((self.tag, old.into()));
                Traverser { entry: entry.into(), path: Some(Path::Path(path)), ..Self::default() }
            }
            None => Traverser { entry: entry.into(), ..Self::default() },
        }
    }

    pub fn get_entry(&self) -> &Entry {
        &self.entry
    }

    pub fn get_entry_mut(&mut self) -> &mut Entry {
        &mut self.entry
    }

    pub fn get_graph_element(&self) -> Option<&GraphElement> {
        match self.entry {
            Entry::Element(Element::InGraph(ref e)) => Some(e),
            _ => None,
        }
    }

    pub fn get_graph_element_mut(&mut self) -> Option<&mut GraphElement> {
        match self.entry {
            Entry::Element(Element::InGraph(ref mut e)) => Some(e),
            _ => None,
        }
    }

    pub fn select_first(&self, tag: AsTag) -> Option<Cow<Entry>> {
        if !self.selected.is_empty() {
            // // self.selected.retain(|(_, v)| !v.is_none());
            // if !self.selected.is_empty() {
            //     for (k, v) in self.selected.iter_mut() {
            //         if *k == tag {
            //             let entry = std::mem::replace(v, Entry::default());
            //             return Some(Cow::Owned(entry));
            //         }
            //     }
            // }
        }

        match self.path {
            Some(Path::LabeledPath(ref path)) => {
                for (k, v) in path.iter() {
                    if *k == tag {
                        return Some(Cow::Borrowed(v));
                    }
                }
                None
            }
            Some(Path::Path(ref path)) => {
                for (k, v) in path.iter() {
                    if let Some(key) = k {
                        if *key == tag {
                            return Some(Cow::Borrowed(v));
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }

    pub fn select_last(&self, tag: AsTag) -> Option<Cow<Entry>> {
        if !self.selected.is_empty() {
            // self.selected.retain(|(_, v)| !v.is_none());
            // if !self.selected.is_empty() {
            //     for (k, v) in self.selected.iter_mut() {
            //         if *k == tag {
            //             let entry = std::mem::replace(v, Entry::default());
            //             return Some(Cow::Owned(entry));
            //         }
            //     }
            // }
        }

        match self.path {
            Some(Path::LabeledPath(ref path)) => {
                for (k, v) in path.iter().rev() {
                    if *k == tag {
                        return Some(Cow::Borrowed(v));
                    }
                }
                None
            }
            Some(Path::Path(ref path)) => {
                for (k, v) in path.iter().rev() {
                    if let Some(key) = k {
                        if *key == tag {
                            return Some(Cow::Borrowed(v));
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }

    pub fn select_all(&self, tag: AsTag) -> Vec<Cow<Entry>> {
        let mut result = vec![];
        if !self.selected.is_empty() {
            // self.selected.retain(|(_, v)| !v.is_none());
            // if !self.selected.is_empty() {
            //     for (k, v) in self.selected.iter_mut() {
            //         if *k == tag {
            //             let entry = std::mem::replace(v, Entry::default());
            //             result.push(Cow::Owned(entry));
            //         }
            //     }
            // }
        }

        match self.path {
            Some(Path::LabeledPath(ref path)) => {
                for (k, v) in path.iter() {
                    if *k == tag {
                        result.push(Cow::Borrowed(v));
                    }
                }
            }
            Some(Path::Path(ref path)) => {
                for (k, v) in path.iter() {
                    if let Some(key) = k {
                        if *key == tag {
                            result.push(Cow::Borrowed(v));
                        }
                    }
                }
            }
            _ => (),
        }
        result
    }

    pub fn has_cyclic_path(&self) -> bool {
        todo!()
    }

    pub fn unfold(self) -> DynIter<Traverser> {
        todo!()
    }

    pub fn path(self) -> Option<Traverser> {
        match self.path {
            Some(Path::Path(mut path)) => {
                let mut p = Vec::with_capacity(path.len() + 1);
                for (_, e) in path.drain(..) {
                    p.push(e.into());
                }
                p.push(self.entry.into());
                let entry = Entry::Collection(p);
                Some(Traverser { entry, ..Self::default() })
            }
            _ => {
                // can't get path from labeled path or no path traverser;
                None
            }
        }
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for Entry {}

impl Hash for Entry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl Encode for Entry {
    fn write_to<W: WriteExt>(&self, writer: &mut W) -> std::io::Result<()> {
        todo!()
    }
}

impl Decode for Entry {
    fn read_from<R: ReadExt>(reader: &mut R) -> std::io::Result<Self> {
        todo!()
    }
}

impl Encode for Traverser {
    fn write_to<W: WriteExt>(&self, writer: &mut W) -> std::io::Result<()> {
        todo!()
    }
}

impl Decode for Traverser {
    fn read_from<R: ReadExt>(reader: &mut R) -> std::io::Result<Self> {
        todo!()
    }
}

impl PartialEq for Traverser {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialOrd for Traverser {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl Eq for Traverser {}

impl Ord for Traverser {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}

impl AddAssign for Traverser {
    fn add_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl Default for TravelObject {
    fn default() -> Self {
        TravelObject::None
    }
}

impl TravelObject {
    pub fn is_none(&self) -> bool {
        match self {
            TravelObject::None => true,
            _ => false,
        }
    }
}

impl Default for Element {
    fn default() -> Self {
        Element::OutGraph(TravelObject::None)
    }
}

impl Element {
    pub fn is_none(&self) -> bool {
        match self {
            Element::InGraph(_) => false,
            Element::OutGraph(o) => o.is_none(),
        }
    }
}

impl Default for Entry {
    fn default() -> Self {
        Entry::Element(Default::default())
    }
}

impl Into<Entry> for GraphElement {
    fn into(self) -> Entry {
        Entry::Element(Element::InGraph(self))
    }
}

impl Into<Entry> for TravelObject {
    fn into(self) -> Entry {
        Entry::Element(Element::OutGraph(self))
    }
}

impl Into<Entry> for Traverser {
    fn into(self) -> Entry {
        self.entry
    }
}

impl Into<Object> for Entry {
    fn into(self) -> Object {
        todo!()
    }
}

impl Into<Element> for Entry {
    fn into(self) -> Element {
        match self {
            Entry::Element(e) => e,
            others => {
                let t = TravelObject::Entry(others.into());
                Element::OutGraph(t)
            }
        }
    }
}
