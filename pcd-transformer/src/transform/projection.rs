use std::sync::Arc;

use pcd_core::pointcloud::point::{Point, PointCloud};
use projection_transform::{
    crs::*, jprect::JPRZone, proj_transform::ProjTransformShared, vshift::Jgd2011ToWgs84,
};

use super::Transform;

/// Coordinate transformation strategy
enum TransformStrategy {
    /// Japan-specific optimized transformation (JGD2011 -> WGS84)
    JapanOptimized {
        jgd2wgs: Arc<Jgd2011ToWgs84>,
        input_is_rectangular: bool,
    },
    /// PROJ-based transformation for global coordinate systems
    ProjBased { proj_transform: ProjTransformShared },
}

pub struct ProjectionTransform {
    strategy: TransformStrategy,
    output_epsg: EpsgCode,
}

impl Transform for ProjectionTransform {
    fn transform(&self, point_cloud: PointCloud) -> PointCloud {
        let input_epsg = point_cloud.metadata.epsg;

        match &self.strategy {
            TransformStrategy::JapanOptimized {
                jgd2wgs,
                input_is_rectangular,
            } => self.transform_japan_optimized(point_cloud, jgd2wgs, *input_is_rectangular),
            TransformStrategy::ProjBased { proj_transform } => {
                self.transform_proj_based(point_cloud, proj_transform)
            }
        }
    }
}

impl ProjectionTransform {
    /// Create a new projection transformer with automatic strategy selection
    ///
    /// # Arguments
    /// * `input_epsg` - Input coordinate system EPSG code
    /// * `output_epsg` - Output coordinate system EPSG code
    ///
    /// # Returns
    /// * `Ok(ProjectionTransform)` - Successfully created transformer
    /// * `Err(String)` - Failed to create transformation
    pub fn new_auto(input_epsg: EpsgCode, output_epsg: EpsgCode) -> Result<Self, String> {
        // Check if this is a Japan-specific transformation that can use optimized path
        if is_japan_crs(input_epsg) && output_epsg == EPSG_WGS84_GEOGRAPHIC_3D {
            let jgd2wgs = Arc::new(Jgd2011ToWgs84::default());
            let input_is_rectangular = is_jgd2011_rectangular(input_epsg);

            Ok(Self {
                strategy: TransformStrategy::JapanOptimized {
                    jgd2wgs,
                    input_is_rectangular,
                },
                output_epsg,
            })
        } else {
            // Use PROJ for all other transformations
            let proj_transform = ProjTransformShared::new(input_epsg, output_epsg)
                .map_err(|e| format!("Failed to create PROJ transformer: {}", e))?;

            Ok(Self {
                strategy: TransformStrategy::ProjBased { proj_transform },
                output_epsg,
            })
        }
    }

    /// Legacy constructor for Japan-specific transformations
    pub fn new(jgd2wgs: Arc<Jgd2011ToWgs84>, output_epsg: EpsgCode) -> Self {
        Self {
            strategy: TransformStrategy::JapanOptimized {
                jgd2wgs,
                input_is_rectangular: true,
            },
            output_epsg,
        }
    }

    /// Transform using Japan-optimized path (legacy implementation)
    fn transform_japan_optimized(
        &self,
        point_cloud: PointCloud,
        jgd2wgs: &Arc<Jgd2011ToWgs84>,
        input_is_rectangular: bool,
    ) -> PointCloud {
        let input_epsg = point_cloud.metadata.epsg;
        let mut points = vec![];

        for (x, y, z, point) in point_cloud.iter() {
            let (lng, lat, height) = if input_is_rectangular {
                Self::rectangular_to_lnglat(x, y, z, input_epsg)
            } else {
                (x, y, z)
            };

            let (lng, lat, height) = jgd2wgs.convert(lng, lat, height);

            points.push(Point {
                x: lng,
                y: lat,
                z: height,
                color: point.color.clone(),
                attributes: point.attributes.clone(),
            });
        }

        PointCloud::new(points, self.output_epsg)
    }

    /// Transform using PROJ library (global coordinate systems)
    fn transform_proj_based(
        &self,
        point_cloud: PointCloud,
        proj_transform: &ProjTransformShared,
    ) -> PointCloud {
        let mut points = vec![];

        for (x, y, z, point) in point_cloud.iter() {
            // Transform coordinates using PROJ
            let (new_x, new_y, new_z) = proj_transform
                .transform(x, y, z)
                .expect("PROJ transformation failed");

            points.push(Point {
                x: new_x,
                y: new_y,
                z: new_z,
                color: point.color.clone(),
                attributes: point.attributes.clone(),
            });
        }

        PointCloud::new(points, self.output_epsg)
    }

    fn rectangular_to_lnglat(x: f64, y: f64, height: f64, input_epsg: EpsgCode) -> (f64, f64, f64) {
        let zone = JPRZone::from_epsg(input_epsg).unwrap();
        let proj = zone.projection();
        let (lng, lat, height) = proj.project_inverse(x, y, height).unwrap();
        (lng, lat, height)
    }
}

/// Check if EPSG code is a JGD2011 rectangular coordinate system
fn is_jgd2011_rectangular(epsg: EpsgCode) -> bool {
    matches!(
        epsg,
        EPSG_JGD2011_JPRECT_I..=EPSG_JGD2011_JPRECT_XIX
            | EPSG_JGD2011_JPRECT_I_JGD2011_HEIGHT..=EPSG_JGD2011_JPRECT_XIII_JGD2011_HEIGHT
    )
}
