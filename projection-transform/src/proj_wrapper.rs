//! Generic coordinate transformation using PROJ library
//!
//! This module provides support for transforming between any EPSG coordinate systems
//! using the PROJ library's comprehensive database of coordinate reference systems.

use crate::{crs::EpsgCode, error::TransformError};
use proj::Proj;

/// Generic coordinate transformation using PROJ library
pub struct ProjTransform {
    proj: Proj,
}

impl ProjTransform {
    /// Create a new PROJ transformation from one EPSG code to another
    pub fn new(from_epsg: EpsgCode, to_epsg: EpsgCode) -> Result<Self, TransformError> {
        let from_crs = format!("EPSG:{}", from_epsg);
        let to_crs = format!("EPSG:{}", to_epsg);

        let proj = Proj::new_known_crs(&from_crs, &to_crs, None)
            .ok_or_else(|| TransformError::UnsupportedCrs {
                from: from_epsg,
                to: to_epsg,
            })?;

        Ok(Self { proj })
    }

    /// Transform a point from source to target CRS
    pub fn transform(&self, x: f64, y: f64, z: f64) -> Result<(f64, f64, f64), TransformError> {
        // PROJ expects (x, y, z) or (longitude, latitude, height) depending on CRS
        let result = self
            .proj
            .convert((x, y, z))
            .map_err(|e| TransformError::ProjectionError(e.to_string()))?;

        Ok((result.0, result.1, result.2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wgs84_to_web_mercator() {
        // Transform from WGS84 (EPSG:4326) to Web Mercator (EPSG:3857)
        let transform = ProjTransform::new(4326, 3857).expect("Failed to create transformation");

        // Test point: (longitude, latitude) = (0, 0)
        let (x, y, z) = transform.transform(0.0, 0.0, 0.0).unwrap();

        // At (0, 0), Web Mercator should be (0, 0)
        assert!((x - 0.0).abs() < 1e-6);
        assert!((y - 0.0).abs() < 1e-6);
        assert!((z - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_jgd2011_to_wgs84() {
        // Transform from JGD2011 Geographic 3D (EPSG:6697) to WGS84 Geographic 3D (EPSG:4979)
        let transform = ProjTransform::new(6697, 4979).expect("Failed to create transformation");

        // Test point in Japan
        let (lng, lat, height) = transform.transform(139.0, 35.0, 100.0).unwrap();

        // The transformation should be close to identity for horizontal coordinates
        // (JGD2011 and WGS84 are very similar)
        assert!((lng - 139.0).abs() < 0.01);
        assert!((lat - 35.0).abs() < 0.01);
        // Height might differ due to geoid vs ellipsoid
    }

    #[test]
    fn test_utm_to_wgs84() {
        // Transform from UTM Zone 33N (EPSG:32633) to WGS84 (EPSG:4326)
        let transform = ProjTransform::new(32633, 4326).expect("Failed to create transformation");

        // Test point: UTM coordinates (500000, 0) - center of zone at equator
        let (lng, lat, _z) = transform.transform(500000.0, 0.0, 0.0).unwrap();

        // Should be near 15 degrees longitude (center of UTM zone 33)
        assert!((lng - 15.0).abs() < 0.1);
        assert!(lat.abs() < 0.1);
    }
}
