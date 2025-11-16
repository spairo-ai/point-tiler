/// Octree-based tiling scheme for 3D point cloud data
///
/// Unlike the geographic tiling scheme which uses 2D lat/lng grid,
/// this scheme recursively subdivides 3D space into octants.

use std::fmt;

/// 3D Axis-Aligned Bounding Box
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
}

impl BoundingBox {
    pub fn new(min_x: f64, min_y: f64, min_z: f64, max_x: f64, max_y: f64, max_z: f64) -> Self {
        BoundingBox {
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
        }
    }

    /// Create a bounding box that encompasses the entire world (WGS84 coordinates)
    pub fn world() -> Self {
        BoundingBox::new(-180.0, -90.0, -1000.0, 180.0, 90.0, 10000.0)
    }

    /// Get the center point of the bounding box
    pub fn center(&self) -> (f64, f64, f64) {
        (
            (self.min_x + self.max_x) / 2.0,
            (self.min_y + self.max_y) / 2.0,
            (self.min_z + self.max_z) / 2.0,
        )
    }

    /// Get the size of the bounding box in each dimension
    pub fn size(&self) -> (f64, f64, f64) {
        (
            self.max_x - self.min_x,
            self.max_y - self.min_y,
            self.max_z - self.min_z,
        )
    }

    /// Calculate the diagonal length of the bounding box
    pub fn diagonal(&self) -> f64 {
        let (sx, sy, sz) = self.size();
        (sx * sx + sy * sy + sz * sz).sqrt()
    }

    /// Check if a point is inside this bounding box
    pub fn contains(&self, x: f64, y: f64, z: f64) -> bool {
        x >= self.min_x
            && x <= self.max_x
            && y >= self.min_y
            && y <= self.max_y
            && z >= self.min_z
            && z <= self.max_z
    }

    /// Expand the bounding box to include a point
    pub fn expand_to_include(&mut self, x: f64, y: f64, z: f64) {
        self.min_x = self.min_x.min(x);
        self.min_y = self.min_y.min(y);
        self.min_z = self.min_z.min(z);
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
        self.max_z = self.max_z.max(z);
    }

    /// Merge with another bounding box
    pub fn merge(&mut self, other: &BoundingBox) {
        self.min_x = self.min_x.min(other.min_x);
        self.min_y = self.min_y.min(other.min_y);
        self.min_z = self.min_z.min(other.min_z);
        self.max_x = self.max_x.max(other.max_x);
        self.max_y = self.max_y.max(other.max_y);
        self.max_z = self.max_z.max(other.max_z);
    }
}

/// Octant index (0-7) representing the 8 subdivisions of a bounding box
/// Encoding: bit 0 = x split, bit 1 = y split, bit 2 = z split
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OctantIndex(pub u8);

impl OctantIndex {
    /// Create an octant index from x, y, z binary flags
    pub fn from_xyz(x_high: bool, y_high: bool, z_high: bool) -> Self {
        OctantIndex((x_high as u8) | ((y_high as u8) << 1) | ((z_high as u8) << 2))
    }

    /// Determine which octant a point belongs to within a bounding box
    pub fn from_point(bbox: &BoundingBox, x: f64, y: f64, z: f64) -> Self {
        let (cx, cy, cz) = bbox.center();
        OctantIndex::from_xyz(x >= cx, y >= cy, z >= cz)
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

impl fmt::Display for OctantIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Octree node identifier using Morton code (Z-order curve)
/// Format: depth (4 bits) + morton code (60 bits)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OctreeNodeId(pub u64);

impl OctreeNodeId {
    /// Root node (depth 0)
    pub fn root() -> Self {
        OctreeNodeId(0)
    }

    /// Create a node ID from depth and octant path
    pub fn new(depth: u8, octant_path: &[u8]) -> Self {
        assert!(depth <= 15, "Maximum depth is 15");
        assert_eq!(depth as usize, octant_path.len());

        let mut morton: u64 = 0;
        for (i, &octant) in octant_path.iter().enumerate() {
            assert!(octant < 8, "Octant index must be 0-7");
            morton |= (octant as u64) << (i * 3);
        }

        OctreeNodeId((depth as u64) << 60 | morton)
    }

    /// Get the depth of this node
    pub fn depth(&self) -> u8 {
        (self.0 >> 60) as u8
    }

    /// Get the morton code of this node
    pub fn morton_code(&self) -> u64 {
        self.0 & 0x0FFFFFFFFFFFFFFF
    }

    /// Get the parent node ID
    pub fn parent(&self) -> Option<OctreeNodeId> {
        let depth = self.depth();
        if depth == 0 {
            None
        } else {
            let parent_depth = depth - 1;
            let morton = self.morton_code() >> 3; // Remove last 3 bits
            Some(OctreeNodeId((parent_depth as u64) << 60 | morton))
        }
    }

    /// Get the child node ID for a given octant
    pub fn child(&self, octant: u8) -> OctreeNodeId {
        assert!(octant < 8, "Octant index must be 0-7");
        let depth = self.depth();
        assert!(depth < 15, "Maximum depth exceeded");

        let child_depth = depth + 1;
        let morton = (self.morton_code() << 3) | (octant as u64);
        OctreeNodeId((child_depth as u64) << 60 | morton)
    }

    /// Get the octant index within the parent
    pub fn octant_in_parent(&self) -> Option<u8> {
        if self.depth() == 0 {
            None
        } else {
            Some((self.morton_code() & 0x7) as u8)
        }
    }

    /// Encode as (depth, morton_code) for compatibility
    pub fn to_tuple(&self) -> (u8, u64) {
        (self.depth(), self.morton_code())
    }
}

impl fmt::Display for OctreeNodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "d{}_m{:016x}", self.depth(), self.morton_code())
    }
}

/// Subdivide a bounding box into 8 octants
pub fn subdivide_bbox(bbox: &BoundingBox) -> [BoundingBox; 8] {
    let (cx, cy, cz) = bbox.center();

    [
        // Octant 0: x_low, y_low, z_low
        BoundingBox::new(bbox.min_x, bbox.min_y, bbox.min_z, cx, cy, cz),
        // Octant 1: x_high, y_low, z_low
        BoundingBox::new(cx, bbox.min_y, bbox.min_z, bbox.max_x, cy, cz),
        // Octant 2: x_low, y_high, z_low
        BoundingBox::new(bbox.min_x, cy, bbox.min_z, cx, bbox.max_y, cz),
        // Octant 3: x_high, y_high, z_low
        BoundingBox::new(cx, cy, bbox.min_z, bbox.max_x, bbox.max_y, cz),
        // Octant 4: x_low, y_low, z_high
        BoundingBox::new(bbox.min_x, bbox.min_y, cz, cx, cy, bbox.max_z),
        // Octant 5: x_high, y_low, z_high
        BoundingBox::new(cx, bbox.min_y, cz, bbox.max_x, cy, bbox.max_z),
        // Octant 6: x_low, y_high, z_high
        BoundingBox::new(bbox.min_x, cy, cz, cx, bbox.max_y, bbox.max_z),
        // Octant 7: x_high, y_high, z_high
        BoundingBox::new(cx, cy, cz, bbox.max_x, bbox.max_y, bbox.max_z),
    ]
}

/// Get the bounding box for a specific octree node
pub fn get_node_bbox(root_bbox: &BoundingBox, node_id: OctreeNodeId) -> BoundingBox {
    let depth = node_id.depth();
    if depth == 0 {
        return *root_bbox;
    }

    let mut bbox = *root_bbox;
    let morton = node_id.morton_code();

    for level in 0..depth {
        let octant = ((morton >> (level * 3)) & 0x7) as u8;
        let octants = subdivide_bbox(&bbox);
        bbox = octants[octant as usize];
    }

    bbox
}

/// Calculate geometric error for an octree node
/// Error is proportional to the diagonal size of the bounding box
pub fn octree_geometric_error(bbox: &BoundingBox) -> f64 {
    bbox.diagonal() * 0.5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box_center() {
        let bbox = BoundingBox::new(0.0, 0.0, 0.0, 10.0, 10.0, 10.0);
        assert_eq!(bbox.center(), (5.0, 5.0, 5.0));
    }

    #[test]
    fn test_bounding_box_contains() {
        let bbox = BoundingBox::new(0.0, 0.0, 0.0, 10.0, 10.0, 10.0);
        assert!(bbox.contains(5.0, 5.0, 5.0));
        assert!(bbox.contains(0.0, 0.0, 0.0));
        assert!(bbox.contains(10.0, 10.0, 10.0));
        assert!(!bbox.contains(-1.0, 5.0, 5.0));
        assert!(!bbox.contains(11.0, 5.0, 5.0));
    }

    #[test]
    fn test_octant_index() {
        let bbox = BoundingBox::new(0.0, 0.0, 0.0, 10.0, 10.0, 10.0);
        assert_eq!(OctantIndex::from_point(&bbox, 2.0, 2.0, 2.0), OctantIndex(0));
        assert_eq!(OctantIndex::from_point(&bbox, 7.0, 2.0, 2.0), OctantIndex(1));
        assert_eq!(OctantIndex::from_point(&bbox, 2.0, 7.0, 2.0), OctantIndex(2));
        assert_eq!(OctantIndex::from_point(&bbox, 7.0, 7.0, 2.0), OctantIndex(3));
        assert_eq!(OctantIndex::from_point(&bbox, 2.0, 2.0, 7.0), OctantIndex(4));
        assert_eq!(OctantIndex::from_point(&bbox, 7.0, 2.0, 7.0), OctantIndex(5));
        assert_eq!(OctantIndex::from_point(&bbox, 2.0, 7.0, 7.0), OctantIndex(6));
        assert_eq!(OctantIndex::from_point(&bbox, 7.0, 7.0, 7.0), OctantIndex(7));
    }

    #[test]
    fn test_subdivide_bbox() {
        let bbox = BoundingBox::new(0.0, 0.0, 0.0, 10.0, 10.0, 10.0);
        let octants = subdivide_bbox(&bbox);

        assert_eq!(octants.len(), 8);
        assert_eq!(octants[0], BoundingBox::new(0.0, 0.0, 0.0, 5.0, 5.0, 5.0));
        assert_eq!(octants[7], BoundingBox::new(5.0, 5.0, 5.0, 10.0, 10.0, 10.0));
    }

    #[test]
    fn test_octree_node_id() {
        let root = OctreeNodeId::root();
        assert_eq!(root.depth(), 0);
        assert_eq!(root.parent(), None);

        let child = root.child(3);
        assert_eq!(child.depth(), 1);
        assert_eq!(child.parent(), Some(root));
        assert_eq!(child.octant_in_parent(), Some(3));

        let grandchild = child.child(5);
        assert_eq!(grandchild.depth(), 2);
        assert_eq!(grandchild.parent(), Some(child));
    }

    #[test]
    fn test_node_id_from_path() {
        let path = vec![3, 5, 2];
        let node_id = OctreeNodeId::new(3, &path);
        assert_eq!(node_id.depth(), 3);

        let mut current = node_id;
        for i in (0..3).rev() {
            assert_eq!(current.octant_in_parent(), Some(path[i]));
            current = current.parent().unwrap();
        }
    }

    #[test]
    fn test_get_node_bbox() {
        let root_bbox = BoundingBox::new(0.0, 0.0, 0.0, 16.0, 16.0, 16.0);

        let root_id = OctreeNodeId::root();
        assert_eq!(get_node_bbox(&root_bbox, root_id), root_bbox);

        let child_id = root_id.child(7); // Top-right-back octant
        let child_bbox = get_node_bbox(&root_bbox, child_id);
        assert_eq!(child_bbox, BoundingBox::new(8.0, 8.0, 8.0, 16.0, 16.0, 16.0));
    }
}
