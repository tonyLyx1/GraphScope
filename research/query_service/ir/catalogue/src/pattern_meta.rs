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

use std::collections::{BTreeMap, BTreeSet};

use ir_core::plan::meta::Schema;

use super::pattern::Direction;

#[derive(Debug)]
pub struct PatternMeta {
    vertex_map: BTreeMap<String, i32>,
    edge_map: BTreeMap<String, i32>,
    vertex_connect_edges: BTreeMap<i32, Vec<(i32, Direction)>>,
    edge_connect_vertices: BTreeMap<i32, Vec<(i32, i32)>>,
    vertex_vertex_edges: BTreeMap<(i32, i32), Vec<(i32, Direction)>>,
}

impl PatternMeta {
    pub fn get_vertex_num(&self) -> usize {
        self.vertex_map.len()
    }

    pub fn get_edge_num(&self) -> usize {
        self.edge_map.len()
    }

    pub fn get_all_vertex_ids(&self) -> Vec<i32> {
        let mut all_vertex_ids = Vec::with_capacity(self.vertex_map.len());
        for (_, vertex_id) in &self.vertex_map {
            all_vertex_ids.push(*vertex_id)
        }
        all_vertex_ids
    }

    pub fn get_all_edge_ids(&self) -> Vec<i32> {
        let mut all_edge_ids = Vec::with_capacity(self.edge_map.len());
        for (_, edge_id) in &self.edge_map {
            all_edge_ids.push(*edge_id)
        }
        all_edge_ids
    }

    pub fn get_vertex_id(&self, label_name: &str) -> Option<i32> {
        match self.vertex_map.get(label_name) {
            Some(id) => Some(*id),
            None => None,
        }
    }

    pub fn get_edge_id(&self, label_name: &str) -> Option<i32> {
        match self.edge_map.get(label_name) {
            Some(id) => Some(*id),
            None => None,
        }
    }

    pub fn get_connect_vertices_of_v(&self, src_v_id: i32) -> BTreeSet<i32> {
        match self.vertex_connect_edges.get(&src_v_id) {
            Some(connections) => {
                let mut connect_vertices = BTreeSet::new();
                for (edge_id, dir) in connections {
                    let possible_edges = self.edge_connect_vertices.get(edge_id).unwrap();
                    for (start_v_id, end_v_id) in possible_edges {
                        match *dir {
                            Direction::Out => {
                                connect_vertices.insert(*end_v_id);
                            }
                            Direction::Incoming => {
                                connect_vertices.insert(*start_v_id);
                            }
                        }
                    }
                }
                connect_vertices
            }
            None => BTreeSet::new(),
        }
    }

    pub fn get_connect_edges_of_v(&self, src_v_id: i32) -> Vec<(i32, Direction)> {
        match self.vertex_connect_edges.get(&src_v_id) {
            Some(connections) => {
                let mut connections_vec = Vec::new();
                for (edge_id, dir) in connections {
                    connections_vec.push((*edge_id, *dir));
                }
                connections_vec
            }
            None => Vec::new(),
        }
    }

    pub fn get_edges_between_vertices(&self, src_v_id: i32, dst_v_id: i32) -> Vec<(i32, Direction)> {
        match self
            .vertex_vertex_edges
            .get(&(src_v_id, dst_v_id))
        {
            Some(edges) => edges.clone(),
            None => Vec::new(),
        }
    }
}

impl From<Schema> for PatternMeta {
    fn from(src_schema: Schema) -> PatternMeta {
        let (table_map, relation_labels) = src_schema.get_pattern_schema_info();
        let mut pattern_meta = PatternMeta {
            vertex_map: BTreeMap::new(),
            edge_map: BTreeMap::new(),
            vertex_connect_edges: BTreeMap::new(),
            edge_connect_vertices: BTreeMap::new(),
            vertex_vertex_edges: BTreeMap::new(),
        };
        for (name, id) in &table_map {
            match relation_labels.get(name) {
                Some(connections) => {
                    pattern_meta.edge_map.insert(name.clone(), *id);
                    for (start_v_meta, end_v_meta) in connections {
                        let start_v_name = start_v_meta.get_name();
                        let end_v_name = end_v_meta.get_name();
                        let start_v_id = src_schema.get_table_id(&start_v_name).unwrap();
                        let end_v_id = src_schema.get_table_id(&end_v_name).unwrap();
                        pattern_meta
                            .vertex_connect_edges
                            .entry(start_v_id)
                            .or_insert(Vec::new())
                            .push((*id, Direction::Out));
                        pattern_meta
                            .vertex_connect_edges
                            .entry(end_v_id)
                            .or_insert(Vec::new())
                            .push((*id, Direction::Incoming));
                        pattern_meta
                            .edge_connect_vertices
                            .entry(*id)
                            .or_insert(Vec::new())
                            .push((start_v_id, end_v_id));
                        pattern_meta
                            .vertex_vertex_edges
                            .entry((start_v_id, end_v_id))
                            .or_insert(Vec::new())
                            .push((*id, Direction::Out));
                        pattern_meta
                            .vertex_vertex_edges
                            .entry((end_v_id, start_v_id))
                            .or_insert(Vec::new())
                            .push((*id, Direction::Incoming));
                    }
                }
                None => {
                    pattern_meta
                        .vertex_map
                        .insert(name.clone(), *id);
                }
            }
        }
        pattern_meta
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use ir_core::{plan::meta::Schema, JsonIO};

    use super::PatternMeta;

    fn read_modern_graph_schema() -> Schema {
        let modern_schema_file = File::open("resource/modern_schema.json").unwrap();
        Schema::from_json(modern_schema_file).unwrap()
    }

    #[test]
    fn test_modern_graph_schema() {
        let modern_graph_schema = read_modern_graph_schema();
        let modern_pattern_meta = PatternMeta::from(modern_graph_schema);
        assert_eq!(modern_pattern_meta.get_edge_num(), 2);
        assert_eq!(modern_pattern_meta.get_vertex_num(), 2);
        assert_eq!(
            modern_pattern_meta
                .get_connect_edges_of_v(0)
                .len(),
            3
        );
        assert_eq!(
            modern_pattern_meta
                .get_connect_edges_of_v(1)
                .len(),
            1
        );
        assert_eq!(
            modern_pattern_meta
                .get_connect_vertices_of_v(0)
                .len(),
            2
        );
        assert_eq!(
            modern_pattern_meta
                .get_connect_vertices_of_v(1)
                .len(),
            1
        );
        assert_eq!(
            modern_pattern_meta
                .get_edges_between_vertices(0, 0)
                .len(),
            2
        );
        assert_eq!(
            modern_pattern_meta
                .get_edges_between_vertices(0, 1)
                .len(),
            1
        );
        assert_eq!(
            modern_pattern_meta
                .get_edges_between_vertices(1, 0)
                .len(),
            1
        );
    }
}
