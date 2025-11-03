use std::sync::Arc;

use pcd_core::pointcloud::point::{Point, PointCloud};
use projection_transform::{
    crs::*, jprect::JPRZone, proj_wrapper::ProjTransform, vshift::Jgd2011ToWgs84,
};

use super::Transform;

pub struct ProjectionTransform {
    jgd2wgs: Arc<Jgd2011ToWgs84>,
    output_epsg: EpsgCode,
}

impl Transform for ProjectionTransform {
    fn transform(&self, point_cloud: PointCloud) -> PointCloud {
        let input_epsg = point_cloud.metadata.epsg;

        // Check if this is a Japan-specific coordinate system that can use optimized path
        let is_japan_cs = matches!(
            input_epsg,
            EPSG_JGD2011_JPRECT_I
                | EPSG_JGD2011_JPRECT_II
                | EPSG_JGD2011_JPRECT_III
                | EPSG_JGD2011_JPRECT_IV
                | EPSG_JGD2011_JPRECT_V
                | EPSG_JGD2011_JPRECT_VI
                | EPSG_JGD2011_JPRECT_VII
                | EPSG_JGD2011_JPRECT_VIII
                | EPSG_JGD2011_JPRECT_IX
                | EPSG_JGD2011_JPRECT_X
                | EPSG_JGD2011_JPRECT_XI
                | EPSG_JGD2011_JPRECT_XII
                | EPSG_JGD2011_JPRECT_XIII
                | EPSG_JGD2011_JPRECT_XIV
                | EPSG_JGD2011_JPRECT_XV
                | EPSG_JGD2011_JPRECT_XVI
                | EPSG_JGD2011_JPRECT_XVII
                | EPSG_JGD2011_JPRECT_XVIII
                | EPSG_JGD2011_JPRECT_XIX
                | EPSG_JGD2011_JPRECT_I_JGD2011_HEIGHT
                | EPSG_JGD2011_JPRECT_II_JGD2011_HEIGHT
                | EPSG_JGD2011_JPRECT_III_JGD2011_HEIGHT
                | EPSG_JGD2011_JPRECT_IV_JGD2011_HEIGHT
                | EPSG_JGD2011_JPRECT_V_JGD2011_HEIGHT
                | EPSG_JGD2011_JPRECT_VI_JGD2011_HEIGHT
                | EPSG_JGD2011_JPRECT_VII_JGD2011_HEIGHT
                | EPSG_JGD2011_JPRECT_VIII_JGD2011_HEIGHT
                | EPSG_JGD2011_JPRECT_IX_JGD2011_HEIGHT
                | EPSG_JGD2011_JPRECT_X_JGD2011_HEIGHT
                | EPSG_JGD2011_JPRECT_XI_JGD2011_HEIGHT
                | EPSG_JGD2011_JPRECT_XII_JGD2011_HEIGHT
                | EPSG_JGD2011_JPRECT_XIII_JGD2011_HEIGHT
        );

        if is_japan_cs && self.output_epsg == EPSG_WGS84_GEOGRAPHIC_3D {
            // Use optimized Japan-specific transformation path
            self.transform_from_jgd2011(point_cloud, Some(input_epsg))
        } else {
            // Use generic PROJ transformation for all other cases
            self.transform_generic(point_cloud)
        }
    }
}

impl ProjectionTransform {
    pub fn new(jgd2wgs: Arc<Jgd2011ToWgs84>, output_epsg: EpsgCode) -> Self {
        Self {
            jgd2wgs,
            output_epsg,
        }
    }

    fn rectangular_to_lnglat(x: f64, y: f64, height: f64, input_epsg: EpsgCode) -> (f64, f64, f64) {
        let zone = JPRZone::from_epsg(input_epsg).unwrap();
        let proj = zone.projection();
        let (lng, lat, height) = proj.project_inverse(x, y, height).unwrap();
        (lng, lat, height)
    }

    fn transform_generic(&self, point_cloud: PointCloud) -> PointCloud {
        let input_epsg = point_cloud.metadata.epsg;

        // Create PROJ transformation
        let proj_transform = ProjTransform::new(input_epsg, self.output_epsg)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to create PROJ transformation from EPSG:{} to EPSG:{}: {}",
                    input_epsg, self.output_epsg, e
                )
            });

        let mut points = vec![];
        for (x, y, z, point) in point_cloud.iter() {
            match proj_transform.transform(x, y, z) {
                Ok((new_x, new_y, new_z)) => {
                    points.push(Point {
                        x: new_x,
                        y: new_y,
                        z: new_z,
                        color: point.color.clone(),
                        attributes: point.attributes.clone(),
                    });
                }
                Err(e) => {
                    eprintln!("Warning: Failed to transform point ({}, {}, {}): {}", x, y, z, e);
                    // Skip points that fail to transform
                    continue;
                }
            }
        }

        PointCloud::new(points, self.output_epsg)
    }

    fn transform_from_jgd2011(
        &self,
        point_cloud: PointCloud,
        rectangular: Option<EpsgCode>,
    ) -> PointCloud {
        let mut points = vec![];
        match self.output_epsg {
            EPSG_WGS84_GEOGRAPHIC_3D => {
                for (x, y, z, point) in point_cloud.iter() {
                    let (lng, lat, height) = if let Some(input_epsg) = rectangular {
                        Self::rectangular_to_lnglat(x, y, z, input_epsg)
                    } else {
                        (x, y, z)
                    };

                    let (lng, lat, height) = self.jgd2wgs.convert(lng, lat, height);

                    points.push(Point {
                        x: lng,
                        y: lat,
                        z: height,
                        color: point.color.clone(),
                        attributes: point.attributes.clone(),
                    });
                }
            }
            _ => {
                panic!("Unsupported output CRS: {}", self.output_epsg);
            }
        };
        PointCloud::new(points, self.output_epsg)
    }
}
