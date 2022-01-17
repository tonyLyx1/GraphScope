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

use std::collections::hash_map::Entry;
use std::collections::{BTreeSet, HashMap};
use std::io;
use std::sync::RwLock;

use ir_common::generated::common as common_pb;
use ir_common::generated::schema as schema_pb;
use ir_common::NameOrId;

use crate::error::{IrError, IrResult};
use crate::JsonIO;

lazy_static! {
    pub static ref STORE_META: RwLock<StoreMeta> = RwLock::new(StoreMeta::default());
}

pub fn set_schema_from_json<R: io::Read>(read: R) {
    if let Ok(mut meta) = STORE_META.write() {
        if let Ok(schema) = Schema::from_json(read) {
            meta.schema = Some(schema);
        }
    }
}

/// The simple schema, mapping either label or property name into id.
pub fn set_schema_simple(
    entities: Vec<EntityPair>, relations: Vec<RelationTriplet>, columns: Vec<EntityPair>,
) {
    if let Ok(mut meta) = STORE_META.write() {
        let schema: Schema = (entities, relations, columns).into();
        meta.schema = Some(schema)
    }
}

pub fn reset_schema() {
    if let Ok(mut meta) = STORE_META.write() {
        meta.schema = None;
    }
}

#[derive(Clone, Debug, Default)]
pub struct StoreMeta {
    pub schema: Option<Schema>,
}

#[derive(Clone, Debug, Default)]
pub struct Column {
    name: String,
    id: i32,
    data_type: common_pb::DataType,
}

#[derive(Clone, Debug, Default)]
pub struct EntityPair {
    name: String,
    id: i32,
}

impl From<(String, i32)> for EntityPair {
    fn from(tuple: (String, i32)) -> Self {
        EntityPair { name: tuple.0, id: tuple.1 }
    }
}

#[derive(Clone, Debug, Default)]
pub struct RelationTriplet {
    edge: EntityPair,
    src: EntityPair,
    dst: EntityPair,
}

impl From<(EntityPair, EntityPair, EntityPair)> for RelationTriplet {
    fn from(tuple: (EntityPair, EntityPair, EntityPair)) -> Self {
        Self { edge: tuple.0, src: tuple.1, dst: tuple.2 }
    }
}

impl From<schema_pb::ColumnKey> for Column {
    fn from(column_pb: schema_pb::ColumnKey) -> Self {
        Column {
            name: column_pb.name.clone(),
            id: column_pb.id,
            data_type: unsafe { std::mem::transmute::<i32, common_pb::DataType>(column_pb.data_type) },
        }
    }
}

fn into_entity(entity_pb: schema_pb::Entity) -> (EntityPair, Vec<Column>) {
    let entity = EntityPair { name: entity_pb.name.clone(), id: entity_pb.id };
    let columns = entity_pb
        .columns
        .into_iter()
        .map(|col| col.into())
        .collect();

    (entity, columns)
}

fn into_entity_pb(tuple: (EntityPair, Vec<Column>)) -> schema_pb::Entity {
    schema_pb::Entity {
        id: tuple.0.id,
        name: tuple.0.name.clone(),
        columns: tuple
            .1
            .into_iter()
            .map(|col| schema_pb::ColumnKey {
                id: col.id,
                name: col.name.clone(),
                data_type: unsafe { std::mem::transmute::<common_pb::DataType, i32>(col.data_type) },
            })
            .collect(),
    }
}

fn into_relation(rel_pb: schema_pb::Relation) -> (RelationTriplet, Vec<Column>) {
    let src = EntityPair { name: rel_pb.src_name.clone(), id: rel_pb.src_id };
    let dst = EntityPair { name: rel_pb.dst_name.clone(), id: rel_pb.dst_id };
    let edge = EntityPair { name: rel_pb.name.clone(), id: rel_pb.id };
    let columns = rel_pb
        .columns
        .into_iter()
        .map(|col| col.into())
        .collect();

    (RelationTriplet { src, dst, edge }, columns)
}

fn into_relation_pb(tuple: (RelationTriplet, Vec<Column>)) -> schema_pb::Relation {
    schema_pb::Relation {
        src_id: tuple.0.src.id,
        src_name: tuple.0.src.name.clone(),
        dst_id: tuple.0.dst.id,
        dst_name: tuple.0.dst.name.clone(),
        id: tuple.0.edge.id,
        name: tuple.0.edge.name.clone(),
        columns: tuple
            .1
            .into_iter()
            .map(|col| schema_pb::ColumnKey {
                id: col.id,
                name: col.name.clone(),
                data_type: unsafe { std::mem::transmute::<common_pb::DataType, i32>(col.data_type) },
            })
            .collect(),
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum MetaType {
    Entity = 0,
    Relation = 1,
    Column = 2,
}

impl Default for MetaType {
    fn default() -> Self {
        Self::Entity
    }
}

#[derive(Clone, Debug, Default)]
pub struct Schema {
    /// A map from table name to its internally encoded id
    /// In the concept of graph database, this is also known as label
    table_map: HashMap<String, i32>,
    /// A map from column name to its internally encoded id
    /// In the concept of graph database, this is also known as property
    column_map: HashMap<String, i32>,
    /// A reversed map of `table_map` and `column_map`
    id_name_rev: [HashMap<i32, String>; 3],
    /// Is the table name mapped as id
    is_table_id: bool,
    /// Is the column name mapped as id
    is_column_id: bool,
    /// The label (string) mappings from relation to its src and dst entities
    rel_entity_labels: HashMap<String, Vec<(String, String)>>,
    /// The label (id) mappings from relation to its src and dst entities
    rel_entity_ids: HashMap<i32, Vec<(i32, i32)>>,
    /// Entities
    entities: Vec<(EntityPair, Vec<Column>)>,
    /// Relations
    rels: Vec<(RelationTriplet, Vec<Column>)>,
}

impl Schema {
    pub fn get_table_id(&self, name: &str) -> Option<i32> {
        self.table_map.get(name).cloned()
    }

    pub fn get_table_id_from_pb(&self, name: &common_pb::NameOrId) -> Option<i32> {
        name.item.as_ref().and_then(|item| match item {
            common_pb::name_or_id::Item::Name(name) => self.get_table_id(name),
            common_pb::name_or_id::Item::Id(id) => Some(*id),
        })
    }

    pub fn get_table_names(&self, ty: MetaType) -> &HashMap<i32, String> {
        let idx = unsafe { std::mem::transmute::<_, i32>(ty) } as usize;
        &self.id_name_rev[idx]
    }

    pub fn get_column_id(&self, name: &str) -> Option<i32> {
        self.column_map.get(name).cloned()
    }

    pub fn get_column_id_from_pb(&self, name: &common_pb::NameOrId) -> Option<i32> {
        name.item.as_ref().and_then(|item| match item {
            common_pb::name_or_id::Item::Name(name) => self.get_column_id(name),
            common_pb::name_or_id::Item::Id(id) => Some(*id),
        })
    }

    pub fn get_column_names(&self) -> &HashMap<i32, String> {
        &self.id_name_rev[2]
    }

    pub fn is_column_id(&self) -> bool {
        self.is_column_id
    }

    pub fn is_table_id(&self) -> bool {
        self.is_table_id
    }

    pub fn get_entity_tables(&self, rel_table: &NameOrId) -> Option<(NameOrId, NameOrId)> {
        match rel_table {
            NameOrId::Str(name) => self
                .rel_entity_labels
                .get(name)
                .map(|labels| (NameOrId::Str(labels[0].0.clone()), NameOrId::Str(labels[0].1.clone()))),
            NameOrId::Id(id) => self
                .rel_entity_ids
                .get(id)
                .map(|labels| (NameOrId::Id(labels[0].0), NameOrId::Id(labels[0].1))),
        }
    }
}

impl From<(Vec<EntityPair>, Vec<RelationTriplet>, Vec<EntityPair>)> for Schema {
    fn from(tuple: (Vec<EntityPair>, Vec<RelationTriplet>, Vec<EntityPair>)) -> Self {
        let (entities, relations, columns) = tuple;
        let mut schema = Schema::default();
        schema.is_table_id = !entities.is_empty() || !relations.is_empty();
        schema.is_column_id = !columns.is_empty();

        if schema.is_table_id {
            for pair in entities.into_iter() {
                schema
                    .table_map
                    .insert(pair.name.clone(), pair.id);
                schema.id_name_rev[0].insert(pair.id, pair.name);
            }
            for triplet in relations.into_iter() {
                let (pair, src, dst) = (triplet.edge, triplet.src, triplet.dst);
                schema
                    .table_map
                    .insert(pair.name.clone(), pair.id);
                schema.id_name_rev[1].insert(pair.id, pair.name.clone());
                schema
                    .rel_entity_ids
                    .insert(pair.id, vec![(src.id, dst.id)]);
                schema
                    .rel_entity_labels
                    .insert(pair.name, vec![(src.name, dst.name)]);
            }
        }
        if schema.is_column_id {
            for pair in columns.into_iter() {
                schema
                    .column_map
                    .insert(pair.name.clone(), pair.id);
                schema.id_name_rev[2].insert(pair.id, pair.name);
            }
        }

        schema
    }
}

impl JsonIO for Schema {
    fn into_json<W: io::Write>(self, writer: W) -> io::Result<()> {
        let entities_pb: Vec<schema_pb::Entity> = if !self.entities.is_empty() {
            self.entities
                .clone()
                .into_iter()
                .map(|tuple| into_entity_pb(tuple))
                .collect()
        } else {
            let mut entities = Vec::new();
            for (&id, name) in &self.id_name_rev[0] {
                // TODO(longbin) Write columns
                entities.push(schema_pb::Entity { id, name: name.clone(), columns: vec![] })
            }
            entities
        };

        let relations_pb: Vec<schema_pb::Relation> = if !self.rels.is_empty() {
            self.rels
                .clone()
                .into_iter()
                .map(|tuple| into_relation_pb(tuple))
                .collect()
        } else {
            let mut relations = Vec::new();
            for (&id, name) in &self.id_name_rev[1] {
                relations.push(schema_pb::Relation {
                    src_id: -1,
                    src_name: "".to_string(),
                    dst_id: -1,
                    dst_name: "".to_string(),
                    id,
                    name: name.clone(),
                    columns: vec![],
                })
            }
            relations
        };

        let schema_pb = schema_pb::Schema {
            entities: entities_pb,
            rels: relations_pb,
            is_table_id: self.is_table_id,
            is_column_id: self.is_column_id,
        };
        serde_json::to_writer_pretty(writer, &schema_pb)?;
        Ok(())
    }

    fn from_json<R: io::Read>(reader: R) -> io::Result<Self>
    where
        Self: Sized,
    {
        let schema_pb = serde_json::from_reader::<_, schema_pb::Schema>(reader)?;
        let mut schema = Schema::default();
        schema.entities = schema_pb
            .entities
            .clone()
            .into_iter()
            .map(|entity_pb| into_entity(entity_pb))
            .collect();
        schema.rels = schema_pb
            .rels
            .clone()
            .into_iter()
            .map(|rel_pb| into_relation(rel_pb))
            .collect();
        schema.is_table_id = schema_pb.is_table_id;
        schema.is_column_id = schema_pb.is_column_id;
        for entity in schema_pb.entities {
            if schema_pb.is_table_id {
                let key = &entity.name;
                if !schema.table_map.contains_key(key) {
                    schema.table_map.insert(key.clone(), entity.id);
                    schema.id_name_rev[0].insert(entity.id, key.clone());
                }
            }
            if schema_pb.is_column_id {
                for column in entity.columns {
                    let key = &column.name;
                    if !schema.column_map.contains_key(key) {
                        schema.column_map.insert(key.clone(), column.id);
                        schema.id_name_rev[2].insert(column.id, key.clone());
                    }
                }
            }
        }

        for rel in schema_pb.rels {
            if schema_pb.is_table_id {
                let key = &rel.name;
                if !schema.table_map.contains_key(key) {
                    schema.table_map.insert(key.clone(), rel.id);
                    schema.id_name_rev[1].insert(rel.id, key.clone());
                    schema
                        .rel_entity_ids
                        .insert(rel.id, vec![(rel.src_id, rel.dst_id)]);
                    schema
                        .rel_entity_labels
                        .insert(key.clone(), vec![(rel.src_name, rel.dst_name)]);
                }
            }
            if schema_pb.is_column_id {
                for column in rel.columns {
                    let key = &column.name;
                    if !schema.column_map.contains_key(key) {
                        schema.column_map.insert(key.clone(), column.id);
                        schema.id_name_rev[2].insert(column.id, key.clone());
                    }
                }
            }
        }

        Ok(schema)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
/// Record the runtime schema of the node in the logical plan, for it being the vertex/edge
pub struct NodeMeta {
    /// Mostly record whether entity (vertex) or relation (edge) this node obtains
    meta_type: MetaType,
    /// The table names (labels)
    tables: BTreeSet<NameOrId>,
    /// The required columns (columns)
    columns: BTreeSet<NameOrId>,
}

impl NodeMeta {
    pub fn new(meta_type: MetaType) -> NodeMeta {
        let mut schema = NodeMeta::default();
        schema.meta_type = meta_type;
        schema
    }

    pub fn set_meta_type(&mut self, ty: MetaType) {
        self.meta_type = ty;
    }

    pub fn get_meta_type(&self) -> MetaType {
        self.meta_type
    }

    pub fn insert_column(&mut self, col: NameOrId) {
        self.columns.insert(col);
    }

    pub fn get_columns(&self) -> &BTreeSet<NameOrId> {
        &self.columns
    }

    pub fn insert_table(&mut self, table: NameOrId) {
        self.tables.insert(table);
    }

    pub fn get_tables(&self) -> &BTreeSet<NameOrId> {
        &self.tables
    }
}

/// To record any tag-related data while processing the logical plan
#[derive(Default, Clone, Debug)]
pub struct PlanMeta {
    /// To record all possible tables/columns of a node, which is typically referred from a tag
    /// while processing projection, selection, groupby, orderby, and etc. For example, when
    /// select the record via an expression "a.name == \"John\"", the tag "a" must refer to
    /// some node in the logical plan, and the node requires the column of \"John\". Such
    /// information is critical in distributed processing, as the computation may not align
    /// with the storage to access the required column. Thus, such information can help
    /// the computation route and fetch columns.
    node_metas: HashMap<u32, NodeMeta>,
    /// The tag must refer to a valid node in the plan.
    tag_nodes: HashMap<NameOrId, u32>,
    /// To ease the processing, tag may be transformed to an internal id.
    /// This maintains the mappings
    tag_ids: HashMap<NameOrId, NameOrId>,
    /// To record the current node's id in the logical plan. Note that nodes that have operators that
    /// of `As` or `Selection` does not alter curr_node.
    curr_node: u32,
    /// The maximal tag id that has been assigned, for mapping tag ids.
    max_tag_id: u32,
    /// Whether to preprocess the operator.
    is_preprocess: bool,
    /// Whether to partition the task
    is_partition: bool,
}

impl PlanMeta {
    pub fn new(node_id: u32) -> Self {
        let mut plan_meta = PlanMeta::default();
        plan_meta.curr_node = node_id;
        plan_meta.node_metas.entry(node_id).or_default();
        plan_meta
    }

    pub fn curr_node_meta_mut(&mut self) -> &mut NodeMeta {
        self.node_metas
            .entry(self.curr_node)
            .or_default()
    }

    pub fn tag_node_meta_mut(&mut self, tag_opt: Option<&NameOrId>) -> IrResult<&mut NodeMeta> {
        if let Some(tag) = tag_opt {
            if let Some(&node_id) = self.tag_nodes.get(tag) {
                Ok(self.node_metas.entry(node_id).or_default())
            } else {
                Err(IrError::TagNotExist(tag.clone()))
            }
        } else {
            Ok(self.curr_node_meta_mut())
        }
    }

    pub fn get_node_meta(&self, id: u32) -> Option<&NodeMeta> {
        self.node_metas.get(&id)
    }

    pub fn get_all_node_metas(&self) -> &HashMap<u32, NodeMeta> {
        &self.node_metas
    }

    pub fn curr_node_meta(&self) -> Option<&NodeMeta> {
        self.get_node_meta(self.curr_node)
    }

    pub fn insert_tag_node(&mut self, tag: NameOrId, node: u32) {
        self.tag_nodes.entry(tag).or_insert(node);
    }

    pub fn get_tag_node(&self, tag: &NameOrId) -> Option<u32> {
        self.tag_nodes.get(tag).cloned()
    }

    pub fn get_all_tag_nodes(&self) -> &HashMap<NameOrId, u32> {
        &self.tag_nodes
    }

    pub fn get_or_set_tag_id(&mut self, tag: NameOrId) -> &NameOrId {
        let entry = self.tag_ids.entry(tag);
        match entry {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                let new_tag_id: NameOrId = (self.max_tag_id as i32).into();
                self.max_tag_id += 1;
                v.insert(new_tag_id)
            }
        }
    }

    pub fn set_curr_node(&mut self, curr_node: u32) {
        self.curr_node = curr_node;
    }

    pub fn get_curr_node(&self) -> u32 {
        self.curr_node
    }

    pub fn is_preprocess(&self) -> bool {
        self.is_preprocess
    }

    pub fn set_preprocess(&mut self, is_preprocess: bool) {
        self.is_preprocess = is_preprocess;
    }

    pub fn is_partition(&self) -> bool {
        self.is_partition
    }

    pub fn set_partition(&mut self, is_partition: bool) {
        self.is_partition = is_partition;
    }
}
