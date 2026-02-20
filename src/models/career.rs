use std::collections::HashMap;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub enum InteractionState {
    Idle,
    Dragging,
    Zooming,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CareerNode {
    pub id: i32,
    pub title: String,
    pub year: i32,
    pub parent_id: Option<i32>,
    pub children: IndexMap<i32, CareerNode>,
    pub technology_name: Option<String>,
    pub technology_category_title: Option<String>,
    pub logo_path: Option<String>,
    pub logo_name: Option<String>,
}
#[derive(Clone, Deserialize, Serialize, Default)]
pub struct CareerNodeTree {
    pub roots: HashMap<i32, CareerNode>,
}

impl CareerNodeTree {
    pub fn build_from_flat_data(nodes: Vec<CareerNode>) -> Self {
        let mut node_map: HashMap<i32, CareerNode> =
            nodes.into_iter().map(|node| (node.id, node)).collect();

        let root_id = node_map
            .values()
            .find(|node| node.parent_id.is_none())
            .expect("No root node found. A node without parent_id is required.")
            .id;

        let mut root = node_map
            .remove(&root_id)
            .expect("Unable to find the root after extraction.");

        let mut unprocessed = Vec::new();
        while !node_map.is_empty() {
            for (id, node) in node_map.drain() {
                if let Some(parent_id) = node.parent_id {
                    if let Some(parent_node) = Self::find_node_mut(&mut root, parent_id) {
                        parent_node.children.insert(node.id, node);
                    } else {
                        unprocessed.push((id, node));
                    }
                }
            }

            for (id, node) in unprocessed.drain(..) {
                node_map.insert(id, node);
            }
        }

        CareerNodeTree {
            roots: HashMap::from([(root_id, root)]),
        }
    }

    fn find_node_mut(node: &mut CareerNode, id: i32) -> Option<&mut CareerNode> {
        if node.id == id {
            return Some(node);
        }
        for child in node.children.values_mut() {
            if let Some(found) = Self::find_node_mut(child, id) {
                return Some(found);
            }
        }
        None
    }
}

#[cfg(feature = "ssr")]
impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for CareerNode {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> sqlx::Result<Self> {
        use sqlx::Row;

        Ok(Self {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
            year: row.try_get("year")?,
            parent_id: row.try_get("parent_id")?,
            technology_name: row.try_get("technology_name")?,
            technology_category_title: row.try_get("technology_category_title")?,
            logo_path: row.try_get("logo_path")?,
            logo_name: row.try_get("logo_name")?,
            children: IndexMap::new(),
        })
    }
}


