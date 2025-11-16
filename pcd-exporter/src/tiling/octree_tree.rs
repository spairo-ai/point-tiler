/// Octree-based tile tree structure for 3D Tiles
///
/// This module provides an octree data structure where each node can have
/// up to 8 children, representing spatial subdivision in 3D space.

use cesiumtiles::tileset;
use super::octree_scheme::{BoundingBox, OctreeNodeId, octree_geometric_error};

#[derive(Debug)]
pub struct OctreeContent {
    pub node_id: OctreeNodeId,
    pub content_path: String,
    pub bbox: BoundingBox,
}

impl Default for OctreeContent {
    fn default() -> Self {
        OctreeContent {
            node_id: OctreeNodeId::root(),
            content_path: String::new(),
            bbox: BoundingBox::new(
                f64::MAX,
                f64::MAX,
                f64::MAX,
                f64::MIN,
                f64::MIN,
                f64::MIN,
            ),
        }
    }
}

#[derive(Debug)]
pub struct OctreeNode {
    node_id: OctreeNodeId,
    contents: Vec<OctreeContent>,
    // 8 children for octree (indexed by octant 0-7)
    children: [Option<Box<OctreeNode>>; 8],
    bbox: BoundingBox,
}

impl OctreeNode {
    fn new(node_id: OctreeNodeId, bbox: BoundingBox) -> Self {
        OctreeNode {
            node_id,
            contents: vec![],
            children: [None, None, None, None, None, None, None, None],
            bbox,
        }
    }

    fn update_boundary(&mut self) {
        // Reset bbox to empty
        self.bbox = BoundingBox::new(
            f64::MAX,
            f64::MAX,
            f64::MAX,
            f64::MIN,
            f64::MIN,
            f64::MIN,
        );

        // Expand bbox to include all children
        for child_opt in &mut self.children {
            if let Some(child) = child_opt {
                child.update_boundary();
                self.bbox.merge(&child.bbox);
            }
        }

        // Expand bbox to include all contents
        for content in &self.contents {
            self.bbox.merge(&content.bbox);
        }
    }

    fn into_tileset_tile(mut self) -> tileset::Tile {
        self.update_boundary();

        let children = {
            let children: Vec<_> = self
                .children
                .into_iter()
                .flatten()
                .map(|child| (*child).into_tileset_tile())
                .collect();
            if children.is_empty() {
                None
            } else {
                Some(children)
            }
        };

        let (content, contents) = {
            match self.contents.len() {
                0 => (None, None),
                1 => {
                    let content = tileset::Content {
                        uri: self.contents[0].content_path.clone(),
                        ..Default::default()
                    };
                    (Some(content), None)
                }
                _ => {
                    let contents: Vec<_> = self
                        .contents
                        .into_iter()
                        .map(|content| tileset::Content {
                            uri: content.content_path,
                            ..Default::default()
                        })
                        .collect();
                    (None, Some(contents))
                }
            }
        };

        let geometric_error_value = octree_geometric_error(&self.bbox);

        tileset::Tile {
            geometric_error: geometric_error_value,
            refine: Some(tileset::Refine::Replace),
            bounding_volume: tileset::BoundingVolume::new_region([
                self.bbox.min_x.to_radians(),
                self.bbox.min_y.to_radians(),
                self.bbox.max_x.to_radians(),
                self.bbox.max_y.to_radians(),
                self.bbox.min_z,
                self.bbox.max_z,
            ]),
            content,
            contents,
            children,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct OctreeTree {
    root: OctreeNode,
    root_bbox: BoundingBox,
}

impl OctreeTree {
    pub fn new(root_bbox: BoundingBox) -> Self {
        Self {
            root: OctreeNode::new(OctreeNodeId::root(), root_bbox),
            root_bbox,
        }
    }

    pub fn into_tileset_root(self) -> tileset::Tile {
        self.root.into_tileset_tile()
    }

    pub fn add_content(&mut self, content: OctreeContent) {
        let node = self.get_or_create_node(content.node_id);
        node.contents.push(content);
    }

    fn get_or_create_node(&mut self, node_id: OctreeNodeId) -> &mut OctreeNode {
        if node_id.depth() == 0 {
            return &mut self.root;
        }

        // Build path from root to target node
        let mut path = Vec::new();
        let mut current_id = node_id;
        while current_id.depth() > 0 {
            path.push(current_id.octant_in_parent().unwrap());
            current_id = current_id.parent().unwrap();
        }
        path.reverse();

        // Traverse and create nodes as needed
        let mut current_node = &mut self.root;
        let mut current_id = OctreeNodeId::root();
        let mut current_bbox = self.root_bbox;

        for &octant in &path {
            current_id = current_id.child(octant);
            let octants = super::octree_scheme::subdivide_bbox(&current_bbox);
            current_bbox = octants[octant as usize];

            current_node = current_node.children[octant as usize].get_or_insert_with(|| {
                Box::new(OctreeNode::new(current_id, current_bbox))
            });
        }

        current_node
    }
}

impl Default for OctreeTree {
    fn default() -> Self {
        Self::new(BoundingBox::world())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_octree_node_creation() {
        let bbox = BoundingBox::new(0.0, 0.0, 0.0, 10.0, 10.0, 10.0);
        let node = OctreeNode::new(OctreeNodeId::root(), bbox);
        assert_eq!(node.node_id, OctreeNodeId::root());
        assert_eq!(node.contents.len(), 0);
    }

    #[test]
    fn test_octree_tree_add_content() {
        let bbox = BoundingBox::new(0.0, 0.0, 0.0, 100.0, 100.0, 100.0);
        let mut tree = OctreeTree::new(bbox);

        let content = OctreeContent {
            node_id: OctreeNodeId::root(),
            content_path: "test.glb".to_string(),
            bbox: BoundingBox::new(0.0, 0.0, 0.0, 10.0, 10.0, 10.0),
        };

        tree.add_content(content);
        assert_eq!(tree.root.contents.len(), 1);
    }

    #[test]
    fn test_octree_tree_nested_nodes() {
        let bbox = BoundingBox::new(0.0, 0.0, 0.0, 100.0, 100.0, 100.0);
        let mut tree = OctreeTree::new(bbox);

        let child_id = OctreeNodeId::root().child(3);
        let content = OctreeContent {
            node_id: child_id,
            content_path: "child.glb".to_string(),
            bbox: BoundingBox::new(50.0, 50.0, 0.0, 100.0, 100.0, 50.0),
        };

        tree.add_content(content);

        // Verify child was created
        assert!(tree.root.children[3].is_some());
    }
}
