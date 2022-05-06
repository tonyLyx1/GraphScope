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

use crate::{Direction, LabelID};

#[derive(Debug)]
pub struct PatternMeta {
    /// Key: vertex label name, Value: vertex labal id
    vertex_map: BTreeMap<String, LabelID>,
    /// Key: edge label name, Value: edge label id
    edge_map: BTreeMap<String, LabelID>,
    /// Key: vertex label id, Value: BTreeSet<(edge label id, direction)>
    vertex_connect_edges: BTreeMap<LabelID, BTreeSet<(LabelID, Direction)>>,
    /// Key: edge label id, Value: Vec<(src vertex label id, dst vertex label id)>
    edge_connect_vertices: BTreeMap<LabelID, Vec<(LabelID, LabelID)>>,
    /// Key: (src vertex label id, dst vertex label id), Value: Vec<(edge label id, direction)>
    vertex_vertex_edges: BTreeMap<(LabelID, LabelID), Vec<(LabelID, Direction)>>,
}

/// Initializer of PatternMeta
impl From<Schema> for PatternMeta {
    /// Pick necessary info from schema and reorganize them into the generated PatternMeta
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
                // Case that this is an edge label
                Some(connections) => {
                    pattern_meta.edge_map.insert(name.clone(), *id);
                    for (start_v_meta, end_v_meta) in connections {
                        let start_v_name = start_v_meta.get_name();
                        let end_v_name = end_v_meta.get_name();
                        let start_v_id = src_schema.get_table_id(&start_v_name).unwrap();
                        let end_v_id = src_schema.get_table_id(&end_v_name).unwrap();
                        // Update connect information
                        pattern_meta
                            .vertex_connect_edges
                            .entry(start_v_id)
                            .or_insert(BTreeSet::new())
                            .insert((*id, Direction::Out));
                        pattern_meta
                            .vertex_connect_edges
                            .entry(end_v_id)
                            .or_insert(BTreeSet::new())
                            .insert((*id, Direction::In));
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
                            .push((*id, Direction::In));
                    }
                }
                // Case that this is an vertex label
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

/// Methods for access some fields of PatternMeta or get some info from PatternMeta
impl PatternMeta {
    pub fn get_vertex_num(&self) -> usize {
        self.vertex_map.len()
    }

    pub fn get_edge_num(&self) -> usize {
        self.edge_map.len()
    }

    pub fn get_all_vertex_label_names(&self) -> Vec<String> {
        self.vertex_map
            .iter()
            .map(|(vertex_name, _)| vertex_name.clone())
            .collect()
    }

    pub fn get_all_edge_label_names(&self) -> Vec<String> {
        self.edge_map
            .iter()
            .map(|(edge_name, _)| edge_name.clone())
            .collect()
    }

    pub fn get_all_vertex_label_ids(&self) -> Vec<LabelID> {
        self.vertex_map
            .iter()
            .map(|(_, vertex_id)| *vertex_id)
            .collect()
    }

    pub fn get_all_edge_label_ids(&self) -> Vec<LabelID> {
        self.edge_map
            .iter()
            .map(|(_, edge_id)| *edge_id)
            .collect()
    }

    pub fn get_vertex_id(&self, label_name: &str) -> Option<LabelID> {
        self.vertex_map.get(label_name).cloned()
    }

    pub fn get_edge_id(&self, label_name: &str) -> Option<LabelID> {
        self.edge_map.get(label_name).cloned()
    }

    /// Given a soruce vertex label, find all its neighboring connect vertices(label)
    pub fn get_connect_vertices_of_v(&self, src_v_label: LabelID) -> BTreeSet<LabelID> {
        match self.vertex_connect_edges.get(&src_v_label) {
            Some(connections) => {
                let mut connect_vertices = BTreeSet::new();
                for (edge_id, dir) in connections {
                    let possible_edges = self.edge_connect_vertices.get(edge_id).unwrap();
                    for (start_v_id, end_v_id) in possible_edges {
                        if *start_v_id == src_v_label && *dir == Direction::Out {
                            connect_vertices.insert(*end_v_id);
                        }
                        if *end_v_id == src_v_label && *dir == Direction::In {
                            connect_vertices.insert(*start_v_id);
                        }
                    }
                }

                connect_vertices
            }
            None => BTreeSet::new(),
        }
    }

    /// Given a source vertex label, find all its neiboring connected edges(label)
    pub fn get_connect_edges_of_v(&self, src_v_label: LabelID) -> Vec<(LabelID, Direction)> {
        match self.vertex_connect_edges.get(&src_v_label) {
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

    /// Given a source edge label, find all possible pairs of its (src vertex label, dst vertex label)
    pub fn get_connect_vertices_of_e(&self, src_e_label: LabelID) -> Vec<(LabelID, LabelID)> {
        match self.edge_connect_vertices.get(&src_e_label) {
            Some(connections) => connections.clone(),
            None => Vec::new(),
        }
    }

    /// Given a src vertex label and a dst vertex label, find all possible edges(label) between them with directions
    pub fn get_edges_between_vertices(
        &self, src_v_label: LabelID, dst_v_label: LabelID,
    ) -> Vec<(LabelID, Direction)> {
        match self
            .vertex_vertex_edges
            .get(&(src_v_label, dst_v_label))
        {
            Some(edges) => edges.clone(),
            None => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::PatternMeta;
    use crate::test_cases::*;
    use crate::Direction;

    /// Test whether the pattern meta from the modern graph obeys our expectation
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

    /// Test whether the pattern meta from the ldbc graph obeys our expectation
    #[test]
    fn test_ldbc_graph_schema() {
        let ldbc_graph_schema = read_ldbc_graph_schema();
        let ldbc_pattern_meta = PatternMeta::from(ldbc_graph_schema.clone());
        assert_eq!(
            ldbc_pattern_meta.get_all_edge_label_ids().len()
                + ldbc_pattern_meta
                    .get_all_vertex_label_ids()
                    .len(),
            ldbc_graph_schema
                .get_pattern_schema_info()
                .0
                .len()
        );
        let all_vertex_names = ldbc_pattern_meta.get_all_vertex_label_names();
        for vertex_name in &all_vertex_names {
            let v_id_from_schema = ldbc_graph_schema
                .get_table_id(vertex_name)
                .unwrap();
            let v_id_from_pattern_meta = ldbc_pattern_meta
                .get_vertex_id(vertex_name)
                .unwrap();
            assert_eq!(v_id_from_schema, v_id_from_pattern_meta);
        }
        let all_edge_names = ldbc_pattern_meta.get_all_edge_label_names();
        for edge_name in &all_edge_names {
            let e_id_from_schema = ldbc_graph_schema
                .get_table_id(edge_name)
                .unwrap();
            let e_id_from_pattern_meta = ldbc_pattern_meta
                .get_edge_id(edge_name)
                .unwrap();
            assert_eq!(e_id_from_schema, e_id_from_pattern_meta);
        }
        let all_edge_ids = ldbc_pattern_meta.get_all_edge_label_ids();
        let mut vertex_vertex_edges = BTreeMap::new();
        for edge_id in all_edge_ids {
            let edge_connect_vertices = ldbc_pattern_meta.get_connect_vertices_of_e(edge_id);
            for (start_v_id, end_v_id) in edge_connect_vertices {
                vertex_vertex_edges
                    .entry((start_v_id, end_v_id))
                    .or_insert(Vec::new())
                    .push((edge_id, Direction::Out));
                vertex_vertex_edges
                    .entry((end_v_id, start_v_id))
                    .or_insert(Vec::new())
                    .push((edge_id, Direction::In));
            }
        }
        for ((start_v_id, end_v_id), mut connections) in vertex_vertex_edges {
            let mut edges_between_vertices =
                ldbc_pattern_meta.get_edges_between_vertices(start_v_id, end_v_id);
            assert_eq!(connections.len(), edges_between_vertices.len());
            connections.sort();
            edges_between_vertices.sort();
            for i in 0..connections.len() {
                assert_eq!(connections[i], edges_between_vertices[i]);
            }
        }
        let all_vertex_ids = ldbc_pattern_meta.get_all_vertex_label_ids();
        let mut vertex_vertex_edges = BTreeMap::new();
        for vertex_id in all_vertex_ids {
            let connect_edges = ldbc_pattern_meta.get_connect_edges_of_v(vertex_id);
            for (edge_id, dir) in connect_edges {
                let edge_connections = ldbc_pattern_meta.get_connect_vertices_of_e(edge_id);
                for (start_v_id, end_v_id) in edge_connections {
                    if start_v_id == vertex_id && dir == Direction::Out {
                        vertex_vertex_edges
                            .entry((start_v_id, end_v_id))
                            .or_insert(Vec::new())
                            .push((edge_id, Direction::Out));
                    }
                    if end_v_id == vertex_id && dir == Direction::In {
                        vertex_vertex_edges
                            .entry((end_v_id, start_v_id))
                            .or_insert(Vec::new())
                            .push((edge_id, Direction::In));
                    }
                }
            }
        }
        for ((start_v_id, end_v_id), mut connections) in vertex_vertex_edges {
            let mut edges_between_vertices =
                ldbc_pattern_meta.get_edges_between_vertices(start_v_id, end_v_id);
            assert_eq!(connections.len(), edges_between_vertices.len());
            connections.sort();
            edges_between_vertices.sort();
            for i in 0..connections.len() {
                assert_eq!(connections[i], edges_between_vertices[i]);
            }
        }
    }
}
