/// PROJ-based coordinate transformations for global coordinate systems
///
/// This module provides a wrapper around the PROJ library to handle
/// coordinate transformations for all non-Japan coordinate systems.

use crate::crs::EpsgCode;
use crate::error::Error;
use proj::Proj;
use std::sync::Arc;

/// PROJ-based transformer for global coordinate systems
pub struct ProjTransform {
    proj: Proj,
    input_epsg: EpsgCode,
    output_epsg: EpsgCode,
}

impl ProjTransform {
    /// Create a new PROJ-based transformer
    ///
    /// # Arguments
    /// * `input_epsg` - Input coordinate system EPSG code
    /// * `output_epsg` - Output coordinate system EPSG code
    ///
    /// # Returns
    /// * `Ok(ProjTransform)` - Successfully created transformer
    /// * `Err(Error)` - Failed to create transformation (unsupported EPSG codes)
    pub fn new(input_epsg: EpsgCode, output_epsg: EpsgCode) -> Result<Self, Error> {
        // Create PROJ transformation string
        let proj_string = format!("EPSG:{} +to EPSG:{}", input_epsg, output_epsg);

        // Initialize PROJ transformation
        let proj = Proj::new(&proj_string).map_err(|e| {
            Error::UnsupportedTransformation {
                from: input_epsg,
                to: output_epsg,
                reason: format!("PROJ initialization failed: {}", e),
            }
        })?;

        Ok(Self {
            proj,
            input_epsg,
            output_epsg,
        })
    }

    /// Transform a single point (x, y, z)
    ///
    /// # Arguments
    /// * `x` - X coordinate (or longitude for geographic)
    /// * `y` - Y coordinate (or latitude for geographic)
    /// * `z` - Z coordinate (or height/elevation)
    ///
    /// # Returns
    /// * Transformed coordinates (x, y, z)
    pub fn transform(&self, x: f64, y: f64, z: f64) -> Result<(f64, f64, f64), Error> {
        // PROJ's convert method handles 2D coordinates (x, y)
        // Height/elevation is preserved for most transformations
        let (new_x, new_y) = self
            .proj
            .convert((x, y))
            .map_err(|e| Error::TransformationFailed {
                reason: format!("PROJ transformation failed: {}", e),
            })?;

        // Z coordinate (height/elevation) is typically preserved
        // For datum transformations, this should be handled separately if needed
        Ok((new_x, new_y, z))
    }

    /// Transform multiple points in batch
    ///
    /// # Arguments
    /// * `points` - Slice of points as (x, y, z) tuples
    ///
    /// # Returns
    /// * Vector of transformed points
    pub fn transform_batch(&self, points: &[(f64, f64, f64)]) -> Result<Vec<(f64, f64, f64)>, Error> {
        points
            .iter()
            .map(|&(x, y, z)| self.transform(x, y, z))
            .collect()
    }

    /// Get input EPSG code
    pub fn input_epsg(&self) -> EpsgCode {
        self.input_epsg
    }

    /// Get output EPSG code
    pub fn output_epsg(&self) -> EpsgCode {
        self.output_epsg
    }
}

/// Thread-safe PROJ transformer
///
/// This wrapper provides thread-safe access to PROJ transformations
/// by using Arc for shared ownership across threads.
#[derive(Clone)]
pub struct ProjTransformShared {
    inner: Arc<ProjTransform>,
}

impl ProjTransformShared {
    /// Create a new thread-safe PROJ transformer
    pub fn new(input_epsg: EpsgCode, output_epsg: EpsgCode) -> Result<Self, Error> {
        let transform = ProjTransform::new(input_epsg, output_epsg)?;
        Ok(Self {
            inner: Arc::new(transform),
        })
    }

    /// Transform a single point
    pub fn transform(&self, x: f64, y: f64, z: f64) -> Result<(f64, f64, f64), Error> {
        self.inner.transform(x, y, z)
    }

    /// Transform multiple points in batch
    pub fn transform_batch(&self, points: &[(f64, f64, f64)]) -> Result<Vec<(f64, f64, f64)>, Error> {
        self.inner.transform_batch(points)
    }

    /// Get input EPSG code
    pub fn input_epsg(&self) -> EpsgCode {
        self.inner.input_epsg()
    }

    /// Get output EPSG code
    pub fn output_epsg(&self) -> EpsgCode {
        self.inner.output_epsg()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crs::*;

    #[test]
    fn test_wgs84_to_utm() {
        // Test WGS84 geographic to UTM Zone 10N (California)
        let transform = ProjTransform::new(EPSG_WGS84_GEOGRAPHIC_3D, EPSG_WGS84_UTM_10N).unwrap();

        // San Francisco: -122.4194, 37.7749
        let (lon, lat, height) = (-122.4194, 37.7749, 100.0);
        let (x, y, z) = transform.transform(lon, lat, height).unwrap();

        // Expected UTM coordinates (approximate)
        assert!((x - 551_000.0).abs() < 1000.0, "X coordinate mismatch");
        assert!((y - 4_180_000.0).abs() < 1000.0, "Y coordinate mismatch");
        assert!((z - 100.0).abs() < 1.0, "Z coordinate mismatch");
    }

    #[test]
    fn test_utm_to_wgs84() {
        // Test UTM Zone 33N to WGS84 (Europe)
        let transform = ProjTransform::new(EPSG_WGS84_UTM_33N, EPSG_WGS84_GEOGRAPHIC_3D).unwrap();

        // Rome approximate UTM coordinates
        let (x, y, z) = (291_952.0, 4_640_623.0, 50.0);
        let (lon, lat, height) = transform.transform(x, y, z).unwrap();

        // Expected geographic coordinates (Rome: ~12.4964°E, 41.9028°N)
        assert!((lon - 12.4964).abs() < 0.01, "Longitude mismatch");
        assert!((lat - 41.9028).abs() < 0.01, "Latitude mismatch");
        assert!((height - 50.0).abs() < 1.0, "Height mismatch");
    }

    #[test]
    fn test_nad83_state_plane() {
        // Test NAD83 California Zone 1 to WGS84
        let transform = ProjTransform::new(EPSG_NAD83_CALIFORNIA_ZONE_1, EPSG_WGS84_GEOGRAPHIC_3D);

        // This test verifies the transformation can be created
        // Actual coordinates would depend on specific location
        assert!(transform.is_ok(), "Should create NAD83 State Plane transformer");
    }

    #[test]
    fn test_batch_transform() {
        let transform = ProjTransform::new(EPSG_WGS84_GEOGRAPHIC_3D, EPSG_WGS84_UTM_31N).unwrap();

        let points = vec![
            (10.0, 50.0, 100.0),  // Germany
            (11.0, 51.0, 150.0),
            (12.0, 52.0, 200.0),
        ];

        let results = transform.transform_batch(&points).unwrap();
        assert_eq!(results.len(), 3);

        // Verify all points were transformed
        for (result, &original) in results.iter().zip(points.iter()) {
            assert!(result.0 > 200_000.0 && result.0 < 800_000.0, "X in valid UTM range");
            assert!(result.1 > 5_000_000.0 && result.1 < 6_000_000.0, "Y in valid UTM range");
            assert!((result.2 - original.2).abs() < 1.0, "Height preserved");
        }
    }

    #[test]
    fn test_thread_safe_transform() {
        let transform = ProjTransformShared::new(EPSG_WGS84_GEOGRAPHIC_3D, EPSG_WGS84_UTM_10N).unwrap();

        // Clone for thread safety
        let transform_clone = transform.clone();

        let (x, y, z) = transform_clone.transform(-122.0, 37.0, 100.0).unwrap();
        assert!(x > 500_000.0 && x < 700_000.0);
    }

    #[test]
    fn test_invalid_epsg() {
        // Test with invalid EPSG codes
        let result = ProjTransform::new(99999, EPSG_WGS84_GEOGRAPHIC_3D);
        assert!(result.is_err(), "Should fail with invalid EPSG code");
    }

    #[test]
    fn test_european_systems() {
        // Test ETRS89 UTM 32N to WGS84
        let transform = ProjTransform::new(EPSG_ETRS89_UTM_32N, EPSG_WGS84_GEOGRAPHIC_3D);
        assert!(transform.is_ok(), "Should support ETRS89");

        // Test UK OSGB36 to WGS84
        let transform_uk = ProjTransform::new(EPSG_OSGB36_BRITISH_NATIONAL_GRID, EPSG_WGS84_GEOGRAPHIC_3D);
        assert!(transform_uk.is_ok(), "Should support OSGB36");
    }

    #[test]
    fn test_australian_systems() {
        // Test GDA2020 MGA Zone 55 to WGS84
        let transform = ProjTransform::new(EPSG_GDA2020_MGA_ZONE_55, EPSG_WGS84_GEOGRAPHIC_3D);
        assert!(transform.is_ok(), "Should support GDA2020");
    }
}
